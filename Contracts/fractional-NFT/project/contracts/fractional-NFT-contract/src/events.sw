library;

pub struct AdminEvent {
    /// The new identity which may unlock the contract and change the admin.
    new_admin: Option<Identity>,
    /// The old identity which has given up admin rights of this contract.
    previous_admin: Identity,
}

pub struct DepositEvent {
    /// The user with permission to unlock the NFT from the contract.
    admin: Option<Identity>,
    /// The contract which manages the NFT.
    asset_id: ContractId,
    /// The total number of tokens that may ever be minted.
    supply: u64,
    /// The id of the NFT locked in the contract.
    token_id: u64,
}

pub struct WithdrawEvent {
    /// The contract which manages the NFT.
    asset_id: ContractId,
    /// The identity which ownership of the NFT has been sent to.
    owner: Identity,
    /// The id of the NFT which has been unlocked.
    token_id: u64,
}
