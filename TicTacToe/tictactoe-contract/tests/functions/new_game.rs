use crate::utils::{interface::new_game, setup::setup};

mod success {

    use super::*;
    use crate::utils::setup::NewGameEvent;

    #[tokio::test]
    async fn create_a_new_game() {
        let (player_one, player_two) = setup().await;

        let response = new_game(
            &player_one.contract,
            &player_one.identity,
            &player_two.identity,
        )
        .await;
        let log = response.decode_logs_with_type::<NewGameEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            NewGameEvent {
                player_one: player_one.identity,
                player_two: player_two.identity,
            }
        );
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "GameHasNotEnded")]
    async fn when_the_game_is_still_playing() {
        let (player_one, player_two) = setup().await;

        new_game(
            &player_one.contract,
            &player_one.identity,
            &player_two.identity,
        )
        .await;
        new_game(
            &player_one.contract,
            &player_one.identity,
            &player_two.identity,
        )
        .await;
    }
}
