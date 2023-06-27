use fuel_merkle::{
    binary::in_memory::MerkleTree,
    common::{empty_sum_sha256, Bytes32},
};
use fuels::{
    accounts::ViewOnlyAccount,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, AssetId, Config, Contract, ContractId,
        LoadConfiguration, StorageConfiguration, TxParameters, WalletUnlocked, WalletsConfig,
    },
    types::{Bits256, Identity},
};
use sha2::{Digest, Sha256};

pub const NODE: u8 = 0x01;
pub const LEAF: u8 = 0x00;

abigen!(
    Contract(
        name = "AirdropDistributor",
        abi = "./contracts/distributor-contract/out/debug/distributor-contract-abi.json"
    ),
    Contract(
        name = "SimpleAsset",
        abi = "./contracts/asset-contract/out/debug/asset-contract-abi.json"
    ),
);

pub(crate) struct Asset {
    pub(crate) asset: SimpleAsset<WalletUnlocked>,
    pub(crate) asset_id: ContractId,
}

pub(crate) struct Metadata {
    pub(crate) airdrop_distributor: AirdropDistributor<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

#[derive(Clone)]
struct Node {
    hash: Bytes32,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn new(hash: Bytes32) -> Self {
        Node {
            hash,
            left: None,
            right: None,
        }
    }

    fn left(mut self, node: usize) -> Self {
        self.left = Some(node);
        self
    }

    fn right(mut self, node: usize) -> Self {
        self.right = Some(node);
        self
    }
}

const ASSET_CONTRACT_BINARY_PATH: &str = "../asset-contract/out/debug/asset-contract.bin";
const ASSET_CONTRACT_STORAGE_PATH: &str =
    "../asset-contract/out/debug/asset-contract-storage_slots.json";
const DISTRIBUTOR_CONTRACT_BINARY_PATH: &str = "./out/debug/distributor-contract.bin";
const DISTRIBUTOR_CONTRACT_STORAGE_PATH: &str =
    "./out/debug/distributor-contract-storage_slots.json";

pub(crate) async fn build_tree(
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

pub(crate) async fn build_tree_manual(
    leaves: Vec<(Identity, u64)>,
    height: u64,
    key: u64,
) -> (Bits256, Vec<Bits256>, Bits256) {
    let num_leaves = leaves.len();
    let mut nodes: Vec<Node> = Vec::new();
    let mut leaf_hash: Bytes32 = *empty_sum_sha256();
    let mut proof: Vec<Bits256> = Vec::new();

    assert!(key <= num_leaves as u64);

    // Hash leaves and create leaf nodes
    for (n, leaf) in leaves.iter().enumerate().take(num_leaves) {
        let mut hasher = Sha256::new();

        let identity = leaf.0.clone();
        match identity {
            Identity::Address(identity_a) => {
                hasher.update([0, 0, 0, 0, 0, 0, 0, 0]);
                hasher.update(*identity_a);
            }
            Identity::ContractId(identity_c) => {
                hasher.update([0, 0, 0, 0, 0, 0, 0, 1]);
                hasher.update(*identity_c);
            }
        }
        hasher.update(leaf.1.to_be_bytes());
        let hash_leaf_data: Bytes32 = hasher.finalize().try_into().unwrap();

        let mut hasher2 = Sha256::new();
        hasher2.update([LEAF]);
        hasher2.update(hash_leaf_data);
        let hash2_leaf: Bytes32 = hasher2.finalize().try_into().unwrap();

        let new_node = Node::new(hash2_leaf);
        nodes.push(new_node);
        if n as u64 == key {
            leaf_hash = hash2_leaf;
        }
    }

    let mut iterator = 0;
    // Build tree
    for i in 0..height {
        let current_num_leaves = iterator + 2usize.pow((height - i).try_into().unwrap());

        // Create new depth
        while iterator < current_num_leaves {
            let mut hasher = Sha256::new();
            hasher.update([NODE]);
            hasher.update(nodes[iterator].hash);
            hasher.update(nodes[iterator + 1].hash);
            let hash: Bytes32 = hasher.finalize().try_into().unwrap();

            let new_node = Node::new(hash).left(iterator).right(iterator + 1);
            nodes.push(new_node);
            iterator += 2;
        }
    }

    // Get proof
    let mut key = key;
    let mut index = nodes.len() - 1;
    for i in 0..height as usize {
        let node = nodes[index].clone();

        if node.left.is_none() && node.right.is_none() {
            break;
        }

        let number_subtree_elements =
            (2usize.pow(((height as usize - i) + 1).try_into().unwrap())) / 2;

        if key <= number_subtree_elements as u64 {
            // Go left
            index = node.left.unwrap();
            let proof_node = node.right.unwrap();
            proof.push(Bits256(nodes[proof_node].hash));
        } else {
            // Go right
            index = node.right.unwrap();
            let proof_node = node.left.unwrap();
            proof.push(Bits256(nodes[proof_node].hash));

            key -= number_subtree_elements as u64;
        }
    }

    proof.reverse();

    (
        Bits256(leaf_hash),
        proof,
        Bits256(nodes.last().unwrap().hash),
    )
}

pub(crate) async fn defaults(
    deploy_wallet: &Metadata,
    wallet1: &Metadata,
    wallet2: &Metadata,
    wallet3: &Metadata,
) -> (
    Identity,
    Identity,
    Identity,
    Identity,
    u64,
    u64,
    u64,
    Vec<(Identity, u64)>,
    u64,
    u64,
) {
    let identity_a = Identity::Address(wallet1.wallet.address().into());
    let identity_b = Identity::Address(wallet2.wallet.address().into());
    let identity_c = Identity::Address(wallet3.wallet.address().into());
    let minter = Identity::Address(deploy_wallet.wallet.address().into());
    let key = 0;
    let asset_supply = 10;
    let claim_time = 15;
    let depth = 8;

    let identity_vec = vec![identity_a.clone(), identity_b.clone(), identity_c.clone()];

    let airdrop_leaves = leaves_with_depth(depth, identity_vec.clone()).await;

    (
        identity_a,
        identity_b,
        identity_c,
        minter,
        key,
        airdrop_leaves.len().try_into().unwrap(),
        asset_supply,
        airdrop_leaves,
        claim_time,
        depth,
    )
}

pub(crate) async fn leaves_with_depth(
    depth: u64,
    identities: Vec<Identity>,
) -> Vec<(Identity, u64)> {
    let num_elements_in_tree = 2_i64.pow(depth.try_into().unwrap());
    let num_identities = identities.len();
    let mut return_vec: Vec<(Identity, u64)> = Vec::new();

    for n in 0..num_elements_in_tree {
        let n_u64: u64 = (n % i64::MAX).try_into().unwrap();

        return_vec.push((identities[n as usize % num_identities].clone(), n_u64 + 1));
    }

    return_vec
}

pub(crate) async fn setup() -> (Metadata, Metadata, Metadata, Metadata, Asset) {
    let number_of_wallets = 4;
    let coins_per_wallet = 1;
    let coin_amount = 1000000;
    let config = Config {
        manual_blocks_enabled: true, // Necessary so the `produce_blocks` API can be used locally
        ..Config::local_node()
    };
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(number_of_wallets),
            Some(coins_per_wallet),
            Some(coin_amount),
        ),
        Some(config),
        None,
    )
    .await;

    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();
    let wallet3 = wallets.pop().unwrap();
    let wallet4 = wallets.pop().unwrap();

