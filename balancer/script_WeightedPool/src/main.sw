script;

use std::{
    contract_id::ContractId,
    address::Address,
    vec::Vec,
    logging::log
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
        sender: ContractId,
        recipient: ContractId,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: UserData,
    ) -> (Vec<u64>, Vec<u64>);

    #[storage(read, write)]
    fn on_exit_pool(
        poolId: b256,
        sender: ContractId,
        recipient: ContractId,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: UserData,
    ) -> (Vec<u64>, Vec<u64>);

    #[storage(read, write)]
    fn set_swap_fee_percentage(swapFeePercentage: u64);

    fn force_transfer_coins(coins: u64, asset_id: ContractId, target: ContractId);

    fn transfer_coins_to_output(coins: u64, asset_id: ContractId, recipient: Address);

    fn get_normalized_weights() -> Vec<u64>;

    #[storage(read)]
    fn get_vault() -> ContractId;

    #[storage(read, write)]
    fn query_exit(
        poolId: b256,
        sender: ContractId,
        arecipient: ContractId,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: UserData,
    ) -> (u64, Vec<u64>);

    #[storage(read, write)]
    fn query_join(
        poolId: b256,
        sender: ContractId,
        arecipient: ContractId,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: UserData,
    ) -> (u64, Vec<u64>);
}


fn main() {
    let contract_id = 0xe4fd909ea1bc2d441f760ca1b469dabf081009b9cb2341220b1399c5a44aa3dd;
    let caller = abi(WeightedPool, contract_id);
    let poolId: b256 = 0x0bba70540a1e2657fe2b2b8e97d405d27a890ccd8b57a4dc722e76c506d985e3;
    let id = ~ContractId::from(0x6b63804cfbf9856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0418e);
    let address = ~ContractId::from(0x6b63804cfbf9856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0418e);
    let mut on_exit_pool_balances = ~Vec::new();
    on_exit_pool_balances.push(10);
    on_exit_pool_balances.push(10);
    let mut on_join_pool_balances = ~Vec::new();
    on_join_pool_balances.push(8974);
    on_join_pool_balances.push(517);
    let lastChangeBlock = 200;
    let protocolSwapFeePercentage = 10; 
    let mut in_out = ~Vec::new();
    in_out.push(244);
    in_out.push(1000);
    let on_exit_pool_user_data = UserData {
        kind: RequestKind::EXACT_TOKEN,
        amount: 1,
        maxMinBPTAmount: 1,
        bptAmountInOut: 2,
        amountsInOut: in_out,   
    };
    let on_join_pool_user_data = UserData {
        kind: RequestKind::INIT,
        amount: 100,
        maxMinBPTAmount: 3088,
        bptAmountInOut: 2000,
        amountsInOut: in_out,   
    };
    let query_join_user_data = UserData {
        kind: RequestKind::EXACT_TOKEN,
        amount: 100,
        maxMinBPTAmount: 0,
        bptAmountInOut: 2000,
        amountsInOut: in_out,   
    };

    caller.on_join_pool(
        poolId, address, id, on_join_pool_balances, lastChangeBlock, protocolSwapFeePercentage, on_join_pool_user_data
    );

    caller.query_join(
        poolId, address, id, on_join_pool_balances, lastChangeBlock, protocolSwapFeePercentage, query_join_user_data
    );


    caller.on_exit_pool(
        poolId, address, id, on_exit_pool_balances, lastChangeBlock, protocolSwapFeePercentage, on_exit_pool_user_data
    );

    caller.query_exit(
        poolId, address, id, on_exit_pool_balances, lastChangeBlock, protocolSwapFeePercentage, on_exit_pool_user_data
    );

    caller.set_swap_fee_percentage(10000000000);

    let asset_id = ~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);

    let ret_data_get_normalized_weights = caller.get_normalized_weights();

    let ret_data_get_vault = caller.get_vault();

    let target = ~ContractId::from(contract_id);
    caller.force_transfer_coins(10000, asset_id, target);

    let recipient = ~Address::from(contract_id);
    caller.transfer_coins_to_output(10000, asset_id, recipient);
}

