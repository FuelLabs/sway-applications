use fuel_crypto::Hasher;

use fuel_vm::crypto;
use fuel_vm::prelude::*;

use anyhow::Result;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::str::FromStr;

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

pub fn derive_from_secp() -> Result<()> {
    let secp = Secp256k1::new();
    let secret =
        SecretKey::from_str("862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301")
            .unwrap();
    let public = PublicKey::from_secret_key(&secp, &secret).serialize_uncompressed();
    let public = Bytes64::try_from(&public[1..]).expect("Failed to parse public key!");
    // 64 byte fuel address is the sha-256 hash of the public key.
    let address = Hasher::hash(&public[..]);
    let evm_pubkeyhash = keccak_hash(&public[..]);

    // let message = b"The gift of words is the gift of deception and illusion.";
    // let e = Hasher::hash(&message[..]);



    let data_to_sign: [u8; 32] = [0; 32];

    let sig = crypto::secp256k1_sign_compact_recoverable(secret.as_ref(), data_to_sign.as_ref())
        .expect("Failed to generate signature");

    println!("Secret Key: {:?}", secret);
    println!("Public Key: {:?}", public);
    println!("Fuel Address (sha2-256): {:?}", address);
    println!("EVM pubkey hash (keccak256): {:?}", evm_pubkeyhash);
    // println!("Message Hash: {:?}", e);
    println!("Signature: {:?}", sig);

    Ok(())
}