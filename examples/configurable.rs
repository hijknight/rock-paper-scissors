use rock_paper_scissors::*;

fn main() {
    let mut scores = Scores::new();

    println!("Welcome to the Rock-Paper-Scissors game!");
    println!("Please enter what you want to play to: ");

    let game_settings = loop {
        match GameSettings::from_user_input() {
            Ok(game_settings) => break game_settings,
            Err(err) => println!("Error: {}", err),
        }
    };

    println!();
    let mut round_counter = 0;
    while scores.check_for_winner(&game_settings).is_err() {
        round_counter += 1;
        println!("Round {}:", round_counter);
        let player_moves = PlayerMoves::build_from_input();
        println!();
        println!("User: {}", player_moves.user_move.convert_to_string());
        println!("Enemy: {}", player_moves.enemy_move.convert_to_string());

        let round_winner = player_moves.check_who_wins_round();
        println!();
        match round_winner {
            Winner::Tie => {
                println!("Tie!");
                round_counter -= 1
            },
            Winner::User => {
                println!("User win");
                scores.user_wins += 1;
            },
            Winner::Enemy => {
                println!("Enemy win");
                scores.enemy_wins += 1;
            }
        }

        println!("Current Scores -> User: {} :: Enemy: {}", scores.user_wins, scores.enemy_wins);
    }

    let game_winner = scores.check_for_winner(&game_settings).unwrap(); // can use unwrap here because the while loop
    // will only break if the return is Ok()


    println!("Game Winner: {}", game_winner.convert_to_string());
}