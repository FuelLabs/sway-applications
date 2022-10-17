predicate;

use std::{
    b512::B512,
    constants::ZERO_B256,
    ecr::ec_recover,
    inputs::input_predicate_data,
    
};

fn main() -> bool {
    //Public key of a known EVM account
    let spender_pub_key = ~B512::from(config_spender_h, config_spender_l);

    let signature: B512 = input_predicate_data(0);

    let pub_key_result = ec_recover(signature, ZERO_B256);//placeholder tx_hash
    require(pub_key_result.is_ok(), "Unable to recover public key");
    let pub_key = pub_key_result.unwrap();

    pub_key == spender_pub_key
}