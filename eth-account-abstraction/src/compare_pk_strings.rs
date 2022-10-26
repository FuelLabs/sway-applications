use fuels::{
    prelude::*,
    signers::fuel_crypto::{
        SecretKey,
        Message,
    }
};
use std::str::FromStr;

pub async fn compare_pk_strings() {
    let private_key = "862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301";

    //Using SecretKey::from_str
    let secret = SecretKey::from_str(private_key).unwrap();

    let wallet = WalletUnlocked::new_from_private_key(secret, None);

    let message: [u8; 32] = [0; 32];

    let sig = wallet.sign_message(&message).await.unwrap();

    let message = Message::new(message);

    let pub_key = sig.recover(&message).unwrap();

    sig.verify(&pub_key, &message).unwrap();

    println!("-------------------------");
    println!("Private key 1: {}", secret);
    println!("Public key  1: {}", pub_key);
    println!("Signature   1: {}", sig);


    //Using &str.parse()
    let secret: SecretKey = private_key.parse().unwrap();

    let wallet = WalletUnlocked::new_from_private_key(secret, None);

    let message: [u8; 32] = [0; 32];

    let sig = wallet.sign_message(&message).await.unwrap();

    let message = Message::new(message);

    let pub_key = sig.recover(&message).unwrap();

    sig.verify(&pub_key, &message).unwrap();

    println!("-------------------------");
    println!("Private key 2: {}", secret);
    println!("Public key  2: {}", pub_key);
    println!("Signature   2: {}", sig);
}