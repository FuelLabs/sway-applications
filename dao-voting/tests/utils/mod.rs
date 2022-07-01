use fuels::{prelude::*, tx::ContractId};

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

pub mod abi_calls {
    use fuels::contract::contract::CallResponse;

    use super::*;

    pub async fn constructor(contract: &DaoVoting, token: ContractId) -> CallResponse<()> {
        contract
            .constructor(token)
            .call()
            .await
            .unwrap()
    }

    pub async fn create_proposal(
        contract: &DaoVoting,
        acceptance_percentage: u64,
        deadline: u64,
        proposal: Proposal,
    ) -> CallResponse<()> {
        contract
            .create_proposal(acceptance_percentage, deadline, proposal)
            .call()
            .await
            .unwrap()
    }

    pub async fn deposit(contract: &DaoVoting, tx_params: TxParameters, call_params: CallParameters) -> CallResponse<()> {
        contract
            .deposit()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
    }

    pub async fn withdraw(contract: &DaoVoting, amount: u64) -> CallResponse<()> {
        contract
            .withdraw(amount)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn vote(contract: &DaoVoting, approve: bool, proposal_id: u64, vote_amount: u64) -> CallResponse<()> {
        contract
            .vote(approve, proposal_id, vote_amount)
            .call()
            .await
            .unwrap()
    }

    pub async fn execute(contract: &DaoVoting, id: u64) -> CallResponse<()> {
        contract.execute(id).call().await.unwrap()
    }

    pub async fn unlock_votes(contract: &DaoVoting, id: u64) -> CallResponse<()> {
        contract.unlock_votes(id).call().await.unwrap()
    }

    pub async fn balance(contract: &DaoVoting) -> u64 {
        contract.balance().call().await.unwrap().value
    }

    pub async fn user_balance(contract: &DaoVoting, user_identity: Identity) -> u64 {
        contract
            .user_balance(user_identity)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn user_votes(contract: &DaoVoting, user_identity: Identity, id: u64) -> u64 {
        contract
            .user_votes(id, user_identity)
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn proposal(contract: &DaoVoting, id: u64) -> ProposalInfo {
        contract.proposal(id).call().await.unwrap().value
    }

    pub async fn governance_token_id(contract: &DaoVoting) -> ContractId {
        contract
            .governance_token_id()
            .call()
            .await
            .unwrap()
            .value
    }

    pub async fn proposal_count(contract: &DaoVoting) -> u64 {
        contract.proposal_count().call().await.unwrap().value
    }
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

    pub async fn mint(contract: &GovToken, amount: u64, address: Address) -> bool {
        contract
            .mint_and_send_to_address(amount, address)
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

        let proposal = Proposal {
            call_data: call_data,
            amount: 0,
            asset: asset_id,
            gas: 20000,
        };

        proposal
    }
}
