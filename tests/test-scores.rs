use rock_paper_scissors::*;

#[test]
fn test_scores_new() {
    let scores = Scores::new();

    assert_eq!(scores, Scores {
        user_wins: 0,
        enemy_wins: 0,
    });
}

#[test]
fn test_check_for_winner() {
    let game_settings = GameSettings::from_first_to(3);

    let scores = Scores {
        user_wins: 3,
        enemy_wins: 1,
    };

    assert_eq!(scores.check_for_winner(&game_settings), Ok(Winner::User));

    let scores = Scores {
        user_wins: 2,
        enemy_wins: 3,
    };

    assert_eq!(scores.check_for_winner(&game_settings), Ok(Winner::Enemy));

    let scores = Scores {
        user_wins: 0,
        enemy_wins: 2,
    };

    assert_eq!(scores.check_for_winner(&game_settings), Err("rock-paper-scissors: err: No winner yet"));
}

#[test]
fn test_scores_reset() {
    let mut scores = Scores {
        user_wins: 3,
        enemy_wins: 2,
    };

    scores.reset();

    assert_eq!(scores, Scores {
        user_wins: 0,
        enemy_wins: 0,
    });
}