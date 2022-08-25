use crate::utils::{
    abi_calls::{constructor, create_proposal, proposal_count},
    test_helpers::{proposal_transaction, setup},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn use_can_get_proposal_count() {
        let (_gov_token, gov_token_id, deployer, user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;

        assert_eq!(proposal_count(&user.dao_voting).await, 1);
    }
}
