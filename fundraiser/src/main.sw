contract;

// TODO: enums not supported in storage yet so it won't compile
//       better deadline handling instead of height()?

dep abi;
dep data_structures;
dep errors;
dep events;

use std::{
    assert::require,
    block::height,
    chain::auth::{AuthError, Sender, msg_sender},
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
    logging::log,
    result::*,
    storage::StorageMap,
    token::{force_transfer, transfer_to_output},
};

use abi::Fundraiser;
use data_structures::{Campaign, State};
use errors::{Error, Initialized, StateError, UserError};
use events::{ClaimedEvent, PledgedEvent, UnpledgedEvent};

storage {
    campaign_count: u64,

    /// campaign identifier => data
    campaigns: StorageMap<u64, Campaign>,

    owner: Sender,

    /// campaign identifier => user => amount pledged
    pledgers: StorageMap<(u64, Sender), u64>,

    state: Initialized,
}

impl Fundraiser for Contract {

    fn constructor(owner: Sender) {
        require(storage.state == Initialized::False, StateError::CannotReinitialize);
        storage.owner = owner;
        storage.state = Initialized::True;
    }

    fn create_campaign(author: Sender, asset: ContractId, target_amount: u64, deadline: u64) {
        let sender: Result<Sender, AuthError> = msg_sender();
        require(storage.owner == sender.unwrap(), UserError::UnauthorizedUser);

        storage.campaign_count = storage.campaign_count + 1;
        storage.campaigns.insert(storage.campaign_count, Campaign { 
            author,         // TODO: require non-zero author (0x000...)?
            asset,          // TODO: require non-zero asset (0x000...)?
            target_amount, 
            deadline,       // TODO: require time (block height) in the future?
            claimed: false, 
            total_pledge: 0, 
            state: State::Funding 
        })
    }

    fn pledge(campaign_identifier: u64) {
        _campaign_exists(identifier);

        let mut campaign = storage.campaigns.get(campaign_identifier);
        _check_deadline(campaign);

        require(campaign.state == State::Funding, StateError::FundraiseEnded);
        require(campaign.asset == msg_asset_id(), UserError::IncorrectAssetSent);

        let pledge_amount = msg_amount();
        let sender: Result<Sender, AuthError> = msg_sender();
        let user_pledge = storage.pledgers.get((campaign_identifier, sender.unwrap())) + pledge_amount;

        campaign.total_pledge = storage.total_pledge + pledge_amount;

        storage.campaigns.insert(campaign_identifier, campaign);
        storage.pledgers.insert((campaign_identifier, sender.unwrap()), user_pledge);

        log(PledgedEvent {user: sender.unwrap(), amount, campaign_identifier});
    }

    fn unpledge(campaign_identifier: u64, amount: u64) {
        _campaign_exists(campaign_identifier);

        let mut campaign = storage.campaigns.get(campaign_identifier);
        _check_deadline(campaign);

        require(campaign.state != State::Successful, StateError::CannotUnpledgeSuccessfulFundraise);

        let sender: Result<Sender, AuthError> = msg_sender();
        let user_pledge = storage.pledgers.get((campaign_identifier, sender.unwrap()));

        // Not supported in signature
        let mut amount = amount;
        if user_pledge < amount {
            amount = user_pledge;
        }

        storage.pledgers.insert((campaign_identifier, sender.unwrap()), user_pledge - amount);
        storage.total_pledge = storage.total_pledge - amount;

        _transfer(sender.unwrap(), amount, campaign.asset);

        log(UnpledgedEvent {user: sender.unwrap(), amount, campaign_identifier});
    }
    
    fn claim(campaign_identifier: u64) {
        _campaign_exists(campaign_identifier);

        let mut campaign = storage.campaigns.get(campaign_identifier);
        _check_deadline(campaign);

        require(!campaign.claimed, UserError::AlreadyClaimed);
        require(campaign.state == State::Successful, StateError::FundraiseNotSuccessful);

        let sender: Result<Sender, AuthError> = msg_sender();
        require(campaign.author == sender.unwrap(), UserError::UnauthorizedUser);

        campaign.claimed = true;
        _transfer(sender.unwrap(), campaign.total_pledge, campaign.asset);

        log(ClaimedEvent {user: sender.unwrap(), amount: storage.total_pledge});
    }

    fn get_campaign(campaign_identifier: u64) -> Campaign {
        _campaign_exists(campaign_identifier);
        storage.campaigns.get(campaign_identifier)
    }

    fn get_pledge(campaign_identifier: u64) -> u64 {
        _campaign_exists(campaign_identifier);
        let sender: Result<Sender, AuthError> = msg_sender();
        storage.pledgers.get((campaign_identifier, sender.unwrap()))
    }
}

fn _campaign_exists(campaign_identifier: u64) {
    require(campaign_identifier != 0 && campaign_identifier <= storage.campaign_count, Error::NoSuchCampaign);
}

fn _check_deadline(campaign: Campaign) {
    let mut campaign = campaign;

    if campaign.deadline < height() {
        if campaign.state != State::Successful || campaign.state != State::Failed {
            campaign.state = if campaign.target_amount < campaign.total_pledge { State::Failed } else { State::Successful };

            storage.campaigns.insert(campaign_identifier, campaign);
        }
    }
}

fn _transfer(to: Sender, amount: u64, asset: ContractId) {
    match to {
        Sender::Address(address) => {
            transfer_to_output(amount, asset, address);
        },
        Sender::ContractId(address) => {
            force_transfer(amount, asset, address);
        }
    }
}
