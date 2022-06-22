library errors;

pub enum SetupError {
    AuctionCannotEndBeforeItStarts: (),
    AuctionCannotEndInThePast: (),
    AuctionCannotStartInThePast: (),
    EndPriceCannotBeLargerThanStartPrice: (),
}

pub enum TimeError {
    AuctionAlreadyEnded: (),
    AuctionNotYetStarted: (),
}

pub enum UserError {
    InvalidAuctionID: (),
    BidTooLow: (),
    WrongAssetSent: (),
    SenderNotAuthor: (),
}
