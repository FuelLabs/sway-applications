use fuels::prelude::*;

pub mod utils;
use utils::*;

#[tokio::test]
async fn can_get_contract_id() {
    let instance = get_contract_instance().await;

    // Now you have an instance of your contract you can use to test each function
}
