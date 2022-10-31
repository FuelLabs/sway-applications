use crate::utils::{
    abi_calls::{constructor, create_proposal, proposal},
    test_helpers::{proposal_transaction, setup},
    Identity, ProposalInfo,
};

mod success {
    use super::*;

    #[tokio::test]
    pub async fn user_can_get_proposal() {
        let (_gov_token, gov_token_id, deployer, user, _asset_amount, governor_id) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        let proposal_transaction = proposal_transaction(gov_token_id, 42, true);
        create_proposal(
            &user.dao_voting,
            10,
            10,
            governor_id,
            proposal_transaction.clone(),
        )
        .await;

        assert_eq!(
            proposal(&user.dao_voting, 0).await,
            ProposalInfo {
                author: Identity::Address(user.wallet.address().into()),
                yes_votes: 0,
                no_votes: 0,
                acceptance_percentage: 10,
                id: governor_id,
                proposal_transaction,
                deadline: 15,
                executed: false,
            }
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_invalid_proposal_id() {
        let (_gov_token, _gov_token_id, _deployer, user, _asset_amount, _) = setup().await;
        proposal(&user.dao_voting, 0).await;
    }
}
