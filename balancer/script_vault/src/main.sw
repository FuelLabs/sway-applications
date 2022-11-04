script;

use std::{address::Address, contract_id::ContractId, vec::Vec};

enum PoolSpecialization {
    General:(),
    MinimalSwapInfo: (),
    TwoToken: (),
}

pub struct JoinPoolRequest {
    assets: Vec<ContractId>,
    from_internal_balance: bool,
    max_amounts_in: Vec<u64>,
    // todo this was used in the abi.decode, removing dependencies fo abi.decode
    // user_data: Vec<b256>,
    user_data: UserData,
}
enum RequestKind {
    ExactToken: (),
    ExactTokensOut: (),
    Init: (),
    InForExactTokensOut: (),
    Token: (),
}

//todo of user_data: Vec<b256>
pub struct UserData {
    amount: u64,
    amounts_in_out: Vec<u64>,
    bpt_amount_in_out: u64,
    kind: RequestKind,
    max_min_bpt_amount: u64,
}

pub enum SwapKind {
    GivenIn: (),
    GivenOut: (),
}

struct SingleSwap {
    amount: u64,
    asset_in: ContractId,
    asset_out: ContractId,
    kind: SwapKind,
    pool_id: b256,
    user_data: b256,
}

struct FundManagement {
    from_internal_balance: bool,
    recipient: Address,
    sender: Address,
    to_internal_balance: bool,
}

abi vault {
    #[storage(read, write)]
    fn register_pool(pool_id: b256, specialization: PoolSpecialization) -> b256;
    #[storage(read, write)]
    fn register_tokens(
        pool_id: b256,
        tokens: Vec<ContractId>,
        asset_managers: Vec<Address>,
    );
    #[storage(read, write)]
    fn join_pool(
        pool_id: b256,
        sender: Address,
        recipient: Address,
        request: JoinPoolRequest,
    );
    #[storage(read)]
    fn get_pool(pool_id: b256) -> (ContractId, PoolSpecialization);
    #[storage(read)]
    fn get_pool_tokens(pool_id: b256) -> (Vec<ContractId>, Vec<u64>, u64);
    #[storage(read, write)]
    fn swap(
        single_swap: SingleSwap,
        funds: FundManagement,
        limit: u64,
        deadline: u64,
    ) -> u64;
}

fn main() {
    let caller = abi(vault, 0x68b0ef7f5d7a482840c1f4951b9be43f71980ea621090567a8c6090d49c37c56);
    let pool_id = 0x34e50d5b80a0391081fa746ca17c499d8e3ca990f1e3a62f8417e54092d24934;

    caller.register_pool(pool_id, PoolSpecialization::TwoToken);

    // caller.register_pool(pool_id, PoolSpecialization::MINIMAL_SWAP_INFO);
    // caller.register_pool(pool_id, PoolSpecialization::GENERAL);
    let mut tokens = ~Vec::new();
    tokens.push(~ContractId::from(0x34e50d5b80a0391081fa756ba16c499d6e3ca990f0e3a62f8417e54092d24934));
    tokens.push(~ContractId::from(0x7e6d6d8a583394cc0385b3cc7fbfc157aedacb0ab7808bb53f71943480b9659e));
    let mut asset_managers = ~Vec::new();
    asset_managers.push(~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000));
    asset_managers.push(~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000));

    caller.register_tokens(pool_id, tokens, asset_managers);

    // caller.register_tokens(pool_id, PoolSpecialization::MINIMAL_SWAP_INFO, tokens, asset_managers);
    // caller.register_tokens(pool_id, PoolSpecialization::GENERAL, tokens, asset_managers);
    let sender = ~Address::from(0x6b63804cfbf9856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0418e);
    let recipient = ~Address::from(0x6b63804cfbf7856e68e5b6e7aef238dc8311ec55bec04df774003a2c96e0418e);
    let mut assets = ~Vec::new();
    assets.push(~ContractId::from(0x6b63804cfbf9856e68e5b6e7bef238dd8311ec55bec04df774003a2c96e0418e));
    assets.push(~ContractId::from(0x6b63804cfbf9856e68e5b6e7bef238dc8311ec55bec04df774003a2c96e0418e));
    // assets.push(~ContractId::from(0x6b63804cfbf9856e68e5b6e7bef238dc8311ec55cec04df774003a2c96e0418e));
    let mut max_amounts_in = ~Vec::new();
    max_amounts_in.push(8974);
    max_amounts_in.push(517);
    // max_amounts_in.push(21);
    let mut in_out = ~Vec::new();
    in_out.push(244);
    in_out.push(10);
    // in_out.push(34);
    let user_data = UserData {
        amount: 1,
        amounts_in_out: in_out,
        bpt_amount_in_out: 2,
        kind: RequestKind::Init,
        max_min_bpt_amount: 3088,
    };
    let request = JoinPoolRequest {
        assets: assets,
        from_internal_balance: false,
        max_amounts_in: max_amounts_in,
        user_data: user_data,
    };

    caller.join_pool(pool_id, sender, recipient, request);


    caller.get_pool(pool_id);
    caller.get_pool_tokens(pool_id);
    let ss = SingleSwap {
        amount: 1000,
        asset_in: ~ContractId::from(0x6b63804cfbf9856e68e5b6e7bef238dd8311ec55bec04df774003a2c96e0418e),
        asset_out: ~ContractId::from(0x6b63804cfbf9856e68e5b6e7bef238dc8311ec55bec04df774003a2c96e0418b),
        kind: SwapKind::GivenIn,
        pool_id: pool_id,
        user_data: 0x0000000000000000000000000000000000000000000000000000000000000000,
    };
    let fm = FundManagement {
        from_internal_balance: false,
        recipient: ~Address::from(0x54944e5b8189827e470e5a8bacfc6c3667397dc4e1eef7ef3519d16d6d6c6610),
        sender: ~Address::from(0x54944e5b8189827e470e5a8bacfc6c3667397dc4e1eef7ef3519d16d6d6c6610),
        to_internal_balance: false,
    };
    // caller.swap(
    //     ss, fm, 185, 11579208
    // );
}