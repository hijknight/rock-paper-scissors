use rock_paper_scissors::{PlayerMoves, Scores, Winner};

fn main() {
    let mut scores = Scores::new();
    let first_to = 3;

    println!("Welcome to Rock-Paper-Scissors!");

    // Game loop
    while scores.check_for_winner(3).is_err() {
        let player_moves = PlayerMoves::build();

        let round_winner = player_moves.check_who_wins_round();
        println!(
            "You chose {}. Enemy chose {}.",
            player_moves.user_move.convert_to_string(),
            player_moves.enemy_move.convert_to_string(),
        );
        println!("Result: {}", round_winner.convert_to_string());

        // Update scores
        match round_winner {
            Winner::User => scores.user_wins += 1,
            Winner::Enemy => scores.enemy_wins += 1,
            Winner::Tie => (),
        }

        println!(
            "Current Scores -> You: {}, Enemy: {}",
            scores.user_wins, scores.enemy_wins
        );
    }

    // Display final results
    let game_winner = scores.check_for_winner(first_to).unwrap();
    println!("Game over! {}", game_winner.convert_to_string());
}