    let airdrop_distributor_storage_configuration =
        StorageConfiguration::load_from(DISTRIBUTOR_CONTRACT_STORAGE_PATH);
    let airdrop_distributor_configuration = LoadConfiguration::default()
        .set_storage_configuration(airdrop_distributor_storage_configuration.unwrap());

    let simple_asset_storage_configuration =
        StorageConfiguration::load_from(ASSET_CONTRACT_STORAGE_PATH);
    let simple_asset_configuration = LoadConfiguration::default()
        .set_storage_configuration(simple_asset_storage_configuration.unwrap());

    let airdrop_distributor_id = Contract::load_from(
        DISTRIBUTOR_CONTRACT_BINARY_PATH,
        airdrop_distributor_configuration,
    )
    .unwrap()
    .deploy(&wallet1, TxParameters::default())
    .await
    .unwrap();

    let simple_asset_id =
        Contract::load_from(ASSET_CONTRACT_BINARY_PATH, simple_asset_configuration)
            .unwrap()
            .deploy(&wallet1, TxParameters::default())
            .await
            .unwrap();

    let deployer = Metadata {
        airdrop_distributor: AirdropDistributor::new(
            airdrop_distributor_id.clone(),
            wallet1.clone(),
        ),
        wallet: wallet1.clone(),
    };

    let user1 = Metadata {
        airdrop_distributor: AirdropDistributor::new(
            airdrop_distributor_id.clone(),
            wallet2.clone(),
        ),
        wallet: wallet2,
    };

    let user2 = Metadata {
        airdrop_distributor: AirdropDistributor::new(
            airdrop_distributor_id.clone(),
            wallet3.clone(),
        ),
        wallet: wallet3,
    };

    let user3 = Metadata {
        airdrop_distributor: AirdropDistributor::new(
            airdrop_distributor_id.clone(),
            wallet4.clone(),
        ),
        wallet: wallet4,
    };

    let asset = Asset {
        asset: SimpleAsset::new(simple_asset_id.clone(), wallet1),
        asset_id: ContractId::new(*simple_asset_id.hash()),
    };

    (deployer, user1, user2, user3, asset)
}

pub(crate) async fn get_wallet_balance(wallet: &WalletUnlocked, asset: &AssetId) -> u64 {
    wallet.get_asset_balance(asset).await.unwrap()
}
