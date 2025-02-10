
use rock_paper_scissors::*;

fn main() {
    println!("Welcome to Rock Paper Scissors");
    println!();
    println!("Choose your move:");
    println!("   1. Rock");
    println!("   2. Paper");
    println!("   3. Scissors");

    let mut scores = Scores::new();
    let mut round = 0;

    loop {
        round += 1;
        println!();
        println!("Round {}\n", round);

        let player_moves = PlayerMoves::build(); // gets user input and makes a random enemy move returns MoveType.

        let winner = player_moves.check_who_wins_round();

        println!("{} vs {} : {}", player_moves.user_move.convert_to_string(), player_moves.enemy_move.convert_to_string(), winner.convert_to_string());

        match winner {
            Winner::Tie => {
                println!("Tie! Replaying round {}", round);
                round -= 1;
                continue;
            },
            Winner::User => scores.user_wins += 1,
            Winner::Enemy => scores.enemy_wins += 1,
        }
        println!();
        println!("Scores after round {}:\n", round);
        println!("User: {}", scores.user_wins);
        println!("Enemy: {}", scores.enemy_wins);


        if let Ok(winner) = scores.check_for_winner() {
            println!("{}", winner.convert_to_string());
            break;
        }
    }
}