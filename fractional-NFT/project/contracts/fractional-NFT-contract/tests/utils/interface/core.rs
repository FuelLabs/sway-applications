use crate::utils::setup::{FractionalNFT, Nft};
use fuels::{
    prelude::{Bech32ContractId, ContractId, WalletUnlocked},
    programs::call_response::FuelCallResponse,
    types::Identity,
};

pub(crate) mod fractional_nft {

    use super::*;

    pub(crate) async fn deposit(
        admin: Option<Identity>,
        contract: &FractionalNFT<WalletUnlocked>,
        nft: ContractId,
        supply: u64,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .deposit(admin, nft, supply, token_id)
            .set_contract_ids(&[Bech32ContractId::from(nft)])
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn set_admin(
        contract: &FractionalNFT<WalletUnlocked>,
        new_admin: Option<Identity>,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_admin(new_admin)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn withdraw(
        contract: &FractionalNFT<WalletUnlocked>,
        nft: ContractId,
        to: Identity,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .withdraw(to)
            .set_contract_ids(&[Bech32ContractId::from(nft)])
            .call()
            .await
            .unwrap()
    }
}

pub(crate) mod nft {

    use super::*;

    pub(crate) async fn approve(
        approved: Option<Identity>,
        contract: &Nft<WalletUnlocked>,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .approve(approved, token_id)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn mint(
        amount: u64,
        contract: &Nft<WalletUnlocked>,
        owner: Identity,
    ) -> FuelCallResponse<()> {
        contract.methods().mint(amount, owner).call().await.unwrap()
    }
}
