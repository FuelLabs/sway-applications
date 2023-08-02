library;

pub enum AccessError {
    DistributionAlreadyExists: (),
    DistributionDoesNotExist: (),
    InvalidState: (),
    NoReserveAvailable: (),
    NotTokenAdmin: (),
}

pub enum AssetError {
    InvalidAssetTransfer: (),
    NotEnoughTokensAvailable: (),
}
