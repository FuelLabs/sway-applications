use fuels::{
    contract::contract::CallResponse,
    prelude::*,
    signers::fuel_crypto::{Hasher, Message, SecretKey, Signature},
    tx::{Bytes32, Bytes64},
};

use sha3::{Digest, Keccak256};

abigen!(MultiSigContract, "out/debug/multisig-wallet-abi.json");

pub const VALID_SIGNER_PK: &str =
    "862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301";

pub mod abi_calls {

    use super::*;

    pub async fn constructor(
        contract: &MultiSigContract,
        users: Vec<User>,
        threshold: u64,
    ) -> CallResponse<()> {
        contract
            .methods()
            .constructor(users, threshold)
            .call()
            .await
            .unwrap()
    }

    pub async fn execute_transaction(
        contract: &MultiSigContract,
        to: Identity,
        value: u64,
        data: Bits256,
        signatures_data: Vec<SignatureData>,
    ) -> CallResponse<()> {
        contract
            .methods()
            .execute_transaction(to, value, data, signatures_data)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn transfer(
        contract: &MultiSigContract,
        to: Identity,
        asset_id: ContractId,
        value: u64,
        data: Bits256,
        signatures_data: Vec<SignatureData>,
    ) -> CallResponse<()> {
        contract
            .methods()
            .transfer(to, asset_id, value, data, signatures_data)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn nonce(contract: &MultiSigContract) -> CallResponse<u64> {
        contract.methods().nonce().call().await.unwrap()
    }

    pub async fn balance(contract: &MultiSigContract, asset_id: ContractId) -> CallResponse<u64> {
        contract.methods().balance(asset_id).call().await.unwrap()
    }

    pub async fn transaction_hash(
        contract: &MultiSigContract,
        to: Identity,
        value: u64,
        data: Bits256,
        nonce: u64,
    ) -> CallResponse<Bits256> {
        contract
            .methods()
            .transaction_hash(to, value, data, nonce)
            .call()
            .await
            .unwrap()
    }
}

pub mod test_helpers {

    use super::*;

    pub async fn setup_env(
        private_key: &str,
    ) -> Result<(SecretKey, MultiSigContract, WalletUnlocked), Error> {
        let private_key: SecretKey = private_key.parse().unwrap();

        let mut wallet = WalletUnlocked::new_from_private_key(private_key, None);

        let num_asset = 1;
        let coins_per_asset = 10;
        let amount_per_coin = 200;
        let (coins, _asset_ids) = setup_multiple_assets_coins(
            wallet.address(),
            num_asset,
            coins_per_asset,
            amount_per_coin,
        );

        let (provider, _socket_addr) = setup_test_provider(coins.clone(), vec![], None, None).await;

        wallet.set_provider(provider);

        let contract_id = Contract::deploy(
            "out/debug/multisig-wallet.bin",
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await?;

        let contract_instance = MultiSigContract::new(contract_id, wallet.clone());

        Ok((private_key, contract_instance, wallet))
    }

    pub async fn format_and_sign(
        private_key: SecretKey,
        message_hash: Message,
        format: MessageFormat,
        prefix: MessagePrefix,
        wallet_type: WalletType,
    ) -> SignatureData {
        //Format
        let formatted_message = match format {
            MessageFormat::None() => message_hash,
            MessageFormat::EIP191PersonalSign() => eip_191_personal_sign_format(message_hash),
        };

        //Prefix
        let prefixed_message = match prefix {
            MessagePrefix::None() => formatted_message,
            MessagePrefix::Ethereum() => ethereum_prefix(formatted_message),
        };

        //Sign
        let signature = Signature::sign(&private_key, &prefixed_message);

        //Create SignatureData
        let signature_bytes: Bytes64 = Bytes64::try_from(signature).unwrap();

        let signature = B512::from((
            Bits256(signature_bytes[..32].try_into().unwrap()),
            Bits256(signature_bytes[32..].try_into().unwrap()),
        ));

        SignatureData {
            signature,
            format,
            prefix,
            wallet_type,
        }
    }

    fn eip_191_personal_sign_format(message_hash: Message) -> Message {
        let initial_byte = 0x19u8;
        let version_byte = 0x45u8;

        let mut eip_191_data: Vec<u8> = vec![initial_byte, version_byte];
        eip_191_data.append(&mut message_hash.to_vec());

        let eip_191_formatted_message = keccak_hash(&eip_191_data);
        unsafe { Message::from_bytes_unchecked(*eip_191_formatted_message) }
    }

    fn ethereum_prefix(formatted_message: Message) -> Message {
        let prefix = r#"\x19Ethereum Signed Message:\n32"#;

        let mut eth_prefix_data: Vec<u8> = Vec::new();
        eth_prefix_data.append(&mut prefix.as_bytes().to_vec());
        eth_prefix_data.append(&mut formatted_message.to_vec());

        let eth_prefixed_message = Hasher::hash(eth_prefix_data);
        unsafe { Message::from_bytes_unchecked(*eth_prefixed_message) }
    }

    // A keccak-256 method
    fn keccak_hash<B>(data: B) -> Bytes32
    where
        B: AsRef<[u8]>,
    {
        // create a Keccak256 object
        let mut hasher = Keccak256::new();

        // write input message
        hasher.update(data);

        <[u8; Bytes32::LEN]>::from(hasher.finalize()).into()
    }

    pub fn base_asset_contract_id() -> ContractId {
        ContractId::new(BASE_ASSET_ID.try_into().unwrap())
    }
}
