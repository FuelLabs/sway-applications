library errors;

pub enum AccessError {
    AlreadyInitialized: (),
    NFTTransferNotApproved: (),
    NoNftDeposited: (),
    NotNftOwner: (),
}

pub enum AssetError {
    SupplyNotReturned: (),
}
