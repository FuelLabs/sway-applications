script;

use std::{
    address::Address,
    contract_id::ContractId,
    vec::Vec,
};

enum PoolSpecialization {
    GENERAL: (),
    MINIMAL_SWAP_INFO: (),
    TWO_TOKEN: (),
}

pub struct JoinPoolRequest {
    // assets: Vec<ContractId>,
    // maxAmountsIn: Vec<u64>,
    assets: [b256; 2],
    maxAmountsIn: [u64; 2],
    // todo this was used in the abi.decode, removing dependencies fo abi.decode
    // userData: Vec<b256>,
    userData: UserData,
    fromInternalBalance: bool,
}

enum RequestKind {
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
    // amountsInOut: Vec<u64>,
    amountsInOut: [u64; 2],
}

struct GetPoolTokens{
    tk1: b256,
    tk2: b256,
    am1: u64,
    am2: u64,
    ts: u64,
}


abi vault {
    #[storage(read,write)] 
    fn register_pool(poolId: b256, specialization: PoolSpecialization,) -> b256;
    #[storage(read, write)]#[storage(read, write)]
    fn register_tokens(
        poolId: b256,
        // specialization: PoolSpecialization,
        // tokens: Vec<ContractId>,
        tokens1: [b256; 8],
        // assetManagers: Vec<Address>,
        assetManagers1: [b256; 8],
    );
    #[storage(read, write)]
    fn join_pool(
        poolId: b256,
        sender: Address,
        recipient: Address,
        request: JoinPoolRequest,
    );
    fn get_pool_tokens(poolId: b256) -> GetPoolTokens;
}


fn main() {
    let caller = abi(vault, 0x6de824436002d158aa6e55c690636a98fa13f63d8afed3b447aa0a1307da33b0);
    let pool_id = 0x34e50d5b80a0391081fa746ba17c499d8e3ca990f1e3a62f8417e54092d24934;
    
    let sender = ~Address::from(0x6b63804cfbf9856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0418e);
    let recipient = ~Address::from(0x6b63804cfbf7856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0418e);
    // let mut assets = ~Vec::new();
    // assets.push(~ContractId::from(0x6b63804cfbf9856e68e5b6e7bef238dd8311ec55bec04df774003a2c96e0418e));
    // assets.push(~ContractId::from(0x6b63804cfbf9856e68e5b6e7bef238dc8311ec55bec04df774003a2c96e0418e));
    let ass1 = 0x6b63804cfbf9856e68e5b6e7bef238dd8311ec55bec04df774003a2c96e0418e;
    let ass2 = 0x6b63804cfbf9856e68e5b6e7bef238dc8311ec55bec04df774003a2c96e0418e;
    let ass3 = 0x6b63804cfbf9856e68e5b6e7bef238ed8311ec55bec04df774003a2c96e0418e;
    let ass4 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let assets = [ass1, ass2];
    // let mut maxAmountsIn = ~Vec::new();
    // maxAmountsIn.push(8974);
    // maxAmountsIn.push(517);
    let maxAmountsIn = [8974, 517];
    // let mut in_out = ~Vec::new();
    // in_out.push(244);
    // in_out.push(10);
    let in_out = [244, 10];
    let fromInternalBalance: bool = false;
    let userData = UserData {
        kind: RequestKind::INIT,
        amount: 1,
        maxMinBPTAmount: 3088,
        bptAmountInOut: 2,
        amountsInOut: in_out,   
    };
    let request = JoinPoolRequest {
        assets: assets,
        maxAmountsIn: maxAmountsIn,
        userData: userData,
        fromInternalBalance: fromInternalBalance,
    };
    

    caller.register_pool(pool_id, PoolSpecialization::TWO_TOKEN);


    // // caller.register_pool(pool_id, PoolSpecialization::MINIMAL_SWAP_INFO);
    // // caller.register_pool(pool_id, PoolSpecialization::GENERAL);
    let tk1 = 0x34e50d5b80a0391081fa756ba16c499d6e3ca990f0e3a62f8417e54092d24934;
    let tk2 = 0x34e50d5b80a0391081fb756ba16c499d6e3ca990f0e3a62f8417e54092d24934;
    let tk3 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let tk4 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let tk5 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let tk6 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let tk7 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let tk8 = 0x0000000000000000000000000000000000000000000000000000000000000000;

    let am1 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let am2 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let am3 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let am4 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let am5 = 0x0000000000000000000000000000000000000000000000000000000000000000;    
    let am6 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let am7 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let am8 = 0x0000000000000000000000000000000000000000000000000000000000000000;

    let tokens: [b256; 8] = [tk1, tk2, tk3, tk4, tk5, tk6, tk7, tk8];
    let ams: [b256; 8] = [am1, am2, am3, am4, am5, am6, am7, am8];

    // // let tokens: [ContractId; 2] = [~ContractId::from(0x34e50d5b80a0391081fa756ba16c499d6e3ca990f0e3a62f8417e54092d24934), ~ContractId::from(0x7e6d6d8a583394cc0385b3cc7fbfc157aedacb0ab7808bb53f71943480b9659e)];
    // // let assetManagers: [Address; 2] = [~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000), ~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000)];
    // // tokens.push(~ContractId::from(0x34e50d5b80a0391081fa756ba16c499d6e3ca990f0e3a62f8417e54092d24934));
    // // tokens.push(~ContractId::from(0x7e6d6d8a583394cc0385b3cc7fbfc157aedacb0ab7808bb53f71943480b9659e));
    // // let mut assetManagers = ~Vec::new();
    // // assetManagers.push(~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000));
    // // assetManagers.push(~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000));
    caller.register_tokens(pool_id, tokens, ams);
    caller.join_pool(
        pool_id, sender, recipient, request
    );
    // caller.get_pool_tokens(pool_id);
    // // caller.register_tokens(pool_id, PoolSpecialization::MINIMAL_SWAP_INFO, tokens, assetManagers);
    // // caller.register_tokens(pool_id, PoolSpecialization::GENERAL, tokens, assetManagers);
}