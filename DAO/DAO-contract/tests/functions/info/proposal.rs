use crate::utils::{interface::info::proposal, setup::setup};

mod success {
    use super::*;
    use crate::utils::{
        interface::core::{constructor, create_proposal},
        setup::{proposal_transaction, ProposalInfo},
    };
    use fuels::types::Identity;

    #[tokio::test]
    pub async fn user_can_get_proposal() {
        let (gov_asset_id, _other_asset_id, deployer, user, _asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let proposal_transaction = proposal_transaction(gov_asset_id);
        create_proposal(&user.dao_voting, 10, 10, proposal_transaction.clone()).await;

        assert_eq!(
            proposal(&user.dao_voting, 0).await,
            ProposalInfo {
                author: Identity::Address(user.wallet.address().into()),
                yes_votes: 0,
                no_votes: 0,
                acceptance_percentage: 10,
                proposal_transaction,
                deadline: 13,
                executed: false,
            }
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidId")]
    async fn on_invalid_proposal_id() {
        let (_gov_asset, _gov_asset_id, _deployer, user, _asset_amount) = setup().await;
        proposal(&user.dao_voting, 0).await;
    }
}
