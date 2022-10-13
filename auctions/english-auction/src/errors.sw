library errors;

pub enum AccessError {
    AuctionIsNotClosed: (),
    AuctionIsNotOpen: (),
    NFTTransferNotApproved: (),
    SenderIsNotSeller: (),
}

pub enum AssetError {
    AssetsAreNotTheSame: (),
}

pub enum InitError {
    AuctionDurationNotProvided: (),
    CannotAcceptMoreThanOneNFT: (),
    ReserveLessThanInitialPrice: (),
}

pub enum InputError {
    AuctionDoesNotExist: (),
    InitialPriceNotMet: (),
    IncorrectAmountProvided: (),
    IncorrectAssetProvided: (),
}

pub enum UserError {
    BidderIsSeller: (),
    UserHasAlreadyWithdrawn: (),
}
