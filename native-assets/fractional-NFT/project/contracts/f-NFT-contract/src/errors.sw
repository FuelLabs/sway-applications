library;

pub enum AccessError {
    NotVaultAdmin: (),
}

pub enum BuybackError {
    InvalidAsset: (),
    NotEnoughTokens: (),
}

pub enum DepositError {
    InvalidAsset: (),
    NotEnoughTokensAvailable: (),
}

pub enum StateError {
    InvalidState: (),
}

pub enum VaultCreationError {
    InvalidContractOrSubId: (),
    InvalidSRC20NFT: (),
}

pub enum WithdrawError {
    AllSharesNotReturned: (),
    InvalidAsset: (),
}
