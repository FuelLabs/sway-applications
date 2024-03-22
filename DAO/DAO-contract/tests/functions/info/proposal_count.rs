mod success {
    use crate::utils::{
        interface::{
            core::{constructor, create_proposal},
            info::proposal_count,
        },
        setup::{proposal_transaction, setup},
    };

    #[tokio::test]
    async fn use_can_get_proposal_count() {
        let (gov_asset_id, _other_asset_id, deployer, user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;

        assert_eq!(proposal_count(&user.dao_voting).await, 1);
    }
}
