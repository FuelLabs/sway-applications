use fuels::{
    prelude::*,
    tx::{ContractId},
};

// Load abi from json
abigen!(DaoVoting, "out/debug/dao-voting-abi.json");
abigen!(
    GovToken,
    "tests/artifacts/gov_token/out/debug/gov_token-abi.json"
);

pub struct Metadata {
    pub dao_voting: DaoVoting,
    pub gov_token: Option<GovToken>,
    pub wallet: LocalWallet,
}

pub mod test_helpers {
    use super::*;

    pub async fn setup() -> (GovToken, ContractId, Metadata, Metadata, u64) {
        let num_wallets = 2;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;
        let config = WalletsConfig::new(
            Some(num_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );
    
        let mut wallets = launch_provider_and_get_wallets(config).await;
        let deployer_wallet = wallets.pop().unwrap();
        let user_wallet = wallets.pop().unwrap();
    
        let dao_voting_id = Contract::deploy(
            "./out/debug/dao-voting.bin",
            &deployer_wallet,
            TxParameters::default(),
        )
        .await
        .unwrap();
    
        let gov_token_id = Contract::deploy(
            "./tests/artifacts/gov_token/out/debug/gov_token.bin",
            &deployer_wallet,
            TxParameters::default(),
        )
        .await
        .unwrap();
    
        let gov_token = GovToken::new(gov_token_id.to_string(), deployer_wallet.clone());
    
        let deployer = Metadata {
            dao_voting: DaoVoting::new(dao_voting_id.to_string(), deployer_wallet.clone()),
            gov_token: Some(GovToken::new(
                gov_token_id.to_string(),
                deployer_wallet.clone(),
            )),
            wallet: deployer_wallet,
        };
    
        let user = Metadata {
            dao_voting: DaoVoting::new(dao_voting_id.to_string(), user_wallet.clone()),
            gov_token: None,
            wallet: user_wallet,
        };
    
        let asset_amount: u64 = 10;
    
        (gov_token, gov_token_id, deployer, user, asset_amount)
    }
    
    pub fn get_call_data(asset_id: ContractId) -> daovoting_mod::Proposal {
        let call_data = daovoting_mod::CallData {
            id: asset_id,
            function_selector: 0,
            arguments: 0,
        };
    
        let proposal = daovoting_mod::Proposal {
            call_data: call_data,
            amount: 0,
            asset: asset_id,
            gas: 20000,
        };
    
        proposal
    }
}
