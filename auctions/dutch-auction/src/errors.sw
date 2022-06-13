library errors;

pub enum SetupError {
    AuctionCannotEndBeforeItStarts: (),
    AuctionCannotEndInThePast: (),
    AuctionCannotStartInThePast: (),
    EndPriceCannotBeLargerThanStartPrice: (),
}

pub enum UserError {
    InvalidAuctionID: (),
    BidTooLow: (),
    WrongAssetSent: (),
    SenderNotBeneficiary: (),
}

pub enum TimeError {
    AuctionAlreadyEnded: (),
    AuctionNotYetStarted: (),
}
