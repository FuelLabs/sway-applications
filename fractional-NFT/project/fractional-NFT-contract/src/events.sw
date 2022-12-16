library events;

pub struct Deposited {
    /// The contract which manages the NFT.
    nft: ContractId,
    /// The user with permission to unlock the NFT from the contract.
    owner: Option<Identity>,
    /// The total number of tokens that may ever be minted.
    supply: u64,
    /// The id of the NFT locked in the contract.
    token_id: u64,
}

pub struct OwnerChanged {
    /// The new identity which may unlock the contract.
    new_owner: Option<Identity>,
    /// The old identity which has given up ownership of this contract.
    previous_owner: Identity,
}

pub struct Withdraw {
    /// The contract which manages the NFT.
    nft: ContractId,
    /// The identity which ownership of the NFT has been sent to.
    owner: Identity,
    /// The id of the NFT which has been unlocked.
    token_id: u64,
}
