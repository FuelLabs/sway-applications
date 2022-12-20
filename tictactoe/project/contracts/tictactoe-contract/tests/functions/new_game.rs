use crate::utils::test_helpers::setup;
use crate::utils::abi_calls::new_game;

mod success {
    use super::*;

    #[tokio::test]
    async fn new_game_called_by_many_players() {
        let (p1, p2, p3) = setup().await;

        new_game(&p1.tictactoe, p1.wallet.address(), p1.wallet.address()).await;
        new_game(&p2.tictactoe, p1.wallet.address(), p2.wallet.address()).await;
        new_game(&p3.tictactoe, p3.wallet.address(), p3.wallet.address()).await;
    }
}