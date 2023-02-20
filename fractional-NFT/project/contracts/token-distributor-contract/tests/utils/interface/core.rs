use crate::utils::setup::{Asset, FractionalNFT, Nft, TokenDistributor};
use fuels::{
    prelude::{
        Address, Bech32ContractId, CallParameters, ContractId, SettableContract, TxParameters,
        WalletUnlocked,
    },
    programs::call_response::FuelCallResponse,
    tx::AssetId,
    types::Identity,
};

pub(crate) mod asset {
    use super::*;

    pub(crate) async fn mint_and_send_to_address(
        amount: u64,
        contract: &Asset,
        recipient: Address,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .mint_and_send_to_address(amount, recipient)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }
}

pub(crate) mod nft {
    use super::*;

    pub(crate) async fn approve(
        approved: Option<Identity>,
        contract: &Nft,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .approve(approved, token_id)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn mint(amount: u64, contract: &Nft, owner: Identity) -> FuelCallResponse<()> {
        contract.methods().mint(amount, owner).call().await.unwrap()
    }
}

pub(crate) mod token_distributor {
    use super::*;

    pub(crate) async fn buyback(
        amount: u64,
        contract: &TokenDistributor,
        external_asset: ContractId,
        f_nft: ContractId,
        token_price: u64,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params =
            CallParameters::new(Some(amount), Some(AssetId::from(*external_asset)), None);

        contract
            .methods()
            .buyback(f_nft, token_price)
            .tx_params(tx_params)
            .call_params(call_params)
            .set_contract_ids(&[Bech32ContractId::from(f_nft)])
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn create(
        contract: &TokenDistributor,
        external_asset: ContractId,
        f_nft: ContractId,
        nft: ContractId,
        reserve_price: Option<u64>,
        token_owner: Option<Identity>,
        token_price: u64,
        token_supply: u64,
        token_id: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .create(
                nft,
                external_asset,
                f_nft,
                reserve_price,
                token_owner,
                token_price,
                token_supply,
                token_id,
            )
            .set_contract_ids(&[Bech32ContractId::from(f_nft), Bech32ContractId::from(nft)])
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn end(
        contract: &TokenDistributor,
        wallet: &WalletUnlocked,
        f_nft: ContractId,
        nft: ContractId,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .end(f_nft)
            .set_contracts(&[
                &FractionalNFT::new(f_nft.into(), wallet.clone()) as &dyn SettableContract,
                &Nft::new(nft.into(), wallet.clone()) as &dyn SettableContract,
            ])
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn purchase(
        amount: u64,
        contract: &TokenDistributor,
        external_asset: ContractId,
        f_nft: ContractId,
        price: u64,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(
            Some(amount * price),
            Some(AssetId::from(*external_asset)),
            None,
        );

        contract
            .methods()
            .purchase(amount, f_nft)
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn purchase_admin(
        admin: Option<Identity>,
        amount: u64,
        contract: &TokenDistributor,
        external_asset: ContractId,
        f_nft: ContractId,
        reserve: Option<u64>,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params =
            CallParameters::new(Some(amount), Some(AssetId::from(*external_asset)), None);

        contract
            .methods()
            .purchase_admin(admin, f_nft, reserve)
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn sell(
        amount: u64,
        contract: &TokenDistributor,
        f_nft: ContractId,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        let call_params = CallParameters::new(Some(amount), Some(AssetId::from(*f_nft)), None);

        contract
            .methods()
            .sell(f_nft)
            .tx_params(tx_params)
            .call_params(call_params)
            .append_variable_outputs(1)
            .set_contract_ids(&[Bech32ContractId::from(f_nft)])
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn set_reserve(
        contract: &TokenDistributor,
        f_nft: ContractId,
        reserve: Option<u64>,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_reserve(f_nft, reserve)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn set_token_price(
        contract: &TokenDistributor,
        f_nft: ContractId,
        token_price: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .set_token_price(f_nft, token_price)
            .call()
            .await
            .unwrap()
    }

    pub(crate) async fn withdraw(
        contract: &TokenDistributor,
        f_nft: ContractId,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .withdraw(f_nft)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }
}
