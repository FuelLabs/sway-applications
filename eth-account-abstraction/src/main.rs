use fuels::{
    prelude::*,
    signers::fuel_crypto::{
        SecretKey,
        Message,
    }
};
use std::str::FromStr;

#[async_std::main]
async fn main() {
    derive_pub_key().await;
}

async fn derive_pub_key() {
    let secret = SecretKey::from_str(
        "5f70feeff1f229e4a95e1056e8b4d80d0b24b565674860cc213bdb07127ce1b1",//Example PK from rustSDK book 1.4.7
    ).unwrap();

    let wallet = WalletUnlocked::new_from_private_key(secret, None);

    let message = "0";

    let sig = wallet.sign_message(&message).await.unwrap();

    let message = Message::new(message);

    let pub_key = sig.recover(&message).unwrap();

    sig.verify(&pub_key, &message).unwrap();

    println!("Private key: {}", secret);
    println!("Public key: {}", pub_key);
}