use fuels::{
    prelude::*,
    signers::fuel_crypto::{Message, SecretKey, Signature},
    tx::{Bytes32, Bytes64},
    types::{Bits256, Identity, B512},
};

use rand::{rngs::StdRng, Rng, SeedableRng};
use sha3::{Digest, Keccak256};

abigen!(
    Contract(
        name = "MultiSig",
        abi = "./project/multisig-contract/out/debug/multisig-contract-abi.json"
    ),
    Contract(
        name = "TestContract",
        abi = "./project/multisig-contract/tests/artifacts/callable-contract/out/debug/callable-contract-abi.json"
    )
);

// Test-only private key
pub const VALID_SIGNER_PK: &str =
    "862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301";

pub const MULTISIG_CONTRACT_BINARY_PATH: &str = "./out/debug/multisig-contract.bin";
pub const MULTISIG_CONTRACT_STORAGE_PATH: &str = "./out/debug/multisig-contract-storage_slots.json";
pub const TEST_CONTRACT_BINARY_PATH: &str =
    "./tests/artifacts/callable-contract/out/debug/callable-contract.bin";
pub const TEST_CONTRACT_STORAGE_PATH: &str =
    "./tests/artifacts/callable-contract/out/debug/callable-contract-storage_slots.json";

pub const DEFAULT_THRESHOLD: u64 = 5;
pub const DEFAULT_TRANSFER_AMOUNT: u64 = 200;

pub struct Caller {
    pub contract: MultiSig,
    pub wallet: WalletUnlocked,
}

pub struct Transaction {
    pub contract_identifier: ContractId,
    pub nonce: u64,
    pub value: Option<u64>,
    pub asset_id: Option<ContractId>,
    pub target: Identity,
    pub function_selector: Option<Vec<u8>>,
    pub calldata: Option<Vec<u8>>,
    pub single_value_type_arg: Option<bool>,
    pub forwarded_gas: Option<u64>,
}

pub fn base_asset_contract_id() -> ContractId {
    ContractId::new(BASE_ASSET_ID.try_into().unwrap())
}

pub fn default_users() -> Vec<User> {
    let fuel_user_1 = User {
        address: Bits256::from_hex_str(
            "0xe10f526b192593793b7a1559a391445faba82a1d669e3eb2dcd17f9c121b24b1",
        )
        .unwrap(),
        weight: 3,
    };
    let evm_user_1 = User {
        address: Bits256::from_hex_str(
            "0x000000000000000000000000db4aa29ef306fc8d28025b838ccd3feecaedb333",
        )
        .unwrap(),
        weight: 2,
    };
    vec![fuel_user_1, evm_user_1]
}

fn eip_191_personal_sign_format(message_hash: Message) -> Message {
    let initial_byte = 0x19u8;
    let version_byte = 0x45u8;

    let mut eip_191_data: Vec<u8> = vec![initial_byte, version_byte];
    eip_191_data.append(&mut message_hash.to_vec());

    let eip_191_formatted_message = keccak_hash(&eip_191_data);
    unsafe { Message::from_bytes_unchecked(*eip_191_formatted_message) } // TODO: Remove use of unsafe when feature is available: https://github.com/FuelLabs/fuels-rs/issues/698
}

fn ethereum_prefix(formatted_message: Message) -> Message {
    let prefix = r#"\x19Ethereum Signed Message:\n32"#;

    let mut eth_prefix_data: Vec<u8> = Vec::new();
    eth_prefix_data.append(&mut prefix.as_bytes().to_vec());
    eth_prefix_data.append(&mut formatted_message.to_vec());

    let eth_prefixed_message = keccak_hash(eth_prefix_data);
    unsafe { Message::from_bytes_unchecked(*eth_prefixed_message) } // TODO: Remove use of unsafe when feature is available: https://github.com/FuelLabs/fuels-rs/issues/698
}

pub async fn format_and_sign(
    private_key: SecretKey,
    message_hash: Message,
    message_format: MessageFormat,
    message_prefix: MessagePrefix,
    wallet_type: WalletType,
) -> SignatureInfo {
    let formatted_message = match message_format {
        MessageFormat::None => message_hash,
        MessageFormat::EIP191PersonalSign => eip_191_personal_sign_format(message_hash),
    };

    let prefixed_message = match message_prefix {
        MessagePrefix::None => formatted_message,
        MessagePrefix::Ethereum => ethereum_prefix(formatted_message),
    };

    let signature = Signature::sign(&private_key, &prefixed_message);

    let signature_bytes: Bytes64 = Bytes64::try_from(signature).unwrap();

    let signature = B512::from((
        Bits256(signature_bytes[..32].try_into().unwrap()),
        Bits256(signature_bytes[32..].try_into().unwrap()),
    ));

    SignatureInfo {
        message_format,
        message_prefix,
        signature,
        wallet_type,
    }
}

