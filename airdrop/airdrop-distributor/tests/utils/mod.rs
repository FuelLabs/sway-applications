use fuels::{contract::contract::CallResponse, prelude::*};

// Load abi from json
abigen!(AirdropDistributor, "out/debug/airdrop-distributor-abi.json");
abigen!(SimpleToken, "../simple-token/out/debug/simple-token-abi.json");

pub struct Asset {
    pub token: SimpleToken,
    pub asset_id: ContractId,
}

pub struct Metadata {
    pub airdrop_distributor: AirdropDistributor,
    pub wallet: LocalWallet,
}

pub mod test_helpers{

    use super::*;

    pub async fn setup() -> (Metadata, Metadata, Metadata, Metadata, Asset) {
        let num_wallets = 4;
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
            wallet: wallet1.clone()
        };

        let user1 = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet2.clone()).build(),
            wallet: wallet2.clone()
        };

        let user2 = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet3.clone()).build(),
            wallet: wallet3.clone()
        };

        let user3 = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet4.clone()).build(),
            wallet: wallet4.clone()
        };

        let asset = Asset {
            token: SimpleTokenBuilder::new(simple_token_id.to_string(), wallet1.clone()).build(),
            asset_id: ContractId::new(*simple_token_id.hash()),
        };

        (deployer, user1, user2, user3, asset)
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

    pub async fn end_block(contract: &AirdropDistributor) -> u64 {
        contract.end_block().call().await.unwrap().value
    }

    pub async fn merkle_root(contract: &AirdropDistributor) -> [u8; 32] {
        contract.merkle_root().call().await.unwrap().value
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
