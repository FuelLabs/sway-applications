use crate::utils::{
    interface::{make_move, new_game},
    setup::setup,
};

mod success {

    use super::*;
    use crate::utils::setup::{GameDrawnEvent, GameWonEvent};

    #[tokio::test]
    async fn when_player_one_wins() {
        let (player_one, player_two) = setup().await;
        new_game(
            &player_one.contract,
            &player_one.identity,
            &player_two.identity,
        )
        .await;
        make_move(&player_one.contract, 0).await;
        make_move(&player_two.contract, 1).await;
        make_move(&player_one.contract, 3).await;
        make_move(&player_two.contract, 2).await;
        let response = make_move(&player_one.contract, 6).await;

        let log = response.decode_logs_with_type::<GameWonEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            GameWonEvent {
                player: player_one.identity,
            }
        );
    }

    #[tokio::test]
    async fn when_player_two_wins() {
        let (player_one, player_two) = setup().await;
        new_game(
            &player_one.contract,
            &player_one.identity,
            &player_two.identity,
        )
        .await;
        make_move(&player_one.contract, 0).await;
        make_move(&player_two.contract, 1).await;
        make_move(&player_one.contract, 5).await;
        make_move(&player_two.contract, 4).await;
        make_move(&player_one.contract, 6).await;
        let response = make_move(&player_two.contract, 7).await;

        let log = response.decode_logs_with_type::<GameWonEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            GameWonEvent {
                player: player_two.identity,
            }
        );
    }

    #[tokio::test]
    async fn when_there_is_a_draw() {
        let (player_one, player_two) = setup().await;
        new_game(
            &player_one.contract,
            &player_one.identity,
            &player_two.identity,
        )
        .await;
        make_move(&player_one.contract, 4).await;
        make_move(&player_two.contract, 0).await;
        make_move(&player_one.contract, 7).await;
        make_move(&player_two.contract, 1).await;
        make_move(&player_one.contract, 2).await;
        make_move(&player_two.contract, 6).await;
        make_move(&player_one.contract, 3).await;
        make_move(&player_two.contract, 5).await;
        let response = make_move(&player_one.contract, 8).await;

        let log = response.decode_logs_with_type::<GameDrawnEvent>().unwrap();
        let event = log.get(0).unwrap();

        assert_eq!(
            *event,
            GameDrawnEvent {
                player_one: player_one.identity,
                player_two: player_two.identity,
            }
        );
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

    #[tokio::test]
    #[should_panic(expected = "IncorrectPlayerTurn")]
    async fn when_the_wrong_player_makes_a_move() {
        let (player_one, player_two) = setup().await;
        new_game(
            &player_one.contract,
            &player_one.identity,
            &player_two.identity,
        )
        .await;
        make_move(&player_two.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidPosition")]
    async fn when_a_move_is_out_of_bounds() {
        let (player_one, player_two) = setup().await;
        new_game(
            &player_one.contract,
            &player_one.identity,
            &player_two.identity,
        )
        .await;
        make_move(&player_one.contract, 10).await;
    }

    #[tokio::test]
    #[should_panic(expected = "CellIsNotEmpty")]
    async fn when_a_cell_is_occupied() {
        let (player_one, player_two) = setup().await;
        new_game(
            &player_one.contract,
            &player_one.identity,
            &player_two.identity,
        )
        .await;
        make_move(&player_one.contract, 1).await;
        make_move(&player_two.contract, 1).await;
    }
}
