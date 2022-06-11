contract;

// TODO: enums not supported in storage yet so it won't compile
//       matching on self is not implemented so cannot do equalityy on enums
//       better deadline handling instead of height()?

dep abi;
dep data_structures;
dep errors;
dep events;

use std::{
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
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
use data_structures::{Campaign, State};
use errors::{Error, StateError, UserError};
use events::{CancelledEvent, ClaimedEvent, CreatedCampaign, PledgedEvent, UnpledgedEvent};

storage {
    /// campaign identifier => Campaign Data
    campaigns: StorageMap<u64,
    Campaign>, campaign_count: u64,

    initialized: bool,

    owner: Identity,

    /// campaign identifier => user => amount pledged
    pledgers: StorageMap<(u64,
    Identity), u64>, 
}

impl Fundraiser for Contract {
    fn constructor(owner: Identity) {
        require(!storage.initialized, StateError::CannotReinitialize);
        storage.owner = owner;
        storage.initialized = true;
    }

    fn create_campaign(author: Identity, asset: ContractId, target_amount: u64, deadline: u64) {
        require(storage.initialized, UserError::UnauthorizedUser);
        let user = sender_identity();
        // require(storage.owner == user, UserError::UnauthorizedUser);
        // workaround
        validate_sender(user, storage.owner);

        storage.campaign_count = storage.campaign_count + 1;
        let campaign = Campaign {
            author, // TODO: require non-zero author (0x000...)?
            asset, // TODO: require non-zero asset (0x000...)?
            target_amount, deadline, // TODO: require time (block height) in the future?
            claimed: false,
            total_pledge: 0,
            // state: State::Funding
            state: 0,
        };
        storage.campaigns.insert(storage.campaign_count, campaign);

        log(CreatedCampaign {
            campaign, id: storage.campaign_count
        });
    }

    fn pledge(id: u64) {
        validate_id(id);

        let mut campaign = storage.campaigns.get(id);
        validate_deadline(campaign, id);

        // require(campaign.state == State::Funding, StateError::FundraiseEnded);
        require(campaign.state == 0, StateError::FundraiseEnded);
        require(campaign.asset == msg_asset_id(), UserError::IncorrectAssetSent);

        let pledge_amount = msg_amount();
        let user = sender_identity();
        let user_pledge = storage.pledgers.get((id, user)) + pledge_amount;

        campaign.total_pledge = campaign.total_pledge + pledge_amount;

        storage.campaigns.insert(id, campaign);
        storage.pledgers.insert((id, user), user_pledge);

        log(PledgedEvent {
            amount: pledge_amount, id, user
        });
    }

    fn unpledge(id: u64, amount: u64) {
        validate_id(id);

        let mut campaign = storage.campaigns.get(id);
        validate_deadline(campaign, id);

        // require(campaign.state != State::Successful, StateError::CannotUnpledgeSuccessfulFundraise);
        require(campaign.state != 1, StateError::CannotUnpledgeSuccessfulFundraise);

        let user = sender_identity();
        let user_pledge = storage.pledgers.get((id, user));

        // Not supported in signature
        let mut amount = amount;
        if user_pledge < amount {
            amount = user_pledge;
        }

        storage.pledgers.insert((id, user), user_pledge - amount);
        campaign.total_pledge = campaign.total_pledge - amount;

        transfer(user, amount, campaign.asset);

        log(UnpledgedEvent {
            amount, id, user
        });
    }

    fn claim(id: u64) {
        validate_id(id);

        let mut campaign = storage.campaigns.get(id);
        validate_deadline(campaign, id);

        require(!campaign.claimed, UserError::AlreadyClaimed);

        // require(campaign.state == State::Successful, StateError::FundraiseNotSuccessful);
        require(campaign.state == 1, StateError::FundraiseNotSuccessful);

        let user = sender_identity();
        // require(campaign.author == user, UserError::UnauthorizedUser);
        // workaround
        validate_sender(user, campaign.author);

        campaign.claimed = true;
        transfer(user, campaign.total_pledge, campaign.asset);

        log(ClaimedEvent {
            amount: campaign.total_pledge, id, user
        });
    }

    fn cancel(id: u64) {
        validate_id(id);

        let mut campaign = storage.campaigns.get(id);
        // require(campaign.state == State::Funding, StateError::FundraiseEnded);
        require(campaign.state == 0, StateError::FundraiseEnded);

        let user = sender_identity();
        // require(campaign.author == sender, UserError::UnauthorizedUser);
        // workaround
        validate_sender(user, campaign.author);

        // campaign.state = State::Cancelled;
        campaign.state = 3;
        storage.campaigns.insert(id, campaign);

        log(CancelledEvent {
            id, user
        });
    }

    fn get_campaign(id: u64) -> Campaign {
        validate_id(id);
        storage.campaigns.get(id)
    }

    fn get_pledge(id: u64) -> u64 {
        validate_id(id);
        storage.pledgers.get((id, sender_identity()))
    }
}

fn validate_id(id: u64) {
    require(id != 0 && id <= storage.campaign_count, Error::NoSuchCampaign);
}

fn validate_deadline(campaign: Campaign, id: u64) {
    // workaround since passing in mut does not work yet
    let mut campaign = campaign;

    if campaign.deadline < height() {
        // if campaign.state != State::Successful || campaign.state != State::Failed {
        //     campaign.state = if campaign.target_amount < campaign.total_pledge { State::Failed } else { State::Successful };

        //     storage.campaigns.insert(id, campaign);
        // }
        // workaround for Eq
        // TODO: correct logic
        if campaign.state != 1 || campaign.state != 2 {
            campaign.state = if campaign.target_amount < campaign.total_pledge {
                2
            } else {
                1
            };

            storage.campaigns.insert(id, campaign);
        }
    }
}

fn validate_sender(sender: Identity, other: Identity) {
    match sender {
        Identity::Address(sender) => {
            match other {
                Identity::Address(user) => require(sender.value == user.value, UserError::UnauthorizedUser), _ => revert(42), 
            }
        },
        Identity::ContractId(sender) => {
            match other {
                Identity::ContractId(user) => require(sender.value == user.value, UserError::UnauthorizedUser), _ => revert(42), 
            }
        }
    };
}

fn sender_identity() -> Identity {
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}

fn transfer(to: Identity, amount: u64, asset: ContractId) {
    match to {
        Identity::Address(address) => {
            transfer_to_output(amount, asset, address);
        },
        Identity::ContractId(address) => {
            force_transfer_to_contract(amount, asset, address);
        }
    }
}
