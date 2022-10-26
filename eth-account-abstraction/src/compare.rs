use fuels::{
    prelude::*,
    signers::fuel_crypto::{
        SecretKey,
        Signature,
        Message,
    }
};
use fuel_vm::crypto;
use std::str::FromStr;

pub async fn compare() {
    let private_key = "862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301";

    //Using secp256k1_sign_compact_recoverable
    let secret = SecretKey::from_str(private_key).unwrap();

    let message: [u8; 32] = [0; 32];

    let sig = crypto::secp256k1_sign_compact_recoverable(secret.as_ref(), message.as_ref())
        .expect("Failed to generate signature");
    let sig = unsafe { Signature::from_bytes_unchecked(*sig) };

    let message = Message::new(message);

    let pub_key = sig.recover(&message).unwrap();

    sig.verify(&pub_key, &message).unwrap();

    println!("-------------------------");
    println!("Using secp256k1_sign_compact_recoverable:");
    println!("Private key : {}", secret);
    println!("Public key  : {}", pub_key);
    println!("Signature   : {}", sig);


    //Using sign_message
    let secret = SecretKey::from_str(private_key).unwrap();

    let wallet = WalletUnlocked::new_from_private_key(secret, None);

    let message: [u8; 32] = [0; 32];

    let sig = wallet.sign_message(&message).await.unwrap();

    let message = Message::new(message);

    let pub_key = sig.recover(&message).unwrap();

    sig.verify(&pub_key, &message).unwrap();

    println!("-------------------------");
    println!("Using sign_message:");
    println!("Private key : {}", secret);
    println!("Public key  : {}", pub_key);
    println!("Signature   : {}", sig);
}

/*
Output:

-------------------------
Using secp256k1_sign_compact_recoverable:
Private key : 862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301
Public key  : c69ab3ee05ce57ee70759667c227b63eecb1d40575ac8d16b2538efcb83ded0f34370cf69d5bbda32d1dfa7bd52ec7067f86a91b9a529080a771e2661f469100
Signature   : 4bcc7daf3f607a57dc843c47d97ce580ed3c717a2984938195c1b3d40a5fc580135122329c2f8f636bddc939bd838a2c2fc43082d9b59f93ea7cb75e01de59b7
-------------------------
Using sign_message:
Private key : 862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301
Public key  : 5a76336abf4ac1b759390876d152e016e7e491dac6b074a1f0cde2caf86c654d699e549564eab8f336589d9664b937394ff1a0037f037068577d1bbbf0e56511
Signature   : c8b46627ad739aecd3e9008ad97722e4ff4a1b3856626778a35cad3369934e6e8a36611b0d1b07be01ca62948b494c50c0abe8dfa4892cea704c83871bae3e02
*/