fn keccak_hash<B>(data: B) -> Bytes32
where
    B: AsRef<[u8]>,
{
    let mut hasher = Keccak256::new();

    hasher.update(data);

    <[u8; Bytes32::LEN]>::from(hasher.finalize()).into()
}

pub async fn setup_env(private_key: &str) -> Result<(SecretKey, Caller, Caller)> {
    let private_key: SecretKey = private_key.parse().unwrap();
    let mut deployer_wallet = WalletUnlocked::new_from_private_key(private_key, None);

    let mut non_owner_wallet = WalletUnlocked::new_random(None);

    let number_of_coins = 1;
    let amount_per_coin = 1_000_000;
    let all_coins = vec![deployer_wallet.clone(), non_owner_wallet.clone()]
        .iter()
        .flat_map(|wallet| {
            setup_single_asset_coins(
                wallet.address(),
                BASE_ASSET_ID,
                number_of_coins,
                amount_per_coin,
            )
        })
        .collect::<Vec<_>>();

    let (provider, _socket_addr) = setup_test_provider(all_coins, vec![], None, None).await;
    deployer_wallet.set_provider(provider.clone());
    non_owner_wallet.set_provider(provider);

    let multisig_contract_id = Contract::deploy(
        MULTISIG_CONTRACT_BINARY_PATH,
        &deployer_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(MULTISIG_CONTRACT_STORAGE_PATH.to_string())),
    )
    .await?;

    let deployer = Caller {
        contract: MultiSig::new(multisig_contract_id.clone(), deployer_wallet.clone()),
        wallet: deployer_wallet,
    };

    let non_owner = Caller {
        contract: MultiSig::new(multisig_contract_id, non_owner_wallet.clone()),
        wallet: non_owner_wallet,
    };

    Ok((private_key, deployer, non_owner))
}

pub async fn transfer_parameters(deployer: &Caller, nonce: u64) -> (WalletUnlocked, Transaction) {
    let receiver_wallet = WalletUnlocked::new_random(None);

    let tx = Transaction {
        contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
        nonce,
        value: Some(DEFAULT_TRANSFER_AMOUNT),
        asset_id: Some(base_asset_contract_id()),
        target: Identity::Address(receiver_wallet.address().try_into().unwrap()),
        function_selector: None,
        calldata: None,
        single_value_type_arg: None,
        forwarded_gas: None,
    };

    (receiver_wallet, tx)
}

pub async fn compute_signatures(private_key: SecretKey, tx_hash: Message) -> Vec<SignatureInfo> {
    // - Fuel signature. Fuel wallet. No format. No prefix.
    let fuel_signature = format_and_sign(
        private_key,
        tx_hash,
        MessageFormat::None,
        MessagePrefix::None,
        WalletType::Fuel,
    )
    .await;

    // - EVM signature. EVM wallet. EIP-191 personal sign format. Ethereum prefix.
    let evm_signature = format_and_sign(
        private_key,
        tx_hash,
        MessageFormat::EIP191PersonalSign,
        MessagePrefix::Ethereum,
        WalletType::EVM,
    )
    .await;

    // - Must comply with ascending signers requirement of Multisig's count_approvals
    vec![evm_signature, fuel_signature]
}

#[macro_export]
macro_rules! fn_selector {
    ( $fn_name: ident ( $($fn_arg: ty),* )  ) => {
         ::fuels::core::function_selector::resolve_fn_selector(stringify!($fn_name), &[$( <$fn_arg as ::fuels::types::traits::Parameterize>::param_type() ),*]).to_vec()
    }
}

#[macro_export]
macro_rules! calldata {
    ( $($arg: expr),* ) => {
        ::fuels::core::abi_encoder::ABIEncoder::encode(&[$(::fuels::types::traits::Tokenizable::into_token($arg)),*]).unwrap().resolve(0)
    }
}

pub async fn deploy_test_contract(deployer_wallet: WalletUnlocked) -> Result<TestContract> {
    let test_contract_id = Contract::deploy(
        TEST_CONTRACT_BINARY_PATH,
        &deployer_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(TEST_CONTRACT_STORAGE_PATH.to_string())),
    )
    .await?;

    let test_contract = TestContract::new(test_contract_id, deployer_wallet);

    Ok(test_contract)
}
