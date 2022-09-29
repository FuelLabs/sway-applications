mod utils;

use utils::*;

#[tokio::test]
async fn can_register() {
    let (instance, _id) = get_contract_instance().await;

    // Now you have an instance of your contract you can use to test each function
    register(&instance, String::from("SwaySway"), 5000).await;
}
