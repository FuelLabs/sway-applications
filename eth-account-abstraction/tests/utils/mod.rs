use fuels::{
    contract::{
        script::Script,
    },
    prelude::*,
    signers::fuel_crypto::{
        Message,
        SecretKey,
        Signature,
    },
    tx::{
        AssetId, 
        Bytes32,
        Receipt, 
        Transaction,
    },
};
use sha3::{
    Digest, 
    Keccak256
};

pub async fn test_recover_and_match_address_with_parameters(private_key: &str, eip_191_format: bool) {
    //Setup wallets
    let secret_key1: SecretKey =
            private_key
                .parse()
                .unwrap();

    let mut wallet = WalletUnlocked::new_from_private_key(secret_key1, None);
    
    let all_coins = [&wallet]
        .iter()
        .flat_map(|wallet| {
            setup_single_asset_coins(wallet.address(), AssetId::default(), 10, 1_000_000)
        })
        .collect::<Vec<_>>();

    //Setup provider
    let (provider, _) = setup_test_provider(
        all_coins,
        vec![],
        Some(Config {
            utxo_validation: true,
            ..Config::local_node()
        }),
    )
    .await;
    
    [&mut wallet]
        .iter_mut()
        .for_each(|wallet| wallet.set_provider(provider.clone()));

    //Create signature
    let message = "Data to sign";

    let message_hash = Message::new(message);

    let signature = format_and_sign(wallet, message_hash, eip_191_format).await;
      
    //prepare script and tx
    let script_data: Vec<u8> = [
        signature.to_vec(),
        message_hash.to_vec(),
    ]
    .into_iter()
    .flatten()
    .collect();

    let path_to_bin = "out/debug/eth-account-abstraction.bin";

    //run script
    let receipts = run_compiled_script(path_to_bin, None, script_data).await.unwrap();

    let return_value = receipts[0].val().unwrap();

    //Script returns bool
    println!("Sway script : {:?}", receipts[0]);
    //1 == true
    //0 == false
    assert_eq!(1, return_value);


    //Check inputs to eip-191 hash, in Rust vs Sway
    // println!("Rust Script : {:?}", eip_191_data);
    // println!("Sway script : {:?}", receipts[0].data().unwrap());
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

async fn format_and_sign(wallet: WalletUnlocked, message_hash: Message, eip_191_format: bool) -> Signature {
    if eip_191_format {
        //EIP-191 format
        /*
        let initial_byte= 0x19u8;
        let version_byte= 0x45u8;
        
        let mut eip_191_data: Vec<u8> = vec![initial_byte, version_byte];
        eip_191_data.append(&mut message_hash.to_vec());
        */
        //TODO: replace with correct, unpadded `eip_191_data` from above
        //once padding in Sway is resolved
        let eip_191_data: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 69, 180, 213, 8, 212, 50, 173, 93, 232, 25, 195, 255, 235, 146, 224, 80, 183, 99, 32, 241, 122, 150, 83, 86, 0, 113, 107, 19, 116, 130, 159, 96, 239];
    
        let eip_191_formatted_message = keccak_hash(&eip_191_data);
        
        //Sign
        wallet.sign_message(eip_191_formatted_message).await.unwrap()
    } else {
        //Sign
        wallet.sign_message(message_hash).await.unwrap()
    }
}
