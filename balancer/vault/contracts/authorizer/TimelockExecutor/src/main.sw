
contract;

use std::{
    address::*,

    vec::Vec,
};

abi TimelockExecutor {
    fn execute(target: Address, data: b256) -> b256;
}

// use TimelockAuthorizer::*;

const authorizer:b256 = TimelockAuthorizer(msg.sender);

impl TimelockExecutor for Contract {    
    // TimelockAuthorizer public immutable authorizer;


    fn execute( target: Address, data: b256) -> (b256) {
        require(msg.sender == ~Address::from(authorizer), "ERR_SENDER_NOT_AUTHORIZER");
        Address::functionCall(target, data);
    }
}
