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
    token::{force_transfer_to_contract, transfer_to_output},
};

use abi::Fundraiser;
use data_structures::{Campaign, State};
use errors::{Error, StateError, UserError};
use events::{CancelledEvent, ClaimedEvent, CreatedCampaign, PledgedEvent, UnpledgedEvent};

storage {
    campaign_count: u64,

    /// campaign identifier => Campaign Data
    campaigns: StorageMap<u64, Campaign>,

    owner: Identity,

    /// campaign identifier => user => amount pledged
    pledgers: StorageMap<(u64, Identity), u64>,

    initialized: bool,
}

impl Fundraiser for Contract {

    fn constructor(owner: Identity) {
        require(!storage.initialized, StateError::CannotReinitialize);
        storage.owner = owner;
        storage.initialized = true;
    }

    fn create_campaign(author: Identity, asset: ContractId, target_amount: u64, deadline: u64) {
        require(storage.initialized, UserError::UnauthorizedUser);
        let sender: Result<Identity, AuthError> = msg_sender();
        // require(storage.owner == sender.unwrap(), UserError::UnauthorizedUser);
        // workaround
        match sender.unwrap() {
            Identity::Address(sender) => {
                match storage.owner {
                    Identity::Address(owner) => require(sender.value == owner.value, UserError::UnauthorizedUser),
                    _ => revert(42),
                }
            },
            Identity::ContractId(sender) => {
                match storage.owner {
                    Identity::ContractId(owner) => require(sender.value == owner.value, UserError::UnauthorizedUser),
                    _ => revert(42),
                }
            }
        }

        storage.campaign_count = storage.campaign_count + 1;
        let campaign = Campaign { 
            author,         // TODO: require non-zero author (0x000...)?
            asset,          // TODO: require non-zero asset (0x000...)?
            target_amount, 
            deadline,       // TODO: require time (block height) in the future?
            claimed: false, 
            total_pledge: 0, 
            state: State::Funding 
        };
        storage.campaigns.insert(storage.campaign_count, campaign);

        log(CreatedCampaign { id: storage.campaign_count, campaign });
    }

    fn pledge(id: u64) {
        _campaign_exists(id);

        let mut campaign = storage.campaigns.get(id);
        _check_deadline(campaign, id);

        // require(campaign.state == State::Funding, StateError::FundraiseEnded);
        require(campaign.asset == msg_asset_id(), UserError::IncorrectAssetSent);

        let pledge_amount = msg_amount();
        let sender: Result<Identity, AuthError> = msg_sender();
        let user_pledge = storage.pledgers.get((id, sender.unwrap())) + pledge_amount;

        campaign.total_pledge = campaign.total_pledge + pledge_amount;

        storage.campaigns.insert(id, campaign);
        storage.pledgers.insert((id, sender.unwrap()), user_pledge);

        log(PledgedEvent {user: sender.unwrap(), amount: pledge_amount, id});
    }

    fn unpledge(id: u64, amount: u64) {
        _campaign_exists(id);

        let mut campaign = storage.campaigns.get(id);
        _check_deadline(campaign, id);

        // require(campaign.state != State::Successful, StateError::CannotUnpledgeSuccessfulFundraise);

        let sender: Result<Identity, AuthError> = msg_sender();
        let user_pledge = storage.pledgers.get((id, sender.unwrap()));

        // Not supported in signature
        let mut amount = amount;
        if user_pledge < amount {
            amount = user_pledge;
        }

        storage.pledgers.insert((id, sender.unwrap()), user_pledge - amount);
        campaign.total_pledge = campaign.total_pledge - amount;

        _transfer(sender.unwrap(), amount, campaign.asset);

        log(UnpledgedEvent {user: sender.unwrap(), amount, id});
    }
    
    fn claim(id: u64) {
        _campaign_exists(id);

        let mut campaign = storage.campaigns.get(id);
        _check_deadline(campaign, id);

        require(!campaign.claimed, UserError::AlreadyClaimed);
        // require(campaign.state == State::Successful, StateError::FundraiseNotSuccessful);

        let sender: Result<Identity, AuthError> = msg_sender();
        // require(campaign.author == sender.unwrap(), UserError::UnauthorizedUser);

        campaign.claimed = true;
        _transfer(sender.unwrap(), campaign.total_pledge, campaign.asset);

        log(ClaimedEvent {user: sender.unwrap(), amount: campaign.total_pledge, id});
    }

    fn cancel(id: u64) {
        _campaign_exists(id);
        
        let mut campaign = storage.campaigns.get(id);
        // require(campaign.state == State::Funding, StateError::FundraiseEnded);

        let sender: Result<Identity, AuthError> = msg_sender();
        // require(campaign.author == sender.unwrap(), UserError::UnauthorizedUser);

        campaign.state = State::Cancelled;
        storage.campaigns.insert(id, campaign);

        log(CancelledEvent {user: sender.unwrap(), id});
    }

    fn get_campaign(id: u64) -> Campaign {
        _campaign_exists(id);
        storage.campaigns.get(id)
    }

    fn get_pledge(id: u64) -> u64 {
        _campaign_exists(id);
        let sender: Result<Identity, AuthError> = msg_sender();
        storage.pledgers.get((id, sender.unwrap()))
    }
}

fn _campaign_exists(id: u64) {
    require(id != 0 && id <= storage.campaign_count, Error::NoSuchCampaign);
}

fn _check_deadline(campaign: Campaign, id: u64) {
    let mut campaign = campaign;

    // if campaign.deadline < height() {
    //     if campaign.state != State::Successful || campaign.state != State::Failed {
    //         campaign.state = if campaign.target_amount < campaign.total_pledge { State::Failed } else { State::Successful };

    //         storage.campaigns.insert(id, campaign);
    //     }
    // }
}

fn _transfer(to: Identity, amount: u64, asset: ContractId) {
    match to {
        Identity::Address(address) => {
            transfer_to_output(amount, asset, address);
        },
        Identity::ContractId(address) => {
            force_transfer_to_contract(amount, asset, address);
        }
    }
}
