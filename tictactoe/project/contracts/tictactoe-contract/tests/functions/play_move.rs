use crate::utils::test_helpers::setup;
use crate::utils::abi_calls::{new_game, play_move};

mod success {
    use super::*;
}

mod revert {
    use super::*;
    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn move_out_of_bound() {
        let (p1, _, _) = setup().await;
        new_game(&p1.tictactoe, p1.wallet.address(), p1.wallet.address()).await;
        play_move(&p1.tictactoe, 9).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn move_out_of_bound_max() {
        let (p1, _, _) = setup().await;
        new_game(&p1.tictactoe, p1.wallet.address(), p1.wallet.address()).await;
        play_move(&p1.tictactoe, u64::MAX).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn move_already_played() {
        let (p1, _, _) = setup().await;
        new_game(&p1.tictactoe, p1.wallet.address(), p1.wallet.address()).await;
        play_move(&p1.tictactoe, 0).await;
        play_move(&p1.tictactoe, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn wrong_player_called_play_move() {
        let (p1, p2, _) = setup().await;
        new_game(&p1.tictactoe, p1.wallet.address(), p1.wallet.address()).await;
        play_move(&p2.tictactoe, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn game_already_over() {
        let (p1, p2, _) = setup().await;
        new_game(&p1.tictactoe, p1.wallet.address(), p2.wallet.address()).await;
        play_move(&p1.tictactoe, 0).await;
        play_move(&p2.tictactoe, 3).await;
        play_move(&p1.tictactoe, 1).await;
        play_move(&p2.tictactoe, 4).await;
        play_move(&p1.tictactoe, 2).await;
        // p1 won
        play_move(&p2.tictactoe, 5).await;
    }
}