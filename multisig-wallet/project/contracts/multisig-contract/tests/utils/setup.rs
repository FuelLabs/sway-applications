use fuels::{
    accounts::{
        fuel_crypto::{Message, SecretKey, Signature},
        wallet::WalletUnlocked,
    },
    core::codec::{calldata, fn_selector},
    prelude::{
        abigen, setup_single_asset_coins, setup_test_provider, Address, Contract, ContractId,
        Error, LoadConfiguration, StorageConfiguration, TxParameters, BASE_ASSET_ID,
    },
    tx::Bytes32,
    types::{Bits256, Bytes, Identity, B512},
};

use sha3::{Digest, Keccak256};

abigen!(
    Contract(
        name = "MultiSig",
        abi = "./contracts/multisig-contract/out/debug/multisig-contract-abi.json"
    ),
    Contract(
        name = "CallableContract",
        abi = "./contracts/test-artifacts/callable-contract/out/debug/callable-contract-abi.json"
    )
);

// Test-only private key
pub(crate) const VALID_SIGNER_PK: &str =
    "862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301";

const MULTISIG_CONTRACT_BINARY_PATH: &str = "./out/debug/multisig-contract.bin";
const MULTISIG_CONTRACT_STORAGE_PATH: &str = "./out/debug/multisig-contract-storage_slots.json";
const CALLABLE_CONTRACT_BINARY_PATH: &str =
    "../test-artifacts/callable-contract/out/debug/callable-contract.bin";
const CALLABLE_CONTRACT_STORAGE_PATH: &str =
    "../test-artifacts/callable-contract/out/debug/callable-contract-storage_slots.json";

pub(crate) const DEFAULT_CALLDATA_VALUE_PARAM: u64 = 1;
pub(crate) const DEFAULT_FORWARDED_GAS: u64 = 10_000_000;
pub(crate) const DEFAULT_TRANSFER_AMOUNT: u64 = 200;
pub(crate) const DEFAULT_THRESHOLD: u64 = 5;

pub(crate) struct Caller {
    pub(crate) contract: MultiSig<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) fn base_asset_contract_id() -> ContractId {
    ContractId::new(BASE_ASSET_ID.try_into().unwrap())
}

pub(crate) fn default_users() -> Vec<User> {
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

pub(crate) async fn deploy_callable_contract(
    deployer_wallet: WalletUnlocked,
) -> Result<CallableContract<WalletUnlocked>, Error> {
    let callable_contract_storage_configuration =
        StorageConfiguration::load_from(CALLABLE_CONTRACT_STORAGE_PATH);
    let callable_contract_configuration = LoadConfiguration::default()
        .set_storage_configuration(callable_contract_storage_configuration.unwrap());
    let callable_contract_id = Contract::load_from(
        CALLABLE_CONTRACT_BINARY_PATH,
        callable_contract_configuration,
    )
    .unwrap()
    .deploy(&deployer_wallet, TxParameters::default())
    .await?;

    Ok(CallableContract::new(callable_contract_id, deployer_wallet))
}

fn eip_191_personal_sign_format(message_hash: Message) -> Message {
    let initial_byte = 0x19u8;
    let version_byte = 0x45u8;

    let mut eip_191_data: Vec<u8> = vec![initial_byte, version_byte];
    eip_191_data.append(&mut message_hash.to_vec());

    let eip_191_formatted_message = keccak_hash(&eip_191_data);
    Message::from_bytes(*eip_191_formatted_message)
}

fn ethereum_prefix(formatted_message: Message) -> Message {
    let prefix = r#"\x19Ethereum Signed Message:\n32"#;

    let mut eth_prefix_data: Vec<u8> = Vec::new();
    eth_prefix_data.append(&mut prefix.as_bytes().to_vec());
    eth_prefix_data.append(&mut formatted_message.to_vec());

    let eth_prefixed_message = keccak_hash(eth_prefix_data);
    Message::from_bytes(*eth_prefixed_message)
}

pub(crate) async fn format_and_sign(
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

    let signature_bytes: [u8; 64] = signature.into();

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

pub(crate) async fn setup_env(private_key: &str) -> Result<(SecretKey, Caller, Caller), Error> {
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

    let multisig_storage_configuration =
        StorageConfiguration::load_from(MULTISIG_CONTRACT_STORAGE_PATH);
    let multisig_configuration = LoadConfiguration::default()
        .set_storage_configuration(multisig_storage_configuration.unwrap());
    let multisig_contract_id =
        Contract::load_from(MULTISIG_CONTRACT_BINARY_PATH, multisig_configuration)
            .unwrap()
            .deploy(&deployer_wallet, TxParameters::default())
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

pub(crate) fn transfer_parameters(
    deployer: &Caller,
    nonce: u64,
) -> (WalletUnlocked, Identity, Transaction) {
    let receiver_wallet = WalletUnlocked::new_random(None);
    let receiver = Identity::Address(receiver_wallet.address().try_into().unwrap());

    let transaction = Transaction {
        contract_call_params: None,
        contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
        nonce,
        target: receiver.clone(),
        transfer_params: TransferParams {
            asset_id: base_asset_contract_id(),
            value: Some(DEFAULT_TRANSFER_AMOUNT),
        },
    };

    (receiver_wallet, receiver, transaction)
}

pub(crate) fn call_parameters(
    deployer: &Caller,
    nonce: u64,
    callable_contract: &CallableContract<WalletUnlocked>,
    with_value: bool,
) -> Transaction {
    match with_value {
        false => Transaction {
            contract_call_params: Some(ContractCallParams {
                calldata: Bytes(calldata!(
                    Address::from(deployer.wallet.address()),
                    DEFAULT_CALLDATA_VALUE_PARAM
                )),
                forwarded_gas: DEFAULT_FORWARDED_GAS,
                function_selector: Bytes(fn_selector!(change_mapping_without_value(Address, u64))),
                single_value_type_arg: false,
            }),
            contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
            nonce,
            target: Identity::ContractId(callable_contract.contract_id().into()),
            transfer_params: TransferParams {
                asset_id: base_asset_contract_id(),
                value: None,
            },
        },
        true => Transaction {
            contract_call_params: Some(ContractCallParams {
                calldata: Bytes(calldata!(
                    Address::from(deployer.wallet.address()),
                    DEFAULT_CALLDATA_VALUE_PARAM
                )),
                forwarded_gas: DEFAULT_FORWARDED_GAS,
                function_selector: Bytes(fn_selector!(change_mapping_with_value(Address, u64))),
                single_value_type_arg: false,
            }),
            contract_identifier: deployer.contract.contract_id().try_into().unwrap(),
            nonce,
            target: Identity::ContractId(callable_contract.contract_id().into()),
            transfer_params: TransferParams {
                asset_id: base_asset_contract_id(),
                value: Some(DEFAULT_TRANSFER_AMOUNT),
            },
        },
    }
}

pub(crate) async fn transfer_signatures(
    private_key: SecretKey,
    tx_hash: Message,
) -> Vec<SignatureInfo> {
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
