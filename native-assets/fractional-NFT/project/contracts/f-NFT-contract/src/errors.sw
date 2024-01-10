library;

pub enum SubIdError {
    InvalidSubId: (),
}

pub enum DepositError {
    InvalidSRC20NFT: (),
}

pub enum WithdrawError {
    AllSharesNotReturned: (),
    InvalidAsset: (),
}
