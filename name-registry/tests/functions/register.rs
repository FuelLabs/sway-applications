use crate::utils::*;

#[tokio::test]
async fn can_extend() {
    let (instance, _id, _wallet) = get_contract_instance().await;

    let name = String::from("SwaySway");

    register(&instance, &name, 5000).await;
}
