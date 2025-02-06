
use rand::Rng;
use std::io;


/// Checks who wins
/// ---
/// ### Moves:
/// - `1`: rock
/// - `2`: paper
/// - `3`: scissors
///
/// ### (user_move, enemy_move)
/// ### Matches:
/// **Ties:**
/// `(1, 1) | (2, 2) | (3, 3)` => `0` for tie
/// **User wins:**
/// `(1, 3) | (2, 1) | (3, 2)` => `1` for user_win
/// **Enemy wins:**
/// `(3, 1) | (1, 2) | (2, 3)` => `2` for enemy_win
///
/// All other possibilities are represented using `_`.
pub fn check_who_wins(user_move: u8, enemy_move: u8) -> u8 {
    match (user_move, enemy_move) {
        (1, 1) | (2, 2) | (3, 3) => 0,
        (1, 3) | (2, 1) | (3, 2) => 1,
        _ => 2,
    }
}

pub fn enemy_move() -> u8 {
    rand::rng().random_range(1..=3)
}

pub fn convert_move_to_string(move_number: u8) -> String {
    match move_number {
        1 => "rock".to_string(),
        2 => "paper".to_string(),
        3 => "scissors".to_string(),
        _ => "invalid move".to_string(),
    }
}

pub fn handle_user_input() -> u8 {
    let mut user_move: String = String::new();

    io::stdin()
        .read_line(&mut user_move)
        .expect("Failed to read line");

    let user_move: u8 = user_move.trim().parse().expect("Please enter a valid number.");

    user_move
}

pub fn convert_winner_to_string(winner: u8) -> String {
    match winner {
        0 => "Tie!".to_string(),
        1 => "You win!".to_string(),
        _ => "You lose!".to_string(),
    }
}