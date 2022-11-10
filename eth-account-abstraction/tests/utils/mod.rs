use fuels::{
    prelude::*,
    signers::fuel_crypto::{Hasher, Message, SecretKey, Signature},
    tx::{Bytes32, Bytes64},
};

use sha3::{Digest, Keccak256};

abigen!(
    CountApprovalsContract,
    "out/debug/eth-account-abstraction-abi.json"
);

pub async fn test_recover_and_match_addresses(private_key: &str) {
    let (private_key, contract) = setup_env(private_key).await.unwrap();

    let message = "Data to sign";
    let message_hash = Message::new(message);

    //Fuel signature. Fuel wallet. No format. No prefix.
    let signature_data_fuel = format_and_sign(
        private_key,
        message_hash,
        MessageFormat::None(),
        MessagePrefix::None(),
        WalletType::Fuel(),
    )
    .await;

    //EVM signature. EVM wallet. EIP-191 personal sign format. Ethereum prefix.
    let signature_data_evm = format_and_sign(
        private_key,
        message_hash,
        MessageFormat::EIP191PersonalSign(),
        MessagePrefix::Ethereum(),
        WalletType::EVM(),
    )
    .await;

    let signatures_data: Vec<SignatureData> = vec![signature_data_evm, signature_data_fuel];

    let response = contract
        .methods()
        .count_approvals(Bits256(message_hash.try_into().unwrap()), signatures_data)
        .call()
        .await
        .unwrap();

    assert_eq!(response.value, 6);
    println!("Contract response: \n{:?}", response);
}

async fn setup_env(private_key: &str) -> Result<(SecretKey, CountApprovalsContract), Error> {
    let private_key: SecretKey = private_key.parse().unwrap();

    let mut wallet = WalletUnlocked::new_from_private_key(private_key, None);

    let num_assets = 1;
    let coins_per_asset = 10;
    let amount_per_coin = 15;
    let (coins, _asset_ids) = setup_multiple_assets_coins(
        wallet.address(),
        num_assets,
        coins_per_asset,
        amount_per_coin,
    );

    let (provider, _socket_addr) = setup_test_provider(coins.clone(), vec![], None).await;

    wallet.set_provider(provider);

    let contract_id = Contract::deploy(
        "out/debug/eth-account-abstraction.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::default(),
    )
    .await?;

    let contract_instance = CountApprovalsContract::new(contract_id, wallet.clone());

    Ok((private_key, contract_instance))
}

async fn format_and_sign(
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

    let signature = B512 {
        bytes: [
            Bits256(signature_bytes[..32].try_into().unwrap()),
            Bits256(signature_bytes[32..].try_into().unwrap()),
        ],
    };

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
