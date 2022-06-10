library errors;

pub enum Error {
    ContractNotYetInitialized: (),
    CannotReinitialize: (),
    SenderNotAdmin: (),
    AuctionInProgress: (),
    AuctionAlreadyEnded: (),
    BidTooLow: (),
    WrongAssetSent: (),
    EndPriceCannotBeLargerThanStartPrice: (),
    AuctionCannotEndInThePast: (),
    AuctionCannotStartInThePast: (),
    AuctionCannotEndBeforeItStarts: (),
    AuctionNotYetStarted: (),
    InvalidAuctionID: (),
}
