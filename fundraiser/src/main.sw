contract;

// TODO:
//      * matching on self is not implemented so cannot do equality on enums

dep abi;
dep data_structures;
dep errors;
dep events;
dep utils;

use std::{
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    constants::BASE_ASSET_ID,
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    result::*,
    revert::revert,
    storage::StorageMap,
    token::{force_transfer_to_contract, transfer_to_output}
};

use abi::Fundraiser;
use data_structures::{Campaign, CampaignInfo, Pledge};
use errors::{CampaignError, CreationError, UserError};
use events::{CancelledCampaignEvent, ClaimedEvent, CreatedCampaignEvent, PledgedEvent, UnpledgedEvent};
use utils::{sender_identity, transfer, validate_id, validate_sender};

storage {
    /// The total number of unique campaigns that a user has created
    /// This should only be incremented
    /// Cancelling / Claiming should not affect this number
    campaign_count: StorageMap<Identity,
    u64>, /// Campaigns that have been created by a user
    /// Map(Identity => Map(1...campaign_count => Campaign)
    campaign_history: StorageMap<(Identity,
    u64), Campaign>, /// Data describing the content of a campaign
    /// Map(Campaign ID => CampaignInfo)
    campaign_info: StorageMap<u64,
    CampaignInfo>, /// The total number of unique campaigns that a user has pledged to
    /// This should only be incremented.
    /// Unpledging should not affect this number
    pledge_count: StorageMap<Identity,
    u64>, /// Record of how much a user has pledged to a specific campaign
    /// Locked after the deadline
    /// Map(Identity => Map(1...pledge_count => Pledge))
    pledge_history: StorageMap<(Identity,
    u64), Pledge>, /// O(1) look-up to prevent iterating over pledge_history
    /// Map(Identity => Map(Campaign ID => Pledge History Index))
    pledge_history_index: StorageMap<(Identity,
    u64), u64>, /// The number of campaigns created by all users
    total_campaigns: u64,
}

impl Fundraiser for Contract {
    /// Creates a data structure representing a campaign that users can pledge to
    ///
    /// Instead of having a contract per campaign we create an internal representation for the data
    /// and manage it via mappings.
    ///
    /// # Arguments
    ///
    /// * `asset` - A coin that the campaign accepts as a pledge
    /// * `beneficiary` - The recipient to whom the pledge will be sent to upon a successful campaign
    /// * `deadline` - Block height used to dictate the end time of a campaign
    /// * `target_amount` - The amount of `asset` required to deem the campaign a success
    ///
    /// # Reverts
    ///
    /// * When `asset` is the BASE_ASSET
    /// * When the `deadline` is not ahead of the current block height
    /// * When the `target_amount` is 0
    /// * When an AuthError is generated
    #[storage(read, write)]fn create_campaign(asset: ContractId, beneficiary: Identity, deadline: u64, target_amount: u64) {
        require(asset.value != BASE_ASSET_ID, CreationError::CannotUseBaseAsset);
        require(height() < deadline, CreationError::DeadlineMustBeInTheFuture);
        require(0 < target_amount, CreationError::TargetAmountCannotBeZero);

        let user = sender_identity();

        let campaign_info = CampaignInfo {
            asset, author: user,
            beneficiary, cancelled: false,
            claimed: false,
            deadline, target_amount, total_pledge: 0,
        };

        let campaign_count = storage.campaign_count.get(user);

        storage.total_campaigns = storage.total_campaigns + 1;
        storage.campaign_count.insert(user, campaign_count + 1);
        storage.campaign_history.insert((user, campaign_count + 1), Campaign {
            id: storage.total_campaigns
        });
        storage.campaign_info.insert(storage.total_campaigns, campaign_info);

