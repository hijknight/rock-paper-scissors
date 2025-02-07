
use rand::Rng;
use std::io;

// main winner enum, all determine/check winner function will return this
#[derive(Debug, PartialEq)]
pub enum Winner {
    Tie,
    User,
    Enemy,
}

// used for determining winner and keeping track of the state of the game.

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Rock,
    Paper,
    Scissors,
}


#[derive(Debug, PartialEq)]
pub struct Scores {
    pub user_wins: u8,
    pub enemy_wins: u8,
}

impl Scores {
    pub fn new() -> Scores {
        Scores {
            user_wins: 0,
            enemy_wins: 0,
        }
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.user_wins = 0;
        self.enemy_wins = 0;
    }
}

/// Checks who wins
/// ---
/// ### Moves:
/// - `Move::Rock`: rock
/// - `Move::Paper`: paper
/// - `Move::Scissors`: scissors
///
/// ### (user_move, enemy_move)
/// ### Matches:
/// **Ties:**
///
/// `(Move::Rock, Move::Rock) | (Move::Paper, Move::Paper) | (Move::Scissors, Move::Scissors)` => `Winner::Tie` for tie
///
/// **User wins:**
///
/// `(Move::Rock, Move::Scissors) | (Move::Paper, Move::Rock) | (Move::Scissors, Move::Paper)` => `Winner::User` for user_win
///
/// **Enemy wins:**
///
/// `(Move::Scissors, Move::Rock) | (Move::Rock, Move::Paper) | (Move::Paper, Move::Scissors)` => `Winner::Enemy` for enemy_win
///
/// All other possibilities are represented using `_`.
pub fn check_who_wins(user_move: &MoveType, enemy_move: &MoveType) -> Winner {
    match (user_move, enemy_move) {
        (MoveType::Rock, MoveType::Rock) | (MoveType::Paper, MoveType::Paper) | (MoveType::Scissors, MoveType::Scissors) => Winner::Tie,
        (MoveType::Rock, MoveType::Scissors) | (MoveType::Paper, MoveType::Rock) | (MoveType::Scissors, MoveType::Paper) => Winner::User,
        _ => Winner::Enemy,
    }
}

pub fn enemy_move() -> MoveType {
    let rand_num = rand::rng().random_range(1..=3);

    match rand_num {
        1 => MoveType::Rock,
        2 => MoveType::Paper,
        _ => MoveType::Scissors,
    }
}

pub fn convert_move_to_string(move_type: &MoveType) -> String {
    match move_type {
        MoveType::Rock => "Rock".to_string(),
        MoveType::Paper => "Paper".to_string(),
        MoveType::Scissors => "Scissors".to_string(),
    }
}

pub fn get_user_move() -> MoveType {
    let mut user_move: String = String::new();

    io::stdin()
        .read_line(&mut user_move)
        .expect("Failed to read line");

    // get initial user input and parse into an integer,
    // if this returns an Err, the while let below will run and retry handling user input.
    let user_move: Result<u8, _> = user_move.trim().parse();

    while let Err(_) = user_move { // makes sure unwrap cannot panic
        println!("Please enter a valid number.");
        get_user_move();
    }

    // We can use unwrap here because the while loop has already run, so we know this won't panic
    let user_move = user_move.unwrap();

    // finally, after checking   that the value is Ok(u8) and between 1 and 3, we can return the user move statement.
    match user_move {
        1..=3 => {
            match user_move {
                1 => MoveType::Rock,
                2 => MoveType::Paper,
                _ => MoveType::Scissors,
            }
        },
        _ => {
            println!("Please enter a number between 1 and 3.");
            get_user_move()
        }
    }
}

pub fn convert_winner_to_string(winner: &Winner) -> String {
    match winner {
        Winner::Tie => "Tie!".to_string(),
        Winner::User => "You win!".to_string(),
        Winner::Enemy => "You lose!".to_string(),
    }
}

pub fn check_for_winner(scores: &Scores) -> Result<Winner, &str> {
    if scores.user_wins == 3 {
        Ok(Winner::User)
    } else if scores.enemy_wins == 3 {
        Ok(Winner::Enemy)
    } else {
        Err("No winner yet")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_for_winner_works() {
        let scores = Scores {
            user_wins: 3,
            enemy_wins: 1,
        };

        assert_eq!(check_for_winner(&scores), Ok(Winner::User));

        let scores = Scores {
            user_wins: 2,
            enemy_wins: 3
        };

        assert_eq!(check_for_winner(&scores), Ok(Winner::Enemy));

        let scores = Scores {
            user_wins: 1,
            enemy_wins: 0,
        };

        assert_eq!(check_for_winner(&scores), Err("No winner yet"));
    }

    #[test]
    pub fn check_who_wins_works() {
        // should tie
        let some_user_move: MoveType = MoveType::Rock;
        let some_enemy_move: MoveType = MoveType::Rock;
        assert_eq!(check_who_wins(&some_user_move, &some_enemy_move), Winner::Tie);

        // user should win
        let some_user_move: MoveType = MoveType::Rock; // rock smashes scissors
        let some_enemy_move: MoveType = MoveType::Scissors;
        assert_eq!(check_who_wins(&some_user_move, &some_enemy_move), Winner::User);

        // enemy should win
        let some_user_move: MoveType = MoveType::Rock;
        let some_enemy_move: MoveType = MoveType::Paper; // paper covers rock
        assert_eq!(check_who_wins(&some_user_move, &some_enemy_move), Winner::Enemy);
    }

    #[test]
    pub fn convert_winner_to_string_works() {
        let winner = Winner::Tie;

        assert_eq!(convert_winner_to_string(&winner), "Tie!");

        let winner = Winner::User;

        assert_eq!(convert_winner_to_string(&winner), "You win!");

        let winner = Winner::Enemy;

        assert_eq!(convert_winner_to_string(&winner), "You lose!");
    }

    #[test]
    pub fn convert_move_to_string_works() {
        let move_type = MoveType::Rock;

        assert_eq!(convert_move_to_string(&move_type), "Rock");

        let move_type = MoveType::Paper;

        assert_eq!(convert_move_to_string(&move_type), "Paper");

        let move_type = MoveType::Scissors;

        assert_eq!(convert_move_to_string(&move_type), "Scissors");
    }

    #[test]
    fn check_scores_new() {
        let scores = Scores::new();

        assert_eq!(scores, Scores {
            user_wins: 0,
            enemy_wins: 0,
        });
    }
}