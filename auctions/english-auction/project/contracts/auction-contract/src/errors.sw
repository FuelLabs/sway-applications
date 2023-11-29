library;

/// Errors related to permissions.
pub enum AccessError {
    /// The auction is not yet closed.
    AuctionIsNotClosed: (),
    /// The auction is not yet open.
    AuctionIsNotOpen: (),
    /// The sender is not the auction seller.
    SenderIsNotSeller: (),
}

/// Errors related to the initialization of the auction.
pub enum InitError {
    /// The auction duration is not provided.
    AuctionDurationNotProvided: (),
    /// The initial price cannot be zero.
    InitialPriceCannotBeZero: (),
    /// The reserve price cannot be lower than the initial price.
    ReserveLessThanInitialPrice: (),
}

/// Errors related to input parameters.
pub enum InputError {
    /// The requested auction does not exist.
    AuctionDoesNotExist: (),
    /// The initial price of the auction is not met.
    InitialPriceNotMet: (),
    /// The incorrect amount of assets were provided.
    IncorrectAmountProvided: (),
    /// The incorrect asset was provided.
    IncorrectAssetProvided: (),
}

/// Errors made by users.
pub enum UserError {
    /// Sellers cannot bid on their own auctions.
    BidderIsSeller: (),
    /// The user has already withdrawn their owed assets.
    UserHasAlreadyWithdrawn: (),
}
