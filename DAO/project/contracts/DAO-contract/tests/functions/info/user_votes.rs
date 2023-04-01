use crate::utils::{interface::info::user_votes, setup::setup};

mod sucess {
    use super::*;
    use crate::utils::{
        interface::core::{constructor, create_proposal, deposit, vote},
        setup::{mint, proposal_transaction, Votes},
    };
    use fuels::{prelude::CallParameters, tx::AssetId};

    #[tokio::test]
    pub async fn user_can_check_user_votes() {
        let (_gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_token_id).await;

        mint(
            deployer.gov_token.as_ref().unwrap(),
            asset_amount,
            user.wallet.address(),
        )
        .await;

        let call_params = CallParameters::new(
            Some(asset_amount),
            Some(AssetId::from(*gov_token_id)),
            Some(100_000),
        );
        deposit(&user.dao_voting, call_params).await;
        let proposal_transaction = proposal_transaction(gov_token_id);
        create_proposal(&user.dao_voting, 10, 10, proposal_transaction).await;
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 0).await,
            Votes {
                yes_votes: 0,
                no_votes: 0
            }
        );
        vote(&user.dao_voting, true, 0, asset_amount).await;
        assert_eq!(
            user_votes(&user.dao_voting, user.wallet.address(), 0).await,
            Votes {
                yes_votes: asset_amount,
                no_votes: 0
            }
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidId")]
    pub async fn on_invalid_proposal_id() {
        let (_gov_token, _gov_token_id, _deployer, user, _asset_amount) = setup().await;
        user_votes(&user.dao_voting, user.wallet.address(), 0).await;
    }
}