        log(CreatedCampaignEvent {
            campaign_info, id: storage.total_campaigns
        });
    }

    /// Marks a campaign as cancelled preventing further pledges or a claim to be made
    ///
    /// # Arguments
    ///
    /// * `id` - Unique campaign identifier which is a number from the storage.total_campaigns range
    ///
    /// # Reverts
    ///
    /// * When the `id` is either 0 or greater than the total number of campaigns created
    /// * When an AuthError is generated
    /// * When the user is not the author of the campaign
    /// * When the deadline has been surpassed
    /// * When the campaign has already been cancelled
    #[storage(read, write)]fn cancel_campaign(id: u64) {
        validate_id(id, storage.total_campaigns);

        let mut campaign_info = storage.campaign_info.get(id);

        // require(campaign_info.author == sender_identity(), UserError::UnauthorizedUser);
        validate_sender(sender_identity(), campaign_info.author); // workaround

        require(height() < campaign_info.deadline, CampaignError::CampaignEnded);
        require(!campaign_info.cancelled, CampaignError::CampaignHasBeenCancelled);

        campaign_info.cancelled = true;

        storage.campaign_info.insert(id, campaign_info);

        log(CancelledCampaignEvent {
            id
        });
    }

    /// Transfers the total pledge to the beneficiary
    ///
    /// # Arguments
    ///
    /// * `id` - Unique campaign identifier which is a number from the storage.total_campaigns range
    ///
    /// # Reverts
    ///
    /// * When the `id` is either 0 or greater than the total number of campaigns created
    /// * When an AuthError is generated
    /// * When the user is not the author of the campaign
    /// * When the deadline has not been reached
    /// * When the total pledge has not reached the minimum `target_amount`
    /// * When the campaign has already been claimed
    /// * When the campaign has already been cancelled
    #[storage(read, write)]fn claim_pledges(id: u64) {
        validate_id(id, storage.total_campaigns);

        let mut campaign_info = storage.campaign_info.get(id);

        // require(campaign_info.author == sender_identity(), UserError::UnauthorizedUser);
        validate_sender(sender_identity(), campaign_info.author); // workaround

        require(campaign_info.deadline <= height(), CampaignError::DeadlineNotReached);
        require(campaign_info.target_amount <= campaign_info.total_pledge, CampaignError::TargetNotReached);
        require(!campaign_info.claimed, UserError::AlreadyClaimed);
        require(!campaign_info.cancelled, CampaignError::CampaignHasBeenCancelled);

        campaign_info.claimed = true;
        storage.campaign_info.insert(id, campaign_info);

        transfer(campaign_info.beneficiary, campaign_info.total_pledge, campaign_info.asset);

        log(ClaimedEvent {
            id
        });
    }

    /// Allows a user to pledge any amount of the campaign asset towards the campaign goal
    ///
    /// In order to reach the campaign's target amount users must pledge some amount of asset towards
    /// that campaign.
    /// This information is recorded for the campaign and for the user so that they can unpledge.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique campaign identifier which is a number from the storage.total_campaigns range
    ///
    /// # Reverts
    ///
    /// * When the `id` is either 0 or greater than the total number of campaigns created
    /// * When the user attempts to pledge when the deadline has been reached
    /// * When the user pledges a different asset to the one specified in the campaign
    /// * When the user pledges after the campaign has been cancelled
    /// * When an AuthError is generated
    #[storage(read, write)]fn pledge(id: u64) {
        validate_id(id, storage.total_campaigns);

        let mut campaign_info = storage.campaign_info.get(id);

        require(height() < campaign_info.deadline, CampaignError::CampaignEnded);
        require(campaign_info.asset == msg_asset_id(), UserError::IncorrectAssetSent);
        require(!campaign_info.cancelled, CampaignError::CampaignHasBeenCancelled);

        let user = sender_identity();
        let pledge_count = storage.pledge_count.get(user);

        if pledge_count == 0 {
            storage.pledge_history.insert((user, pledge_count + 1), Pledge {
                amount: msg_amount(), id
            });
            storage.pledge_history_index.insert((user, id), pledge_count + 1);
            storage.pledge_count.insert(user, 1);
        } else {
            let pledge_history_index = storage.pledge_history_index.get((user, id));

            if pledge_history_index != 0 {
                let mut pledge = storage.pledge_history.get((user, pledge_history_index));
                pledge.amount = pledge.amount + msg_amount();

                storage.pledge_history.insert((user, pledge_history_index), pledge);
            } else {
                storage.pledge_history.insert((user, pledge_count + 1), Pledge {
                    amount: msg_amount(), id
                });
                storage.pledge_history_index.insert((user, id), pledge_count + 1);
                storage.pledge_count.insert(user, pledge_count + 1);
            }
        }

        campaign_info.total_pledge = campaign_info.total_pledge + msg_amount();

        storage.campaign_info.insert(id, campaign_info);

        log(PledgedEvent {
            amount: msg_amount(), id
        });
    }

    /// Allows a user to unpledge an amount of the campaign asset that they have pledged
    ///
    /// A user may have changed their mind about the amount of an asset that they have pledged
    /// therefore they may wish to unpledge some amount of that pledge.
    /// If they attempt to unpledge more than they have pledged then their total pledge will be returned
    ///
    /// # Arguments
    ///
    /// * `id` - Unique campaign identifier which is a number from the storage.total_campaigns range
    /// * `amount` - The amount of asset that the user wishes to unpledge
    ///
    /// # Reverts
    ///
    /// * When the `id` is either 0 or greater than the total number of campaigns created
    /// * When the user attempts to unpledge after the deadline and `target_amount` have been reached
    /// * When an AuthError is generated
    /// * When the user has not pledged to the campaign represented by the `id`
    #[storage(read, write)]fn unpledge(id: u64, amount: u64) {
        validate_id(id, storage.total_campaigns);

        let mut campaign_info = storage.campaign_info.get(id);

        if campaign_info.deadline <= height() {
            require(campaign_info.total_pledge < campaign_info.target_amount, CampaignError::TargetReached);
        }

        let user = sender_identity();
        let pledge_history_index = storage.pledge_history_index.get((user, id));

        require(pledge_history_index != 0, UserError::UserHasNotPledged);

        let mut pledge = storage.pledge_history.get((user, pledge_history_index));
        let mut amount = amount;

        if pledge.amount < amount {
            amount = pledge.amount;
        }

        pledge.amount = pledge.amount - amount;
        campaign_info.total_pledge = campaign_info.total_pledge - amount;

        storage.pledge_history.insert((user, pledge_history_index), pledge);
        storage.campaign_info.insert(id, campaign_info);

        transfer(user, amount, campaign_info.asset);

        log(UnpledgedEvent {
            amount, id
        });
    }

    /// Returns the total number of campaigns that have been created by all users
    #[storage(read)]fn total_campaigns() -> u64 {
        storage.total_campaigns
    }

    /// Returns information about the specified campaign
    ///
    /// # Arguments
    ///
    /// * `id` - Unique campaign identifier which is a number from the storage.total_campaigns range
    ///
    /// # Reverts
    ///
    /// * When the `id` is either 0 or greater than the total number of campaigns created
    #[storage(read)]fn campaign_info(id: u64) -> CampaignInfo {
        validate_id(id, storage.total_campaigns);
        storage.campaign_info.get(id)
    }

    /// Returns the number of campaigns that the user has created
    ///
    /// # Reverts
    ///
    /// * When an AuthError is generated
    #[storage(read)]fn campaign_count() -> u64 {
        storage.campaign_count.get(sender_identity())
    }

    /// Returns information about the specified campaign for the campaign author
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier which is a number starting from 1...storage.campaign_count
    ///
    /// # Reverts
    ///
    /// * When the `id` is either 0 or greater than the total number of campaigns created by the author
    /// * When an AuthError is generated
    #[storage(read)]fn campaign(id: u64) -> Campaign {
        require(id != 0 && id <= storage.campaign_count.get(sender_identity()), UserError::InvalidHistoryId);
        storage.campaign_history.get((sender_identity(), id))
    }

    /// Returns the number of campaigns that the user has pledged to
    ///
    /// # Reverts
    ///
    /// * When an AuthError is generated
    #[storage(read)]fn pledge_count() -> u64 {
        storage.pledge_count.get(sender_identity())
    }

    /// Returns information about the specified pledge for the user
    ///
    /// # Arguments
    ///
    /// * `pledge_history_index` - Unique identifier which is a number starting from 1...storage.pledge_count
    ///
    /// # Reverts
    ///
    /// * When the `pledge_history_index` is either 0 or greater than the total number of pledges made by the user
    /// * When an AuthError is generated
    #[storage(read)]fn pledged(pledge_history_index: u64) -> Pledge {
        require(pledge_history_index != 0 && pledge_history_index < storage.pledge_count.get(sender_identity()), UserError::InvalidHistoryId);
        storage.pledge_history.get((sender_identity(), pledge_history_index))
    }
}
