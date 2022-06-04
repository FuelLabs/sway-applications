contract;

// TODO: enums not supported in storage yet so it won't compile

dep abi;
dep data_structures;
dep errors;
dep events;

use std::{
    assert::require,
    chain::auth::{AuthError, Sender, msg_sender},
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
    logging::log,
    result::*,
    storage::StorageMap,
};

use abi::Crowdfund;
use data_structures::{State, Status};
use errors::Error;
use events::{PledgedEvent, UnpledgedEvent};

storage {
    author: Sender,
    asset: ContractId,
    deadline: u64,
    pledgers: StorageMap<Sender, u64>,
    state: State,
    target_amount: u64,
}

impl Crowdfund for Contract {
    fn constructor(author: Sender, asset: ContractId, target_amount: u64, deadline: u64) -> bool {
        // TODO: error handling

        storage.author = author;
        storage.asset = asset;
        storage.target_amount = target_amount;
        storage.deadline = deadline;

        true
    }

    fn pledge() -> bool {
        require(storage.asset == msg_asset_id(), "TODO");
        let sender: Result<Sender, AuthError> = msg_sender();
        let pledge = msg_amount();
        let total_pledge = storage.pledgers.get(sender.unwrap()) + pledge;
        storage.pledgers.insert(sender.unwrap(), total_pledge);

        log(PledgedEvent {user: sender.unwrap(), amount: pledge});
        true
    }

    fn unpledge(amount: u64) -> bool {
        true
    }
    
    fn claim() -> bool {
        true
    }
    
    fn status() -> Status {
        Status {
            state: storage.state,
            target_amount: storage.target_amount,
            remaining_time: 1, // TODO
            pledged_amount: this_balance(storage.asset),
        }
    }
}
