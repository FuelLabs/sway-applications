use fuel_merkle::{binary::in_memory::MerkleTree, common::Bytes32};
use fuels::{
    accounts::{predicate::Predicate, Account},
    prelude::*,
    tx::AssetId,
    types::{Bits256, Identity},
};
use sha2::{Digest, Sha256};

// Load abi from json
abigen!(Predicate(
    name = "MyPredicate",
    abi = "out/debug/test_predicate-abi.json"
));

#[tokio::test]
async fn test_merkle_predicate() {
    let (wallet1, wallet2, asset_id) = get_wallets().await;

    // Setup Identities
    let identity1 = Identity::Address(wallet1.address().into());
    let identity2 = Identity::Address(wallet2.address().into());
    let mut vec_of_identities = Vec::new();
    vec_of_identities.push(identity1.clone());
    vec_of_identities.push(identity2.clone());

    // Create leaves
    let depth = 8;
    let key = 1;
    let leaves = leaves_with_depth(depth, vec_of_identities).await;

    // Create merkle tree
    let (_tree, root, _leaf, proof) = build_tree(key.clone(), leaves.to_vec()).await;

    // Create predicate instance and load data
    let code_path = "./out/debug/test_predicate.bin";

    let allotted_amount = leaves[key as usize].1;
    let claiming_identity = leaves[key as usize].0.clone();
    let predicate_data = MyPredicate::encode_data(
        key,
        allotted_amount,
        claiming_identity.clone(),
        proof,
        root,
        leaves.len() as u64,
    );
    let predicate: Predicate = Predicate::load_from(code_path)
        .unwrap()
        .with_data(predicate_data)
        .with_provider(wallet1.try_provider().unwrap().clone());

    // First wallet transfers 500 tokens to predicate.
    let predicate_tokens = 500;
    wallet1
        .transfer(
            predicate.address(),
            predicate_tokens,
            asset_id,
            TxParameters::default(),
        )
        .await
        .unwrap();

    // Check predicate balance that it has the 500 tokens.
    let balance = predicate
        .get_asset_balance(&AssetId::default())
        .await
        .unwrap();
    assert_eq!(balance, predicate_tokens);

    // Unlock and transfer with predicate
    predicate
        .transfer(
            wallet2.address(),
            allotted_amount,
            asset_id,
            TxParameters::default(),
        )
        .await
        .unwrap();

    // Predicate balance is whatever the other wallet was allotted.
    let balance = predicate
        .get_asset_balance(&AssetId::default())
        .await
        .unwrap();
    assert_eq!(balance, predicate_tokens - allotted_amount);

    // Second wallet balance is updated.
    let balance = wallet2
        .get_asset_balance(&AssetId::default())
        .await
        .unwrap();
    assert_eq!(balance, 1000 + allotted_amount);
}

async fn get_wallets() -> (WalletUnlocked, WalletUnlocked, AssetId) {
    let asset_id = AssetId::default();
    let wallets_config = WalletsConfig::new_multiple_assets(
        2,
        vec![AssetConfig {
            id: asset_id,
            num_coins: 1,
            coin_amount: 1_000,
        }],
    );

    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None).await;
    let first_wallet = &wallets[0];
    let second_wallet = &wallets[1];

    (first_wallet.clone(), second_wallet.clone(), asset_id)
}

pub async fn build_tree(
    key: u64,
    leaves: Vec<(Identity, u64)>,
) -> (MerkleTree, Bits256, Bytes32, Vec<Bits256>) {
    let mut tree = MerkleTree::new();

    for datum in leaves.iter() {
        let mut hasher = Sha256::new();
        let identity = datum.0.clone();

        match identity {
            Identity::Address(identity) => {
                hasher.update([0, 0, 0, 0, 0, 0, 0, 0]);
                hasher.update(*identity);
            }
            Identity::ContractId(identity) => {
                hasher.update([0, 0, 0, 0, 0, 0, 0, 1]);
                hasher.update(*identity);
            }
        }
        hasher.update(datum.1.to_be_bytes());

        let digest: [u8; 32] = hasher.finalize().try_into().unwrap();
        tree.push(&digest);
    }

    let merkle_root = tree.root();
    let mut proof = tree.prove(key).unwrap();
    let merkle_leaf = proof.1[0];
    proof.1.remove(0);

    let mut final_proof: Vec<Bits256> = Vec::new();

    for iterator in proof.1 {
        final_proof.push(Bits256(iterator));
    }

    (tree, Bits256(merkle_root), merkle_leaf, final_proof)
}

pub async fn leaves_with_depth(depth: u64, identities: Vec<Identity>) -> Vec<(Identity, u64)> {
    let num_elements_in_tree = 2_i64.pow(depth.try_into().unwrap());
    let num_identities = identities.len();
    let mut return_vec: Vec<(Identity, u64)> = Vec::new();

    for n in 0..num_elements_in_tree {
        let n_u64: u64 = (n % i64::MAX).try_into().unwrap();

        return_vec.push((identities[n as usize % num_identities].clone(), n_u64 + 1));
    }

    return_vec
}
