use fuel_merkle::{
    binary::in_memory::MerkleTree,
    common::{Bytes32, ProofSet},
};
use fuels::{contract::contract::CallResponse, prelude::*};
use sha2::{Digest, Sha256};

// Load abi from json
abigen!(AirdropDistributor, "out/debug/airdrop-distributor-abi.json");
abigen!(SimpleToken, "../simple-token/out/debug/simpletoken-abi.json");

pub struct Asset {
    pub asset_id: ContractId,
    pub token: SimpleToken,
}

pub struct Metadata {
    pub airdrop_distributor: AirdropDistributor,
    pub contract_id: ContractId,
    pub wallet: LocalWallet,
}

pub mod airdrop_distributor_abi_calls {

    use super::*;

    pub async fn claim(
        amount: u64, 
        contract: &AirdropDistributor, 
        key: u64,
        num_leaves: u64,
        proof: Vec<[u8; 32]>, 
        to: Identity,
        token_id: ContractId
    ) -> CallResponse<()> {
        contract
            .claim(amount, key, num_leaves, proof, to)
            .append_variable_outputs(1)
            .set_contracts(&[token_id.into()])
            .call()
            .await
            .unwrap()
    }

    pub async fn airdrop_constructor(
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

    pub async fn token_constructor(
        minter: simpletoken_mod::Identity, 
        contract: &SimpleToken,
        token_supply: u64
    ) -> CallResponse<()> {
        contract.constructor(minter, token_supply).call().await.unwrap()
    }
}

pub mod test_helpers {

    use super::*;

    pub async fn build_tree(
        leaves: Vec<&(u64, Bytes32)>,
        key: u64,
    ) -> (MerkleTree, Bytes32, Bytes32, ProofSet) {
        let mut tree = MerkleTree::new();

        for datum in leaves.iter() {
            let mut bytes: Vec<u8> = Vec::new();
            bytes.push(datum.0.try_into().unwrap());
            bytes.extend_from_slice(&datum.1);

            let mut hasher = Sha256::new();
            hasher.update(bytes.as_slice());

            let digest: [u8; 32] = hasher.finalize().try_into().unwrap();
            tree.push(&digest);
        }

        let merkle_root = tree.root();
        let mut proof = tree.prove(key).unwrap();
        let merkle_leaf = proof.1[0];
        proof.1.remove(0);

        (tree, merkle_root, merkle_leaf, proof.1)
    }

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
            "../simple-token/out/debug/simpletoken.bin",
            &wallet1,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                "../simple-token/out/debug/simpletoken-storage_slots.json".to_string(),
            )),
        )
        .await
        .unwrap();

        let deployer = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet1.clone()).build(),
            contract_id: ContractId::new(*airdrop_distributor_id.hash()),
            wallet: wallet1.clone()
        };

        let user1 = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet2.clone()).build(),
            contract_id: ContractId::new(*airdrop_distributor_id.hash()),
            wallet: wallet2.clone()
        };

        let user2 = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet3.clone()).build(),
            contract_id: ContractId::new(*airdrop_distributor_id.hash()),
            wallet: wallet3.clone()
        };

        let user3 = Metadata {
            airdrop_distributor: AirdropDistributorBuilder::new(airdrop_distributor_id.to_string(), wallet4.clone()).build(),
            contract_id: ContractId::new(*airdrop_distributor_id.hash()),
            wallet: wallet4.clone()
        };

        let asset = Asset {
            asset_id: ContractId::new(*simple_token_id.hash()),
            token: SimpleTokenBuilder::new(simple_token_id.to_string(), wallet1.clone()).build(),
        };

        (deployer, user1, user2, user3, asset)
    }
}
