use fuels::{
    contract::call_response::FuelCallResponse,
    prelude::*,
    signers::fuel_crypto::{Message, SecretKey, Signature},
    tx::{Bytes32, Bytes64},
};

use rand::{rngs::StdRng, Rng, SeedableRng};

use sha3::{Digest, Keccak256};

abigen!(
    MultiSig,
    "./project/multisig-contract/out/debug/multisig-contract-abi.json"
);

pub const VALID_SIGNER_PK: &str =
    "862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301"; // Test-only private key

pub struct Caller {
    pub contract: MultiSig,
    pub wallet: WalletUnlocked,
}

pub mod paths {
    pub const MULTISIG_CONTRACT_BINARY_PATH: &str = "./out/debug/multisig-contract.bin";
    pub const MULTISIG_CONTRACT_STORAGE_PATH: &str =
        "./out/debug/multisig-contract-storage_slots.json";
}

pub mod interface {

    use super::*;

    pub async fn cancel_transaction(contract: &MultiSig) -> FuelCallResponse<()> {
        contract
            .methods()
            .cancel_transaction()
            .call()
            .await
            .unwrap()
    }

    pub async fn constructor(
        contract: &MultiSig,
        users: Vec<User>,
        threshold: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .constructor(threshold, users)
            .call()
            .await
            .unwrap()
    }

    pub async fn execute_transaction(
        contract: &MultiSig,
        to: Identity,
        value: u64,
        data: Bits256,
        signatures_data: Vec<SignatureInfo>,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .execute_transaction(data, signatures_data, to, value)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn transfer(
        contract: &MultiSig,
        to: Identity,
        asset_id: ContractId,
        value: u64,
        data: Bits256,
        signatures_data: Vec<SignatureInfo>,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .transfer(asset_id, data, signatures_data, to, value)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn nonce(contract: &MultiSig) -> FuelCallResponse<u64> {
        contract.methods().nonce().call().await.unwrap()
    }

    pub async fn balance(contract: &MultiSig, asset_id: ContractId) -> FuelCallResponse<u64> {
        contract.methods().balance(asset_id).call().await.unwrap()
    }

    pub async fn transaction_hash(
        contract: &MultiSig,
        to: Identity,
        value: u64,
        data: Bits256,
        nonce: u64,
    ) -> FuelCallResponse<Bits256> {
        contract
            .methods()
            .transaction_hash(data, nonce, to, value)
            .call()
            .await
            .unwrap()
    }
}

pub mod test_helpers {

    use super::*;
    use paths::{MULTISIG_CONTRACT_BINARY_PATH, MULTISIG_CONTRACT_STORAGE_PATH};

    pub const DEFAULT_THRESHOLD: u64 = 5;
    pub const DEFAULT_TRANSFER_AMOUNT: u64 = 200;

    pub async fn setup_env(private_key: &str) -> Result<(SecretKey, Caller, Caller), Error> {
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
            StorageConfiguration::with_storage_path(Some(
                MULTISIG_CONTRACT_STORAGE_PATH.to_string(),
            )),
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

    pub fn transfer_parameters() -> (WalletUnlocked, Identity, Bits256) {
        let receiver_wallet = WalletUnlocked::new_random(None);

        let receiver = Identity::Address(receiver_wallet.address().try_into().unwrap());

        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);

        (receiver_wallet, receiver, data)
    }

    pub async fn transfer_signatures(
        private_key: SecretKey,
        tx_hash: Message,
    ) -> Vec<SignatureInfo> {
        // - Fuel signature. Fuel wallet. No format. No prefix.
        let fuel_signature = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::None(),
            MessagePrefix::None(),
            WalletType::Fuel(),
        )
        .await;

        // - EVM signature. EVM wallet. EIP-191 personal sign format. Ethereum prefix.
        let evm_signature = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::EIP191PersonalSign(),
            MessagePrefix::Ethereum(),
            WalletType::EVM(),
        )
        .await;

        // - Must comply with ascending signers requirement of Multisig's count_approvals
        vec![evm_signature, fuel_signature]
    }

    pub async fn format_and_sign(
        private_key: SecretKey,
        message_hash: Message,
        message_format: MessageFormat,
        message_prefix: MessagePrefix,
        wallet_type: WalletType,
    ) -> SignatureInfo {
        let formatted_message = match message_format {
            MessageFormat::None() => message_hash,
            MessageFormat::EIP191PersonalSign() => eip_191_personal_sign_format(message_hash),
        };

        let prefixed_message = match message_prefix {
            MessagePrefix::None() => formatted_message,
            MessagePrefix::Ethereum() => ethereum_prefix(formatted_message),
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

    fn keccak_hash<B>(data: B) -> Bytes32
    where
        B: AsRef<[u8]>,
    {
        let mut hasher = Keccak256::new();

        hasher.update(data);

        <[u8; Bytes32::LEN]>::from(hasher.finalize()).into()
    }

    pub fn base_asset_contract_id() -> ContractId {
        ContractId::new(BASE_ASSET_ID.try_into().unwrap())
    }
}
