use crate::utils::{
    interface::make_move,
    setup::setup,
};

mod success {

    use super::*;
    
    #[tokio::test]
    async fn make_move() {
        let (player_one, player_two) = setup().await;

    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "GameHasEnded")]
    async fn when_the_game_has_ended() {
        let (player_one, _player_two) = setup().await;

        make_move(&player_one.contract, 1).await;
    }
}

