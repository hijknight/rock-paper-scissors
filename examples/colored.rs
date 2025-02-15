use colored::Colorize;
use rock_paper_scissors::*;

fn main() {
    println!("{}", "Welcome to rock paper scissors!".blue().bold());

    // create game_settings struct with 3.
    let game_settings = GameSettings::from_first_to(3);

    println!("{}", "Game will be first to 3 rounds.".blue().bold());

    let mut scores = Scores::new(); // instantiate Scores struct

    let mut round_counter = 0;

    while scores.check_for_winner(&game_settings).is_err() { // checking to see if there has been a winner yet.
        round_counter += 1;
        let player_moves = PlayerMoves::build_from_input(); // build a PlayerMoves struct from input
        let round_winner = player_moves.check_who_wins_round(); // check who wins round based on moves.

        println!(
            "{}{}{}",
            "-----Round ".magenta(),
            round_counter.to_string().magenta(),
            "-----".magenta()
        );

        println!(
            "{}{}{}{}{}",
            "User: ".italic(),
            player_moves.user_move.convert_to_string().italic(),
            " :: ".italic(),
            "Enemy: ".italic(),
            player_moves.enemy_move.convert_to_string().italic()
        );


        match round_winner {
            Winner::Tie => {
                println!("{}", "It's a tie!".blue());
                round_counter -= 1;
            },
            Winner::User => {
                println!("{}", "You win!".green());
                scores.user_wins += 1;
            },
            Winner::Enemy => {
                println!("{}", "Enemy wins!".red());
                scores.enemy_wins += 1;
            }
        }

        // print scores
        println!("{}", "[Scores]".magenta().bold());
        println!("{}{}", "User: ".magenta(), scores.user_wins.to_string().magenta());
        println!("{}{}", "Enemy: ".magenta(), scores.enemy_wins.to_string().magenta());
    }


    if scores.check_for_winner(&game_settings) == Ok(Winner::User) {
        println!("{}", "You won the game!".green());
    } else {
        println!("{}", "You lost the game!".red());
    }

}
