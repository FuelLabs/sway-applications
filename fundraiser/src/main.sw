contract;

// TODO: matching on self is not implemented so cannot do equalityy on enums
//       change arrays to vec when out
//       currently only the author stores campaign info, need to show user as well

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
use data_structures::{Campaign, State, UserCampaigns};
use errors::{CreationError, UserError};
use events::{CancelledCampaignEvent, ClaimedEvent, CreatedCampaignEvent, PledgedEvent, UnpledgedEvent};
use utils::{sender_identity, transfer, validate_id, validate_sender};

storage {
    /// campaign identifier => Campaign Data
    campaigns: StorageMap<u64,
    Campaign>, campaign_count: u64,

    /// campaign identifier => user => amount pledged
    pledgers: StorageMap<(u64,
    Identity), u64>, user_campaigns: StorageMap<Identity,
    UserCampaigns>, 
}

impl Fundraiser for Contract {
    fn create_campaign(asset: ContractId, beneficiary: Identity, deadline: u64, target_amount: u64) {
        require(height() < deadline, CreationError::DeadlineMustBeInTheFuture);
        require(~ContractId::from(NATIVE_ASSET_ID) != asset, CreationError::CannotUseNativeAsset);
        require(0 < target_amount, CreationError::TargetAmountCannotBeZero);

        let campaign = Campaign {
            asset, author: sender_identity(),
            beneficiary, claimed: false,
            deadline, // state: State::Funding
            state: 0,
            target_amount, total_pledge: 0,
        };

        storage.campaign_count = storage.campaign_count + 1;
        storage.campaigns.insert(storage.campaign_count, campaign);

        // TODO: vec (user_campaigns.active.push(storage.campaign_count))
        let mut user_campaigns = storage.user_campaigns.get(campaign.author);
        user_campaigns.active = [storage.campaign_count];

        log(CreatedCampaignEvent {
            campaign, id: storage.campaign_count
        });
    }

    fn pledge(id: u64) {
        validate_id(id, storage.campaign_count);

        let mut campaign = storage.campaigns.get(id);

        // require(campaign.state == State::Funding, UserError::FundraiseEnded);
        require(campaign.state == 0, UserError::FundraiseEnded); // workaround
        require(height() < campaign.deadline, UserError::FundraiseEnded);
        require(campaign.asset == msg_asset_id(), UserError::IncorrectAssetSent);

        let pledge_amount = msg_amount();
        let user = sender_identity();
        let user_pledge = storage.pledgers.get((id, user)) + pledge_amount;

        campaign.total_pledge = campaign.total_pledge + pledge_amount;
        storage.campaigns.insert(id, campaign);
        storage.pledgers.insert((id, user), user_pledge);

        log(PledgedEvent {
            amount: pledge_amount, id
        });
    }

    fn unpledge(id: u64, amount: u64) {
        validate_id(id, storage.campaign_count);

        let mut campaign = storage.campaigns.get(id);

        // require(campaign.state != State::Successful, UserError::FundraiseEnded);
        require(campaign.state != 1, UserError::FundraiseEnded); // workaround

        let user = sender_identity();
        let user_pledge = storage.pledgers.get((id, user));

        // workaround
        let mut amount = amount;

        if user_pledge < amount {
            amount = user_pledge;
        }

        campaign.total_pledge = campaign.total_pledge - amount;
        storage.campaigns.insert(id, campaign);
        storage.pledgers.insert((id, user), user_pledge - amount);

        transfer(user, amount, campaign.asset);

        log(UnpledgedEvent {
            amount, id
        });
    }

    fn claim(id: u64) {
        validate_id(id, storage.campaign_count);

        let mut campaign = storage.campaigns.get(id);

        // require(campaign.author == sender_identity(), UserError::UnauthorizedUser);
        validate_sender(sender_identity(), campaign.author); // workaround

        // require(campaign.state == State::Successful, UserError::FundraiseNotSuccessful);
        require(campaign.state == 1, UserError::FundraiseNotSuccessful); // workaround
        require(!campaign.claimed, UserError::AlreadyClaimed);

        let mut user_campaigns = storage.user_campaigns.get(campaign.author);

        // workaround
        user_campaigns.completed = user_campaigns.active;
        user_campaigns.active = [0];

        campaign.claimed = true;

        storage.campaigns.insert(id, campaign);
        storage.user_campaigns.insert(campaign.author, user_campaigns);

        transfer(campaign.beneficiary, campaign.total_pledge, campaign.asset);

        log(ClaimedEvent {
            id
        });
    }

    fn cancel(id: u64) {
        validate_id(id, storage.campaign_count);

        let mut campaign = storage.campaigns.get(id);

        // require(campaign.author == sender_identity(), UserError::UnauthorizedUser);
        validate_sender(sender_identity(), campaign.author); // workaround

        // require(campaign.state == State::Funding, UserError::FundraiseEnded);
        require(campaign.state == 0, UserError::FundraiseEnded); // workaround
        require(height() < campaign.deadline, UserError::FundraiseEnded);

        let mut user_campaigns = storage.user_campaigns.get(campaign.author);

        // workaround
        user_campaigns.completed = user_campaigns.active;
        user_campaigns.active = [0];

        // campaign.state = State::Cancelled;
        campaign.state = 3; // workaround

        storage.campaigns.insert(id, campaign);
        storage.user_campaigns.insert(campaign.author, user_campaigns);

        log(CancelledCampaignEvent {
            id
        });
    }

    fn campaign_info(id: u64) -> Campaign {
        validate_id(id, storage.campaign_count);
        storage.campaigns.get(id)
    }

    fn pledged(id: u64) -> u64 {
        validate_id(id, storage.campaign_count);
        storage.pledgers.get((id, sender_identity()))
    }

    /// Returns data regarding the identifiers for active and completed campaigns
    /// Works for all users - including ones not in the contract
    fn user_campaigns(user: Identity) -> UserCampaigns {
        storage.user_campaigns.get(user)
    }

    fn campaign_count() -> u64 {
        storage.campaign_count
    }

    fn update_campaign_state(id: u64) {
        validate_id(id, storage.campaign_count);

        let mut campaign = storage.campaigns.get(id);

        if campaign.deadline < height() {
            // if campaign.state == State::Funding {
            //     campaign.state = if campaign.target_amount < campaign.total_pledge { State::Failed } else { State::Successful };
            //     storage.campaigns.insert(id, campaign);
            // }
            // workaround
            if campaign.state == 0 {
                campaign.state = if campaign.target_amount < campaign.total_pledge {
                    2
                } else {
                    1
                };
                storage.campaigns.insert(id, campaign);
            }
        }
    }
}
