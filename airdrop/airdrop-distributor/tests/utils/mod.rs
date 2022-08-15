use fuels::{contract::contract::CallResponse, prelude::*};

// Load abi from json
abigen!(AirdropDistributor, "out/debug/airdrop-distributor-abi.json");
abigen!(SimpleToken, "../simple-token/out/debug/simple-token-abi.json");

pub struct Metadata {
    airdrop_distributor: AirdropDistributor,
    asset: Option<SimpleToken>,
    wallet: LocalWallet,
}

pub mod test_helpers{

    use super::*;

    pub async fn setup() -> (Metadata, Metadata, Metadata, Metadata, Metadata, Metadata) {
        let num_wallets = 6;
        let coins_per_wallet = 1;
        let coin_amount = 1000000;
        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(
                Some(num_wallets),
                Some(coins_per_wallet),
                Some(coin_amount),
            ),
            None,
        )
        .await;

        let wallet1 = wallets.pop().unwrap();
        let wallet2 = wallets.pop().unwrap();
        let wallet3 = wallets.pop().unwrap();
        let wallet4 = wallets.pop().unwrap();
        let wallet5 = wallets.pop().unwrap();
        let wallet6 = wallets.pop().unwrap();
    
        let airdrop_distributor_id = Contract::deploy(
            "./out/debug/airdrop-distributor.bin",
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                "./out/debug/airdrop-distributor-storage_slots.json".to_string(),
            )),
        )
        .await
        .unwrap();

        let simple_token_id = Contract::deploy(
            "../simple-token/out/debug/simple-token.bin",
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        let deployer = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet1.clone()).build(),
            asset: Some(SimpleTokenBuilder::new(simple_token_id.to_string(), wallet1.clone()).build()),
            wallet: wallet1.clone()
        };

        let user1 = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet2.clone()).build(),
            asset: Some(SimpleTokenBuilder::new(simple_token_id.to_string(), wallet2.clone()).build()),
            wallet: wallet2.clone()
        };

        let user2 = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet3.clone()).build(),
            asset: Some(SimpleTokenBuilder::new(simple_token_id.to_string(), wallet3.clone()).build()),
            wallet: wallet3.clone()
        };

        let user3 = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet4.clone()).build(),
            asset: Some(SimpleTokenBuilder::new(simple_token_id.to_string(), wallet4.clone()).build()),
            wallet: wallet4.clone()
        };

        let user4 = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet5.clone()).build(),
            asset: Some(SimpleTokenBuilder::new(simple_token_id.to_string(), wallet5.clone()).build()),
            wallet: wallet5.clone()
        };

        let user5 = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet6.clone()).build(),
            asset: Some(SimpleTokenBuilder::new(simple_token_id.to_string(), wallet6.clone()).build()),
            wallet: wallet6.clone()
        };

        (deployer, user1, user2, user3, user4, user5)
    }
}

pub mod airdrop_distributor_abi_calls {

    use super::*;

    pub async fn claim(
        amount: u64, 
        contract: &AirdropDistributor, 
        proof: Vec<[u8; 32]>, 
        to: Identity
    ) -> CallResponse<()> {
        contract.claim(amount, proof, to).call().await.unwrap()
    }

    pub async fn constructor(
        claim_time: u64, 
        contract: &AirdropDistributor, 
        merkle_root: [u8; 32], 
        token: ContractId
    ) -> CallResponse<()> {
        contract.constructor(claim_time, merkle_root, token).call().await.unwrap()
    }
}

pub mod simple_token_abi_calls {

    use super::*;

    pub async fn constructor(
        airdrop_contract: ContractId, 
        contract: &SimpleToken,
        token_supply: u64
    ) -> CallResponse<()> {
        contract.constructor(airdrop_contract, token_supply).call().await.unwrap()
    }
}
