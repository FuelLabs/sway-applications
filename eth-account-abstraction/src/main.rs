// mod derive_from_secp;
mod compare;

#[tokio::main]
async fn main() {
    // let _ = derive_from_secp::derive_from_secp();
    compare::compare().await;
}