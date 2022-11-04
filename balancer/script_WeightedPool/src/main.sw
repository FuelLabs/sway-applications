script;

use std::{
    contract_id::ContractId,
    address::Address,
    vec::Vec,
};

pub enum RequestKind {
    INIT: (),
    EXACT_TOKEN: (),
    EXACT_TOKENS_OUT: (),
    IN_FOR_EXACT_TOKENS_OUT: (),
    TOKEN: (),
}

pub struct UserData {
    kind: RequestKind,
    amount: u64,
    maxMinBPTAmount: u64,
    bptAmountInOut: u64,
    amountsInOut: Vec<u64>,  
}

abi WeightedPool {
    #[storage(read, write)]
    fn on_join_pool(
        poolId: b256,
        // sender: Address,
        // recipient: ContractId,
        // balances: Vec<u64>,
        sender: b256,
        recipient: b256,
        balances_array: [u64; 2],
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: UserData,
    );
    // -> (Vec<u64>, Vec<u64>);
}


fn main() {
    let caller = abi(WeightedPool, 0xe36caa830094b6d8ad8bbf082aa76148157dcc10d136e5dd9b3caba0402600a1);
    let poolId: b256 = 0x0bba70540a1e2657fe2b2b8e97d405d27a890ccd8b57a4dc722e76c506d985e3;
    let id = 0x6b63804cfbf9856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0418e;
    let address = 0x6b63804cfbf9856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0418e;
    // let mut balances = ~Vec::new();
    // balances.push(8974);
    // balances.push(517);
    let balances = [8974, 517];
    let lastChangeBlock = 200;
    let protocolSwapFeePercentage = 10; 
    let mut in_out = ~Vec::new();
    in_out.push(244);
    in_out.push(10);
    let userData = UserData {
        kind: RequestKind::INIT,
        amount: 1,
        maxMinBPTAmount: 3088,
        bptAmountInOut: 2,
        amountsInOut: in_out,   
    };
    
    caller.on_join_pool(
        poolId, address, id, balances, lastChangeBlock, protocolSwapFeePercentage, userData
    );

    // caller.set_swap_fee_percentage(10000000000);

    // // let asset_id = ~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);
    // // let target = ~ContractId::from(0x6b63804cfbf9856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0417e);
    // // caller.force_transfer_coins(10000, asset_id, target);

    // // let recipient = ~Address::from(0x6b63804cfbf9856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0417e);
    // // caller.transfer_coins_to_output(10000, asset_id, recipient);

    // let ret_data_get_normalized_weights = caller.get_normalized_weights();

    // let ret_data_get_vault = caller.get_vault();
}

