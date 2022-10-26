// mod derive_from_secp;
mod compare_pk_strings;

#[tokio::main]
async fn main() {
    // let _ = derive_from_secp::derive_from_secp();
    compare_pk_strings::compare_pk_strings().await;
}