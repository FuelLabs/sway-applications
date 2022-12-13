library errors;

pub enum AccessError {
    DistributionAlreadyExists: (),
    InvalidState: (),
    NoReserveAvailable: (),
    NotFNftOwner: (),
}

pub enum AssetError {
    InvalidAssetTransfer: (),
    NotEnoughTokensAvailable: (),
}
