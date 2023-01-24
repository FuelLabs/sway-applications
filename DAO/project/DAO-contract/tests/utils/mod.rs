use fuels::{
    prelude::*, programs::call_response::FuelCallResponse, tx::ContractId, types::Identity,
};

abigen!(
    Contract(
        name = "DaoVoting",
        abi = "./project/DAO-contract/out/debug/DAO-contract-abi.json"
    ),
    Contract(
        name = "GovToken",
        abi = "./project/DAO-contract/tests/artifacts/gov_token/out/debug/gov_token-abi.json"
    ),
);

pub struct Metadata {
    pub dao_voting: DaoVoting,
    pub gov_token: Option<GovToken>,
    pub wallet: WalletUnlocked,
}

pub mod paths {
    pub const DAO_CONTRACT_BINARY_PATH: &str = "./out/debug/DAO-contract.bin";
    pub const DAO_CONTRACT_STORAGE_PATH: &str = "./out/debug/DAO-contract-storage_slots.json";
    pub const GOVERNANCE_TOKEN_BINARY_PATH: &str =
        "./tests/artifacts/gov_token/out/debug/gov_token.bin";
    pub const GOVERNANCE_TOKEN_STORAGE_PATH: &str =
        "./tests/artifacts/gov_token/out/debug/gov_token-storage_slots.json";
}

pub mod abi_calls {

    use super::*;

    pub async fn constructor(contract: &DaoVoting, token: ContractId) -> FuelCallResponse<()> {
        contract.methods().constructor(token).call().await.unwrap()
    }

    pub async fn create_proposal(
        contract: &DaoVoting,
        acceptance_percentage: u64,
        deadline: u64,
        proposal: Proposal,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .create_proposal(acceptance_percentage, deadline, proposal)
            .call()
            .await
            .unwrap()
    }

    pub async fn deposit(
        contract: &DaoVoting,
        call_params: CallParameters,
    ) -> FuelCallResponse<()> {
        let tx_params = TxParameters::new(None, Some(1_000_000), None);
        contract
            .methods()
            .deposit()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
    }

    pub async fn withdraw(contract: &DaoVoting, amount: u64) -> FuelCallResponse<()> {
        contract
            .methods()
            .withdraw(amount)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn vote(
        contract: &DaoVoting,
        approve: bool,
        proposal_id: u64,
        vote_amount: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .vote(approve, proposal_id, vote_amount)
            .call()
            .await
            .unwrap()
    }

    pub async fn execute(contract: &DaoVoting, id: u64) -> FuelCallResponse<()> {
        contract.methods().execute(id).call().await.unwrap()
    }

    pub async fn unlock_votes(contract: &DaoVoting, id: u64) -> FuelCallResponse<()> {
        contract.methods().unlock_votes(id).call().await.unwrap()
    }

    pub async fn balance(contract: &DaoVoting) -> u64 {
        contract.methods().balance().call().await.unwrap().value
    }

    pub async fn user_balance(contract: &DaoVoting, user_identity: &Bech32Address) -> u64 {
        contract
            .methods()
            .user_balance(Identity::Address(user_identity.into()))
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn user_votes(contract: &DaoVoting, user_identity: &Bech32Address, id: u64) -> Votes {
        contract
            .methods()
            .user_votes(id, Identity::Address(user_identity.into()))
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn proposal(contract: &DaoVoting, id: u64) -> ProposalInfo {
        contract.methods().proposal(id).call().await.unwrap().value
    }

    pub async fn governance_token_id(contract: &DaoVoting) -> ContractId {
        contract
            .methods()
            .governance_token_id()
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn proposal_count(contract: &DaoVoting) -> u64 {
        contract
            .methods()
            .proposal_count()
            .call()
            .await
            .unwrap()
            .value
    }
}

pub mod test_helpers {

    use super::*;
    use paths::{
        DAO_CONTRACT_BINARY_PATH, DAO_CONTRACT_STORAGE_PATH, GOVERNANCE_TOKEN_BINARY_PATH,
        GOVERNANCE_TOKEN_STORAGE_PATH,
    };

    pub async fn mint(contract: &GovToken, amount: u64, address: &Bech32Address) -> bool {
        contract
            .methods()
            .mint_and_send_to_address(amount, address.into())
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
            .value
    }

    pub fn proposal_transaction(asset_id: ContractId) -> Proposal {
        let call_data = CallData {
            id: asset_id,
            function_selector: 0,
            arguments: 0,
        };

        Proposal {
            call_data,
            amount: 0,
            asset: asset_id,
            gas: 20000,
        }
    }

    pub async fn setup() -> (GovToken, ContractId, Metadata, Metadata, u64) {
        let num_wallets = 2;
        let coins_per_wallet = 1;
        let amount_per_coin = 1_000_000;
        let config = WalletsConfig::new(
            Some(num_wallets),
            Some(coins_per_wallet),
            Some(amount_per_coin),
        );

        let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;
        let deployer_wallet = wallets.pop().unwrap();
        let user_wallet = wallets.pop().unwrap();

        let dao_voting_id = Contract::deploy(
            DAO_CONTRACT_BINARY_PATH,
            &deployer_wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(DAO_CONTRACT_STORAGE_PATH.to_string())),
        )
        .await
        .unwrap();

        let gov_token_id = Contract::deploy(
            GOVERNANCE_TOKEN_BINARY_PATH,
            &deployer_wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                GOVERNANCE_TOKEN_STORAGE_PATH.to_string(),
            )),
        )
        .await
        .unwrap();

        let gov_token = GovToken::new(gov_token_id.clone(), deployer_wallet.clone());

        let deployer = Metadata {
            dao_voting: DaoVoting::new(dao_voting_id.clone(), deployer_wallet.clone()),
            gov_token: Some(GovToken::new(gov_token_id.clone(), deployer_wallet.clone())),
            wallet: deployer_wallet,
        };

        let user = Metadata {
            dao_voting: DaoVoting::new(dao_voting_id, user_wallet.clone()),
            gov_token: None,
            wallet: user_wallet,
        };

        let asset_amount: u64 = 10;

        (gov_token, gov_token_id.into(), deployer, user, asset_amount)
    }
}
