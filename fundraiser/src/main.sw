contract;

dep abi;
dep data_structures;
dep errors;
dep events;
dep utils;

// Identity and result importing via * is a workaround until bug is fixed
use std::{
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    constants::BASE_ASSET_ID,
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
    identity::*,
    logging::log,
    result::*,
    revert::revert,
    storage::StorageMap,
    token::transfer,
};

use abi::Fundraiser;
use data_structures::{AssetInfo, Campaign, CampaignInfo, Pledge};
use errors::{CampaignError, CreationError, UserError};
use events::{CancelledCampaignEvent, ClaimedEvent, CreatedCampaignEvent, PledgedEvent, UnpledgedEvent};
use utils::{sender_identity, validate_id};

storage {
    /// Total number of unique assets used across all campaigns
    asset_count: u64,

    /// Direct look-up for asset data if the user wants to check via a known ID
    asset_info: StorageMap<ContractId,
    AssetInfo>, /// O(1) look-up to allow searching via asset_count
    /// Map(1...asset_count => asset)
    asset_index: StorageMap<u64,
    ContractId>, /// The total number of unique campaigns that a user has created
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
        // Prevent a user from ever having the ability to accept the base asset in case that becomes
        // a contract - which may be used as a burned address similar to ETH 0x0000...
        require(asset.value != BASE_ASSET_ID, CreationError::CannotUseBaseAsset);

        // Users cannot interact with a campaign that has already ended (is in the past)
        require(height() < deadline, CreationError::DeadlineMustBeInTheFuture);

        // A campaign must have a target to reach and therefore 0 is an invalid amount
        require(0 < target_amount, CreationError::TargetAmountCannotBeZero);

        let user = sender_identity();

        // Create an internal representation of a campaign
        let campaign_info = CampaignInfo {
            asset, author: user,
            beneficiary, cancelled: false,
            claimed: false,
            deadline, target_amount, total_pledge: 0,
        };

        // Keep track of new assets
        let mut asset_info = storage.asset_info.get(asset);
        if !asset_info.exists {
            // New asset so mark it as existing
            asset_info.exists = true;

            // Update storage for new asset
            storage.asset_info.insert(asset, asset_info);

            // Increment asset count to keep track of new total
            storage.asset_count = storage.asset_count + 1;

            // Store in index to allow for asset discovery via iteration over numbers
            storage.asset_index.insert(storage.asset_count, asset);
        }

        // Use the user's number of created campaigns as an ID / way to index this new campaign
        let campaign_count = storage.campaign_count.get(user);

        // We've just created a new campaign so increment the number of created campaigns across all
        // users and store the new campaign
        storage.total_campaigns = storage.total_campaigns + 1;
        storage.campaign_info.insert(storage.total_campaigns, campaign_info);

        // Increment the number of campaigns this user has created and track the ID for the campaign
        // they have just created so that data can be easily retrieved without duplicating data
        storage.campaign_count.insert(user, campaign_count + 1);
        storage.campaign_history.insert((user, campaign_count + 1), Campaign {
            id: storage.total_campaigns
        });

        // We have changed the state by adding a new data structure therefore we log it
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
        // User cannot interact with a non-existent campaign
        validate_id(id, storage.total_campaigns);

        // Retrieve the campaign in order to check its data / update it
        let mut campaign_info = storage.campaign_info.get(id);

        // Only the creator (author) of the campaign can cancel it
        require(campaign_info.author == sender_identity(), UserError::UnauthorizedUser);

        // The campaign can only be cancelled before it has reached its deadline (ended)
        require(height() < campaign_info.deadline, CampaignError::CampaignEnded);

        // User cannot a campaign that has already been cancelled
        // Given the logic below this is unnecessary aside from ignoring event spam
        require(!campaign_info.cancelled, CampaignError::CampaignHasBeenCancelled);

        // Mark the campaign as cancelled
        campaign_info.cancelled = true;

        // Overwrite the previous campaign (which has not been cancelled) with the updated version
        storage.campaign_info.insert(id, campaign_info);

        // We have updated the state of a campaign therefore we must log it
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
        // User cannot interact with a non-existent campaign
        validate_id(id, storage.total_campaigns);

        // Retrieve the campaign in order to check its data / update it
        let mut campaign_info = storage.campaign_info.get(id);

        // Only the creator (author) of the campaign can initiate the claiming process
        require(campaign_info.author == sender_identity(), UserError::UnauthorizedUser);

        // The author should only have the ability to claim after the deadline has been reached
        // (campaign has naturally ended i.e. has not been cancelled)
        require(campaign_info.deadline <= height(), CampaignError::DeadlineNotReached);

        // The author can only claim the pledges once the target amount has been reached otherwise
        // users should be able to withdraw
        require(campaign_info.target_amount <= campaign_info.total_pledge, CampaignError::TargetNotReached);

        // The author can only claim once to prevent the entire contract from being drained
        require(!campaign_info.claimed, UserError::AlreadyClaimed);

        // The author cannot claim after they have cancelled the campaign regardless of any other
        // checks
        require(!campaign_info.cancelled, CampaignError::CampaignHasBeenCancelled);

        // Mark the campaign as claimed and overwrite the previous state with the updated version
        campaign_info.claimed = true;
        storage.campaign_info.insert(id, campaign_info);

        // Transfer the total pledged to this campaign to the beneficiary
        transfer(campaign_info.total_pledge, campaign_info.asset, campaign_info.beneficiary);

        // We have updated the state of a campaign therefore we must log it
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
        // User cannot interact with a non-existent campaign
        validate_id(id, storage.total_campaigns);

        // Retrieve the campaign in order to check its data / update it
        let mut campaign_info = storage.campaign_info.get(id);

        // The users should only have the ability to pledge to campaigns that have not reached their
        // deadline (ended naturally - not been cancelled)
        require(height() < campaign_info.deadline, CampaignError::CampaignEnded);

        // The campaign specifies an asset that it accepts therefore the user must pledge the correct
        // asset in order to update the state of the campaign
        require(campaign_info.asset == msg_asset_id(), UserError::IncorrectAssetSent);

        // A user cannot pledge zero since it does not make sense to do so
        require(0 < msg_amount(), UserError::AmountCannotBeZero);

        // The user should not be able to continue to pledge if the campaign has been cancelled
        // Given the logic below it's unnecessary but it makes sense to stop them
        require(!campaign_info.cancelled, CampaignError::CampaignHasBeenCancelled);

        // Use the user's pledges as an ID / way to index this new pledge
        let user = sender_identity();
        let pledge_count = storage.pledge_count.get(user);

        // Fetch the index to see if the user has pledged to this campaign before or if this is a
        // pledge to a new campaign
        let pledge_history_index = storage.pledge_history_index.get((user, id));

        // Pledging to a campaign that they have already pledged to
        if pledge_history_index != 0 {
            // 0 is the sentinel therefore they have pledged to this ID (campaign)
            // increment their previous amount with the current pledge and update their pledge
            let mut pledge = storage.pledge_history.get((user, pledge_history_index));
            pledge.amount = pledge.amount + msg_amount();

            storage.pledge_history.insert((user, pledge_history_index), pledge);
        }
        // Pledging to a new campaign
        else {
            // First time pledge to this campaign therefore increment everything by 1
            storage.pledge_count.insert(user, pledge_count + 1);

            // Store the data structure required to look up the campaign they have pledged to, also
            // track how much they have pledged so that they can withdraw the correct amount.
            // Moreover, this can be used to show the user how much they have pledged to any campaign
            storage.pledge_history.insert((user, pledge_count + 1), Pledge {
                amount: msg_amount(), id
            });

            // Since we use the campaign ID to interact with the contract use the ID as a key for
            // a reverse look-up. Value is the 1st pledge (count)
            storage.pledge_history_index.insert((user, id), pledge_count + 1);
        }

        // The user has pledged therefore we increment the total amount that this campaign has
        // received.
        campaign_info.total_pledge = campaign_info.total_pledge + msg_amount();

        // Campaign state has been updated therefore overwrite the previous version with the new
        storage.campaign_info.insert(id, campaign_info);

        // Update the asset amount to track the addition of the new pledge
        let mut asset_info = storage.asset_info.get(campaign_info.asset);
        asset_info.amount = asset_info.amount + msg_amount();

        // Update asset state
        storage.asset_info.insert(campaign_info.asset, asset_info);

        // We have updated the state of a campaign therefore we must log it
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
        // User cannot interact with a non-existent campaign
        validate_id(id, storage.total_campaigns);

        // Prevent a user from unpledging 0 since it does not make sense to do so
        require(amount != 0, UserError::AmountCannotBeZero);

        // Retrieve the campaign in order to check its data / update it
        let mut campaign_info = storage.campaign_info.get(id);

        // A user should be able to unpledge at any point except if the deadline has been reached
        // and the author has claimed
        if campaign_info.deadline <= height() {
            require(!campaign_info.claimed, UserError::AlreadyClaimed);
        }

        // Check if the user has pledged to the campaign they are attempting to unpledge from
        let user = sender_identity();
        let pledge_history_index = storage.pledge_history_index.get((user, id));

        require(pledge_history_index != 0, UserError::UserHasNotPledged);

        // User has pledged therefore retrieve the total that they have pledged
        let mut pledge = storage.pledge_history.get((user, pledge_history_index));
        let mut amount = amount; // workaround until `mut` is able to be set as a param

        // If the user is attempting to unpledge more than they have pledged then reset the amount
        // they are withdrawing to the maximum that they have pledged to this campaign
        if pledge.amount < amount {
            amount = pledge.amount;
        }

        // Update the amount that they have pledged
        pledge.amount = pledge.amount - amount;

        // Lower the campaign total pledge by the amount the user has unpledged
        campaign_info.total_pledge = campaign_info.total_pledge - amount;

        // Update the state of their pledge with the new version
        storage.pledge_history.insert((user, pledge_history_index), pledge);

        // Update the campaign state with the updated version as well
        storage.campaign_info.insert(id, campaign_info);

        // Update the asset amount to track the removal of the amount
        let mut asset_info = storage.asset_info.get(campaign_info.asset);
        asset_info.amount = asset_info.amount - amount;

        // Update asset state
        storage.asset_info.insert(campaign_info.asset, asset_info);

        // Transfer back the amount the user has unpledged
        transfer(amount, campaign_info.asset, user);

        // We have updated the state of a campaign therefore we must log it
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
        // User cannot interact with a non-existent campaign
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
        // Validate the ID to ensure that the user has created the campaign
        require(id != 0 && id <= storage.campaign_count.get(sender_identity()), UserError::InvalidID);
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
        // Validate the ID to ensure that the user has pledged
        require(pledge_history_index != 0 && pledge_history_index <= storage.pledge_count.get(sender_identity()), UserError::InvalidID);
        storage.pledge_history.get((sender_identity(), pledge_history_index))
    }

    /// Returns the number of unique assets that have added across all campaigns
    #[storage(read)]fn asset_count() -> u64 {
        storage.asset_count
    }

    /// Returns information about the specificed asset, specifically if it has been added and the
    /// pledged amount
    ///
    /// # Arguments
    ///
    /// * `asset` - Uniquie identifier that identifies the asset
    #[storage(read)]fn asset_info_by_address(asset: ContractId) -> AssetInfo {
        storage.asset_info.get(asset)
    }

    /// Returns information about the specificed asset, specifically if it has been added and the
    /// pledged amount
    ///
    /// The user interface will not know all possible assets that the contract contains therefore
    /// this helper method allows the interface to iterate over the asset_count to discover all assets
    ///
    /// # Arguments
    ///
    /// * `index` - Number from 1...asset_count
    #[storage(read)]fn asset_info_by_count(index: u64) -> AssetInfo {
        storage.asset_info.get(storage.asset_index.get(index))
    }
}
