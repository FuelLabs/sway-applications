library errors;

pub enum AccessError {
    AuctionDoesNotExist: (),
    AuctionIsNotClosed: (),
    AuctionIsNotOpen: (),
    NoReserveSet: (),
}

pub enum InitError {
    AuctionTimeNotProvided: (),
    BuyAssetNotProvided: (),
    CannotReinitialize: (),
    ReserveLessThanInitalPrice: (),
}

pub enum InputError {
    InitalPriceNotMet: (),
    IncorrectAssetProvided: (),
    IncorrectAmountProvided: (),
}

pub enum UserError {
    BidderIsSeller: (),
    UserHasAlreadyWithdrawn: (),
}
