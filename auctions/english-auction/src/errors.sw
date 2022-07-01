library errors;

pub enum AccessError {
    AuctionDoesNotExist: (),
    AuctionIsNotClosed: (),
    AuctionIsNotOpen: (),
    NFTTransferNotApproved: (),
    NoReserveSet: (),
    SenderIsNotSeller: (),
}

pub enum AssetError {
    AssetsAreNotTheSame: (),
}

pub enum InitError {
    AuctionTimeNotProvided: (),
    BuyAssetNotProvided: (),
    CannotReinitialize: (),
    ReserveLessThanInitialPrice: (),
}

pub enum InputError {
    InitialPriceNotMet: (),
    IncorrectAssetProvided: (),
    IncorrectAmountProvided: (),
}

pub enum UserError {
    BidderIsSeller: (),
    UserHasAlreadyWithdrawn: (),
}
