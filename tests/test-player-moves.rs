use rock_paper_scissors::*;

#[test]
fn test_player_move_new() {
    let player_moves = PlayerMoves::new();

    assert_eq!(player_moves, PlayerMoves {
        user_move: MoveType::None,
        enemy_move: MoveType::None,
    })

}

#[test]
fn test_check_who_wins_round() {
    let player_moves = PlayerMoves {
        user_move: MoveType::Rock,
        enemy_move: MoveType::Rock,
    };

    assert_eq!(player_moves.check_who_wins_round(), Winner::Tie);

    let player_moves = PlayerMoves {
        user_move: MoveType::Rock,
        enemy_move: MoveType::Scissors,
    };

    assert_eq!(player_moves.check_who_wins_round(), Winner::User);

    let player_moves = PlayerMoves {
        user_move: MoveType::Paper,
        enemy_move: MoveType::Scissors,
    };

    assert_eq!(player_moves.check_who_wins_round(), Winner::Enemy);
}

