library errors;

pub enum AccessError {
    AlreadyInitialized: (),
    NFTTransferNotApproved: (),
    NotNftOwner: (),
}

pub enum AssetError {
    SupplyNotReturned: (),
}
