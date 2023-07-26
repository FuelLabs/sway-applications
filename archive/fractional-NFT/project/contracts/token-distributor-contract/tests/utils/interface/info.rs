use crate::utils::setup::{FractionalNFT, NFTInfo, Nft, TokenDistribution, TokenDistributor};
use fuels::{
    prelude::{ContractId, WalletUnlocked},
    types::Identity,
};

pub(crate) mod fractional_nft {
    use super::*;

    pub(crate) async fn nft_info(contract: &FractionalNFT<WalletUnlocked>) -> Option<NFTInfo> {
        contract.methods().nft_info().call().await.unwrap().value
    }
}

pub(crate) mod nft {
    use super::*;

    pub(crate) async fn owner_of(
        contract: &Nft<WalletUnlocked>,
        token_id: u64,
    ) -> Option<Identity> {
        contract
            .methods()
            .owner_of(token_id)
            .call()
            .await
            .unwrap()
            .value
    }
}

pub(crate) mod token_distributor {
    use super::*;

    pub async fn token_distribution(
        contract: &TokenDistributor<WalletUnlocked>,
        f_nft: ContractId,
    ) -> Option<TokenDistribution> {
        contract
            .methods()
            .token_distribution(f_nft)
            .call()
            .await
            .unwrap()
            .value
    }
}
