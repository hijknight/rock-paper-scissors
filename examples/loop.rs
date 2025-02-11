use rock_paper_scissors::*;


fn main() {
    let first_to = 3;

    let mut scores = Scores::new();

    let mut round_counter = 0;

    loop {
        round_counter += 1;
        let player_moves = PlayerMoves::build();

        let winner = player_moves.check_who_wins_round();

        match winner {
            Winner::User => {
                scores.user_wins += 1;
                println!("User wins round {}", round_counter)

            },
            Winner::Enemy => {
                scores.enemy_wins += 1;
                println!("Enemy wins round {}", round_counter)
            },
            Winner::Tie => {
                println!("It's a tie! Replaying round {}...", round_counter);
                round_counter -= 1;
            },
        }

        if let Ok(winner) = scores.check_for_winner(first_to) {
            println!("Game Over! Winner: {}", winner.convert_to_string());
            println!();
            println!("Final Scores:");
            println!("    User: {}", scores.user_wins);
            println!("    Enemy: {}", scores.enemy_wins);
            break;
        }
    }
}