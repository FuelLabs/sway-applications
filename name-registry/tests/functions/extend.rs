use crate::utils::*;

#[tokio::test]
async fn can_extend() {
    let (instance, _id, _wallet) = get_contract_instance().await;

    let name = String::from("SwaySway");

    register(&instance, &name, 5000).await;

    let old_expiry = expiry(&instance, &name).await;

    extend(&instance, &name, 5000).await;

    let new_expiry = expiry(&instance, &name).await;

    assert_eq!(old_expiry + 5000, new_expiry);
}
