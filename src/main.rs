
use rps::*;

fn main() {
    println!("Welcome to rock paper scissors");
    println!("Choose your move:");
    println!("1. rock");
    println!("2. paper");
    println!("3. scissors");
    let enemy_move = enemy_move(); // generates random number for enemy move

    let user_move = handle_user_input(); // handles user input and returns an u8 for the user's move

    println!("You chose {}", convert_move_to_string(user_move));
    println!("Enemy chose {}", convert_move_to_string(enemy_move));

    let winner = check_who_wins(user_move, enemy_move); // matches user move and enemy move to winner and returns an u8 corresponding to the winner
    // 0 => tie
    // 1 => user win
    // 2 => enemy win



    println!("{}", convert_winner_to_string(winner)); // converts the winner u8 to a string
    // 0 => tie    // 0 => tie
    //     // 1 => user win
    //     // 2 => enemy win

}