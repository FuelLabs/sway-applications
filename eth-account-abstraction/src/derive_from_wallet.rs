use fuels::prelude::*;
use fuels::signers::fuel_crypto::SecretKey;
use std::str::FromStr;
use fuel_crypto::{
    Hasher,
    Message,
};
use sha3::{Digest, Keccak256};
use fuel_types::Bytes32;

pub async fn derive_from_wallet() {
    //Setup wallet
    let (provider, _address) = setup_test_provider(vec![], vec![], None).await;
    let secret = SecretKey::from_str(
        "862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301",
    ).unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider));

    //Sign message
    let message = "my message";
    let signature = wallet.sign_message(message).await.unwrap();

    //Recover public key
    let message = Message::new(message);
    let public_key = signature.recover(&message).unwrap();

    //Derive Fuel address
    let fuel_address = Hasher::hash(&public_key[..]);

    //Derive EVM address
    let evm_pubkey_hash = keccak_hash(&public_key[..]);

    //Display values
    println!("Secret Key: {:?}", secret);
    println!("Public Key: {:?}", public_key);
    println!("Fuel Address (sha2-256): {:?}", fuel_address);
    println!("EVM pubkey hash (keccak256): {:?}", evm_pubkey_hash);
}

// A keccak-256 method for generating EVM signatures
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