contract;

// TODO: matching on self is not implemented so cannot do equalityy on enums
//       change campaigns to use mappings like the user pledge history

dep abi;
dep data_structures;
dep errors;
dep events;
dep utils;

use std::{
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    constants::NATIVE_ASSET_ID,
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
use data_structures::{Campaigns, Campaign, State, Pledge};
use errors::{CreationError, UserError};
use events::{CancelledCampaignEvent, ClaimedEvent, CreatedCampaignEvent, PledgedEvent, UnpledgedEvent};
use utils::{sender_identity, transfer, validate_id, validate_sender};

storage {
    /// Data describing the content of a campaign
    /// Map(Campaign ID => Campaign)
    campaign: StorageMap<u64, Campaign>,

    /// Campaigns that have been created by a user
    /// Map(User => Campaigns)
    campaigns: StorageMap<Identity, Campaigns>, 

    /// The number of campaigns created by all users
    campaign_count: u64,
    
    /// The total number of unique campaigns that a user has pledged to
    /// This should only be incremented. 
    /// Unpledging should not affect this number
    pledge_count: StorageMap<Identity, u64>, 

    /// Record of how much a user has pledged to a specific campaign
    /// Locked after the deadline
    /// Map(Identity => Map(1...pledge_count => Pledge))
    pledge_history: StorageMap<(Identity, u64), Pledge>,

    /// O(1) look-up to prevent iterating over pledge_history
    /// Map(Identity => Map(Campaign ID => Pledge History Index))
    pledge_history_index: StorageMap<(Identity, u64), u64>,
}

impl Fundraiser for Contract {
    fn create_campaign(asset: ContractId, beneficiary: Identity, deadline: u64, target_amount: u64) {
        require(height() < deadline, CreationError::DeadlineMustBeInTheFuture);
        require(NATIVE_ASSET_ID != asset.value, CreationError::CannotUseNativeAsset);
        require(0 < target_amount, CreationError::TargetAmountCannotBeZero);

        let campaign = Campaign {
            asset, 
            author: sender_identity(),
            beneficiary, 
            claimed: false,
            deadline, 
            // state: State::Funding
            state: 0,
            target_amount, 
            total_pledge: 0,
        };

        storage.campaign_count = storage.campaign_count + 1;
        storage.campaign.insert(storage.campaign_count, campaign);

        // TODO: change to mappings
        let mut campaigns = storage.campaigns.get(campaign.author);
        campaigns.active = [storage.campaign_count];

        log(CreatedCampaignEvent {campaign, id: storage.campaign_count});
    }

    fn pledge(id: u64) {
        validate_id(id, storage.campaign_count);

        let mut campaign = storage.campaign.get(id);

        // require(campaign.state == State::Funding, UserError::FundraiseEnded);
        require(campaign.state == 0, UserError::FundraiseEnded); // workaround
        require(height() < campaign.deadline, UserError::FundraiseEnded);
        require(campaign.asset == msg_asset_id(), UserError::IncorrectAssetSent);

        let user = sender_identity();
        let pledge_count = storage.pledge_count.get(user);

        if pledge_count == 0 {
            storage.pledge_history.insert((user, pledge_count + 1), Pledge {amount: msg_amount(), id});
            storage.pledge_history_index.insert((user, id), pledge_count + 1);
            storage.pledge_count.insert(user, 1);
        } else {
            let pledge_history_index = storage.pledge_history_index.get((user, id));

            if pledge_history_index != 0 {
                let mut pledge = storage.pledge_history.get((user, pledge_history_index));
                pledge.amount = pledge.amount + msg_amount();

                storage.pledge_history.insert((user, pledge_history_index), pledge);
            } else {
                storage.pledge_history.insert((user, pledge_count + 1), Pledge {amount: msg_amount(), id});
                storage.pledge_history_index.insert((user, id), pledge_count + 1);
                storage.pledge_count.insert(user, pledge_count + 1);
            }
        }

        campaign.total_pledge = campaign.total_pledge + msg_amount();

        storage.campaign.insert(id, campaign);

        log(PledgedEvent {amount: msg_amount(), id});
    }

    fn unpledge(id: u64, amount: u64) {
        validate_id(id, storage.campaign_count);

        let mut campaign = storage.campaign.get(id);

        // require(campaign.state != State::Successful, UserError::FundraiseEnded);
        require(campaign.state != 1, UserError::FundraiseEnded); // workaround

        let user = sender_identity();
        let pledge_count = storage.pledge_count.get(user);

        if pledge_count != 0 {
            let pledge_history_index = storage.pledge_history_index.get((user, id));

            if pledge_history_index != 0 {
                let mut pledge = storage.pledge_history.get((user, pledge_history_index));
                let mut amount = amount;

                if pledge.amount < amount {
                    amount = pledge.amount;
                }

                pledge.amount = pledge.amount - amount;
                campaign.total_pledge = campaign.total_pledge - amount;

                storage.pledge_history.insert((user, pledge_history_index), pledge);
                storage.campaign.insert(id, campaign);

                transfer(user, amount, campaign.asset);

                log(UnpledgedEvent {amount, id});
            }
        }
    }

    fn claim(id: u64) {
        validate_id(id, storage.campaign_count);

        let mut campaign = storage.campaign.get(id);

        // require(campaign.author == sender_identity(), UserError::UnauthorizedUser);
        validate_sender(sender_identity(), campaign.author); // workaround

        // require(campaign.state == State::Successful, UserError::FundraiseNotSuccessful);
        require(campaign.state == 1, UserError::FundraiseNotSuccessful); // workaround
        require(!campaign.claimed, UserError::AlreadyClaimed);

        let mut campaigns = storage.campaigns.get(campaign.author);

        // workaround
        campaigns.completed = campaigns.active;
        campaigns.active = [0];

        campaign.claimed = true;

        storage.campaign.insert(id, campaign);
        storage.campaigns.insert(campaign.author, campaigns);

        transfer(campaign.beneficiary, campaign.total_pledge, campaign.asset);

        log(ClaimedEvent { id });
    }

    fn cancel(id: u64) {
        validate_id(id, storage.campaign_count);

        let mut campaign = storage.campaign.get(id);

        // require(campaign.author == sender_identity(), UserError::UnauthorizedUser);
        validate_sender(sender_identity(), campaign.author); // workaround

        // require(campaign.state == State::Funding, UserError::FundraiseEnded);
        require(campaign.state == 0, UserError::FundraiseEnded); // workaround
        require(height() < campaign.deadline, UserError::FundraiseEnded);

        let mut campaigns = storage.campaigns.get(campaign.author);

        // workaround
        campaigns.completed = campaigns.active;
        campaigns.active = [0];

        // campaign.state = State::Cancelled;
        campaign.state = 3; // workaround

        storage.campaign.insert(id, campaign);
        storage.campaigns.insert(campaign.author, campaigns);

        log(CancelledCampaignEvent {
            id
        });
    }

    fn campaign_count() -> u64 {
        storage.campaign_count
    }

    fn campaign_info(id: u64) -> Campaign {
        validate_id(id, storage.campaign_count);
        storage.campaign.get(id)
    }

    fn pledge_count() -> u64 {
        storage.pledge_count.get(sender_identity())
    }

    fn pledged(id: u64) -> Pledge {
        require(id < storage.pledge_count.get(sender_identity()), "TODO: make this an enum");
        storage.pledge_history.get((sender_identity(), id))
    }

    /// Returns data regarding the identifiers for active and completed campaign
    /// Works for all users - including ones not in the contract
    fn campaigns() -> Campaigns {
        storage.campaigns.get(sender_identity())
    }

    fn update_campaign_state(id: u64) {
        validate_id(id, storage.campaign_count);

        let mut campaign = storage.campaign.get(id);

        if campaign.deadline < height() {
            // if campaign.state == State::Funding {
            //     campaign.state = if campaign.target_amount < campaign.total_pledge { State::Failed } else { State::Successful };
            //     storage.campaign.insert(id, campaign);
            // }
            // workaround
            if campaign.state == 0 {
                campaign.state = if campaign.target_amount < campaign.total_pledge {
                    2
                } else {
                    1
                };
                storage.campaign.insert(id, campaign);
            }
        }
    }
}
