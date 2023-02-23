mod success {

    use crate::utils::{
        abi_calls::{create_escrow, escrows},
        test_helpers::{create_arbiter, create_asset, mint, setup},
        Buyer, EscrowInfo, Seller, State,
    };
    use fuels::types::{Address, Identity};

    #[tokio::test]
    async fn returns_none() {
        let (_arbiter, _buyer, seller, _defaults) = setup().await;
        assert!(matches!(escrows(&seller.contract, 0).await, None));
    }

    #[tokio::test]
    async fn returns_escrow_info() {
        let (arbiter, buyer, seller, defaults) = setup().await;
        let arbiter_obj = create_arbiter(
            arbiter.wallet.address(),
            defaults.asset_id,
            defaults.asset_amount,
        )
        .await;
        let asset = create_asset(defaults.asset_amount, defaults.asset_id).await;

        mint(
            seller.wallet.address(),
            defaults.asset_amount,
            &defaults.asset,
        )
        .await;

        assert!(matches!(escrows(&seller.contract, 0).await, None));

        create_escrow(
            defaults.asset_amount,
            &arbiter_obj,
            &defaults.asset_id,
            vec![asset.clone()],
            buyer.wallet.address(),
            &seller.contract,
            defaults.deadline,
        )
        .await;

        assert_eq!(
            escrows(&seller.contract, 0).await.unwrap(),
            EscrowInfo {
                arbiter: arbiter_obj,
                asset_count: 1,
                buyer: Buyer {
                    address: Identity::Address(Address::from(buyer.wallet.address())),
                    asset: None,
                    deposited_amount: 0,
                },
                deadline: defaults.deadline,
                disputed: false,
                first_asset_index: 0,
                seller: Seller {
                    address: Identity::Address(Address::from(seller.wallet.address())),
                },
                state: State::Pending,
            }
        );
    }
}
