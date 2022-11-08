use fuels::{
    contract::{
        script::Script,
    },
    prelude::*,
    signers::fuel_crypto::{
        Hasher,
        Message,
        SecretKey,
        Signature,
    },
    tx::{
        Bytes32,
        Receipt, 
        Transaction,
    },
};

use sha3::{
    Digest, 
    Keccak256
};


pub async fn test_recover_and_match_address_with_parameters(private_key: &str, eip_191_format: bool, eth_prefix: bool) {

    let private_key = setup_env(private_key).await.unwrap();

    let message = "Data to sign";
    let message_hash = Message::new(message);

    // let signature = format_and_sign(private_key, message_hash, eip_191_format, eth_prefix).await;
    let signature = format_and_sign(private_key, message_hash, eip_191_format, eth_prefix).await;
  
    let script_data: Vec<u8> = [
        signature.to_vec(),
        message_hash.to_vec(),
        ].into_iter().flatten().collect();

    let path_to_bin = "out/debug/eth-account-abstraction.bin";

    let receipts = run_compiled_script(path_to_bin, None, script_data).await.unwrap();

    let return_value = receipts[0].val().unwrap();

    //Script returns bool
    //1 == true
    //0 == false
    println!("Receipt : {:?}", receipts[0]);
    assert_eq!(1, return_value);

}


async fn setup_env(private_key: &str) -> Result< SecretKey, Error> {
    
    let private_key: SecretKey = private_key.parse().unwrap();

    Ok(private_key)
}


async fn format_and_sign(private_key: SecretKey, message_hash: Message, eip_191_format: bool, eth_prefix: bool) -> Signature {

    if eip_191_format {

        let initial_byte= 0x19u8;
        let version_byte= 0x45u8;
        
        let mut eip_191_data: Vec<u8> = vec![initial_byte, version_byte];
        eip_191_data.append(&mut message_hash.to_vec());
        
        let eip_191_formatted_message = keccak_hash(&eip_191_data);

        if eth_prefix {

            let prefix = r#"\x19Ethereum Signed Message:\n32"#;

            let mut eth_prefix_data: Vec<u8> = Vec::new();
            eth_prefix_data.append(&mut prefix.as_bytes().to_vec());
            eth_prefix_data.append(&mut eip_191_formatted_message.to_vec());

            let eth_prefixed_message = Hasher::hash(eth_prefix_data);

            let eth_prefixed_message = unsafe { Message::from_bytes_unchecked(*eth_prefixed_message) };
            Signature::sign(&private_key, &eth_prefixed_message)

        } else {

            let eip_191_formatted_message = unsafe { Message::from_bytes_unchecked(*eip_191_formatted_message) };
            Signature::sign(&private_key, &eip_191_formatted_message)

        }
    } else {

        Signature::sign(&private_key, &message_hash)

    }
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


//custom run_compiled_script; with input data
pub async fn run_compiled_script(
    binary_filepath: &str, 
    provider: Option<Provider>, 
    script_data: Vec<u8>,
) -> Result<Vec<Receipt>, Error> {

    let script_binary = std::fs::read(binary_filepath)?;

    let server = FuelService::new_node(Config::local_node()).await.unwrap();
    
    let provider = provider.unwrap_or(Provider::connect(server.bound_address.to_string()).await?);

    let script = build_script(script_binary, script_data);

    script.call(&provider).await

}


fn build_script(script_binary: Vec<u8>, script_data: Vec<u8>) -> Script {

    let tx_params = TxParameters::default();
    
    Script::new(Transaction::script(
        tx_params.gas_price,
        tx_params.gas_limit,
        tx_params.maturity,
        script_binary,
        script_data,
        vec![],
        vec![],
        vec![],
    ))
    
}
