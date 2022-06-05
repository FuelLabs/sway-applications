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

use abi::Crowdfund;
use data_structures::{State, Info};
use errors::{StateError, UserError};
use events::{ClaimedEvent, PledgedEvent, UnpledgedEvent};

storage {
    author: Sender,
    asset: ContractId,
    claimed: bool,
    deadline: u64,
    pledgers: StorageMap<Sender, u64>,
    state: State,
    target_amount: u64,
    total_pledge: u64,
}

impl Crowdfund for Contract {
    fn constructor(author: Sender, asset: ContractId, target_amount: u64, deadline: u64) -> bool {
        require(storage.state == State::Void, StateError::CannotReinitialize);
        
        storage.author = author;
        storage.asset = asset;
        storage.target_amount = target_amount;
        storage.deadline = deadline;

        storage.state = State::Funding;

        true
    }

    fn pledge() -> bool {
        _check_deadline();

        require(storage.state == State::Funding, StateError::CrowdFundEnded);
        require(storage.asset == msg_asset_id(), UserError::IncorrectAssetSent);

        let amount = msg_amount();
        let sender: Result<Sender, AuthError> = msg_sender();
        let user_pledge = storage.pledgers.get(sender.unwrap()) + amount;

        storage.pledgers.insert(sender.unwrap(), user_pledge);
        storage.total_pledge = storage.total_pledge + amount;

        log(PledgedEvent {user: sender.unwrap(), amount});

        true
    }

    fn unpledge(amount: u64) -> bool {
        _check_deadline();

        require(storage.state != State::Successful, StateError::CannotUnpledgeSuccessfulCrowdfund);

        let sender: Result<Sender, AuthError> = msg_sender();
        let user_pledge = storage.pledgers.get(sender.unwrap());

        // Not supported in signature
        let mut amount = amount;
        if user_pledge < amount {
            amount = user_pledge;
        }

        storage.pledgers.insert(sender.unwrap(), user_pledge - amount);
        storage.total_pledge = storage.total_pledge - amount;

        _transfer(sender.unwrap(), amount, storage.asset);

        log(UnpledgedEvent {user: sender.unwrap(), amount});

        true
    }
    
    fn claim() -> bool {
        _check_deadline();

        require(!storage.claimed, UserError::AlreadyClaimed);
        require(storage.state == State::Successful, StateError::CrowdFundNotSuccessful);

        let sender: Result<Sender, AuthError> = msg_sender();
        require(storage.author == sender.unwrap(), UserError::UnauthorizedUser);

        storage.claimed = true;
        _transfer(sender.unwrap(), storage.total_pledge, storage.asset);

        log(ClaimedEvent {user: sender.unwrap(), amount: storage.total_pledge});

        true
    }
    
    fn status() -> Info {
        Info {
            claimed: storage.claimed,
            remaining_time: if storage.deadline < height() { 0 } else { storage.deadline - height() },
            state: storage.state,
            target_amount: storage.target_amount,
            total_pledge: storage.total_pledge,
        }
    }
}

fn _check_deadline() {
    if storage.deadline < height() {
        if storage.state != State::Successful || storage.state != State::Failed {
            storage.state = if storage.target_amount < storage.total_pledge { State::Failed } else { State::Successful };
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
