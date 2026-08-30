library;

pub enum MintError {
    CannotMintMoreThanOneNFTWithSubId: (),
    MaxNFTsMinted: (),
    NFTAlreadyMinted: (),
    SubIdNone: (),
}

pub enum SetError {
    ValueAlreadySet: (),
}
