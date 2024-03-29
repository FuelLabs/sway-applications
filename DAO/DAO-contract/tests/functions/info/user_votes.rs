use crate::utils::{interface::info::user_votes, setup::setup};

mod sucess {
    use super::*;
    use crate::utils::{
        interface::core::{constructor, create_proposal, deposit, vote},
        setup::{proposal_transaction, Votes},
    };
    use fuels::{prelude::CallParameters, types::AssetId};

    #[tokio::test]
    pub async fn user_can_check_user_votes() {
        let (gov_asset_id, _other_asset_id, deployer, user, asset_amount) = setup().await;
        constructor(&deployer.dao_voting, gov_asset_id).await;

        let call_params = CallParameters::new(asset_amount, AssetId::from(*gov_asset_id), 100_000);
        deposit(&user.dao_voting, call_params).await;
        let proposal_transaction = proposal_transaction(gov_asset_id);
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
        let (_gov_asset, _gov_asset_id, _deployer, user, _asset_amount) = setup().await;
        user_votes(&user.dao_voting, user.wallet.address(), 0).await;
    }
}
