library errors;

pub enum AuthorizationError {
    CannotReinitialize: (),
    SenderNotAdmin: (),
}

pub enum BidError {
    BidTooLow: (),
    WrongAssetSent: (),
}

pub enum SetupError {
    AuctionCannotEndBeforeItStarts: (),
    AuctionCannotEndInThePast: (),
    AuctionCannotStartInThePast: (),
    EndPriceCannotBeLargerThanStartPrice: (),
}

pub enum TechnicalError {
    ContractNotYetInitialized: (),
    InvalidAuctionID: (),
}

pub enum TimeError {
    AuctionAlreadyEnded: (),
    AuctionNotYetStarted: (),
}
