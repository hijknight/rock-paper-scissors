//! # Rock-paper-scissors
//!
//! `rock-paper-scissors` is an open-source Rust game API that allows users to create custom implementations of the classic game "Rock, Paper, Scissors".
//!
//! ## Features
//!
//! - Fully customizable logic for handling moves and determining winners.
//! - Built-in functions to simplify gameplay mechanics, such as move generation, winner determination, and score tracking.
//! - Enum-based design for safety and clarity.
//! - End-to-end examples and tests for ease of use and reliability.
//!
//! ## Crate Structure
//!
//! This crate provides the following core components:
//!
//! 1. **Winner Enum**
//!    Represents the result of a game round or the match.
//!    - `Winner::Tie`: Indicates a tie between the user and the enemy.
//!    - `Winner::User`: Indicates a win for the user.
//!    - `Winner::Enemy`: Indicates a win for the enemy.
//!
//!    ```rust
//!    use rock_paper_scissors::Winner;
//!
//!    let winner = Winner::User;
//!    println!("{}", winner.convert_to_string()); // "You win!"
//!    ```
//!
//! 2. **MoveType Enum**
//!    Represents the possible moves in the game: `Rock`, `Paper`, or `Scissors`. Includes utilities for creating random moves and converting moves to printable `String` values.
//!
//!    ```rust
//!    use rock_paper_scissors::MoveType;
//!
//!    let random_move = MoveType::random_move(); // Generates a random move
//!    let move_name = random_move.convert_to_string(); // Converts the move to a string
//!    println!("{}", move_name); // e.g., "Rock"
//!    ```
//!
//! 3. **PlayerMoves Struct**
//!    Bundles the user and enemy moves for a round. Also provides functionality to determine the round winner based on the moves.
//!
//!    ```rust
//!    use rock_paper_scissors::{PlayerMoves, MoveType, Winner};
//!
//!    let player_moves = PlayerMoves {
//!        user_move: MoveType::Rock,
//!        enemy_move: MoveType::Scissors,
//!    };
//!
//!    let round_winner = player_moves.check_who_wins_round();
//!    assert_eq!(round_winner, Winner::User);
//!    ```
//!
//! 4. **Scores Struct**
//!    Tracks the cumulative scores for the user and enemy in a game session. Allows checking whether the game has a winner based on predefined win conditions (e.g., first to 3 wins).
//!
//!    ```rust
//!    use rock_paper_scissors::{Scores, Winner};
//!
//!    let mut scores = Scores::new();
//!    scores.user_wins += 1;
//!
//!    let winner = scores.check_for_winner();
//!    assert!(winner.is_err()); // No winner yet
//!    ```
//!
//! 5. **User Input and Integration**
//!    Provides utilities to collect and validate user input for moves in a console-based setup.
//!
//!    ```rust
//!    use rock_paper_scissors::{PlayerMoves, MoveType};
//!
//!    let user_move = MoveType::from_user_input();
//!    println!("You chose: {}", user_move.convert_to_string());
//!    ```
//!
//! ## Example Usage
//!
//! Below is an example of a simple game loop using the `rock-paper-scissors` library:
//!
//! ```rust
//! use rock_paper_scissors::{ PlayerMoves, Scores, Winner };
//!
//! fn main() {
//!     let mut scores = Scores::new();
//!
//!     while scores.check_for_winner().is_err() {
//!         let player_moves = PlayerMoves::new();
//!
//!         let round_winner = player_moves.check_who_wins_round();
//!         println!("Round result: {}", round_winner.convert_to_string());
//!
//!         match round_winner {
//!             Winner::User => scores.user_wins += 1,
//!             Winner::Enemy => scores.enemy_wins += 1,
//!             Winner::Tie => println!("It's a tie!"),
//!         }
//!
//!         println!("Scores -> User: {}, Enemy: {}", scores.user_wins, scores.enemy_wins);
//!     }
//!
//!     let game_winner = scores.check_for_winner().unwrap();
//!     println!("Game Over! Winner: {}", game_winner.convert_to_string());
//! }
//! ```
//!
//! ## Error Handling
//!
//! The crate provides robust error handling for invalid user moves and incomplete game states:
//! - Ensures valid move input from users.
//! - Fails gracefully with descriptive errors when conditions (like 3 wins required to finish the game) are not met.

use rand::Rng;
use std::io;
use std::io::Write;

/// # Winner enum
///
/// Represents the different results of a game round.
///
/// - `Winner::Tie`: Indicates a tie between the user and the enemy.
/// - `Winner::User`: Indicates that the user has won the round.
/// - `Winner::Enemy`: Indicates that the enemy has won the round.
///
/// # Examples
///
/// ```rust
/// use rock_paper_scissors::Winner;
///
/// let winner = Winner::User;
/// assert_eq!(winner.convert_to_string(), "You win!");
/// ```
#[derive(Debug, PartialEq)]
pub enum Winner {
    Tie,
    User,
    Enemy,
}


impl Winner {
    /// Converts a `Winner` to a human-readable `String`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rock_paper_scissors::Winner;
    ///
    /// let winner = Winner::Enemy;
    /// assert_eq!(winner.convert_to_string(), "You lose!");
    /// ```
    pub fn convert_to_string(&self) -> String {
        match self {
            Self::Tie => "Tie!".to_string(),
            Self::User => "You win!".to_string(),
            Self::Enemy => "You lose!".to_string(),
        }
    }
}

/// # MoveType enum
///
/// Represents the possible moves in the game: `Rock`, `Paper`, or `Scissors`.
///
/// # Examples
///
/// ```rust
/// use rock_paper_scissors::MoveType;
///
/// let new_move = MoveType::Rock;
/// assert_eq!(new_move.convert_to_string(), "Rock");
///
/// let random_move = MoveType::random_move();
/// assert!(matches!(random_move, MoveType::Rock | MoveType::Paper | MoveType::Scissors));
/// ```
#[derive(Debug, PartialEq)]
pub enum MoveType {
    Rock,
    Paper,
    Scissors,
}

impl MoveType {
    /// Generates a random `MoveType` (one of `Rock`, `Paper`, or `Scissors`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rock_paper_scissors::MoveType;
    ///
    /// let random_move = MoveType::random_move();
    /// assert!(matches!(random_move, MoveType::Rock | MoveType::Paper | MoveType::Scissors));
    /// ```
    pub fn random_move() -> MoveType {
        let rand_num = rand::rng().random_range(1..=3);

        match rand_num {
            1 => MoveType::Rock,
            2 => MoveType::Paper,
            _ => MoveType::Scissors,
        }
    }

    /// Converts the `MoveType` to a human-readable `String`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rock_paper_scissors::MoveType;
    ///
    /// let move_type = MoveType::Scissors;
    /// assert_eq!(move_type.convert_to_string(), "Scissors");
    /// ```
    pub fn convert_to_string(&self) -> String {
        match self {
            Self::Rock => "Rock".to_string(),
            Self::Paper => "Paper".to_string(),
            Self::Scissors => "Scissors".to_string(),
        }
    }

    /// # Gets the User's Move
    ///
    /// Handles user input from the console, validates it, and converts it into a `MoveType`.
    ///
    /// The user is prompted to enter a number corresponding to their move:
    /// - `1` for `Rock`
    /// - `2` for `Paper`
    /// - `3` for `Scissors`
    ///
    /// The function will ensure valid input by repeatedly asking for input until a valid value is provided.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rock_paper_scissors::MoveType;
    ///
    /// println!("Enter your move: (1 = Rock, 2 = Paper, 3 = Scissors)");
    /// let user_move = MoveType::from_user_input();
    ///
    /// assert!(matches!(user_move, MoveType::Rock | MoveType::Paper | MoveType::Scissors));
    /// ```
    ///
    /// # Behavior
    ///
    /// When the user provides invalid input (e.g., letters or numbers outside the valid range),
    /// the function will re-prompt for valid input until it is received
    pub fn from_user_input() -> MoveType {
        loop {
            print!("Enter your move: (1 = Rock, 2 = Paper, 3 = Scissors): ");
            io::stdout().flush().unwrap(); // make sure it is printed before input is given

            let mut user_move: String = String::new();
            io::stdin()
                .read_line(&mut user_move)
                .expect("Failed to read line");

            match user_move.trim().parse::<u8>() {
                Ok(1) => return MoveType::Rock,
                Ok(2) => return MoveType::Paper,
                Ok(3) => return MoveType::Scissors,
                Ok(_) => println!("Please enter a valid number between 1 and 3."),
                _ => println!("Please enter a valid number."),
            }
        }
    }
}

/// # PlayerMoves struct
///
/// Represents the moves made by the user and the enemy in a single round of the game.
///
/// - `user_move`: Move made by the user.
/// - `enemy_move`: Move made by the enemy.
///
/// # Examples
///
/// ```rust
/// use rock_paper_scissors::{PlayerMoves, MoveType, Winner};
///
/// let player_moves = PlayerMoves {
///     user_move: MoveType::Rock,
///     enemy_move: MoveType::Scissors,
/// };
///
/// assert_eq!(player_moves.check_who_wins_round(), Winner::User);
/// ```
#[derive(Debug, PartialEq)]
pub struct PlayerMoves {
    pub user_move: MoveType,
    pub enemy_move: MoveType,
}

impl PlayerMoves {
    /// Creates a new `PlayerMoves` instance with a random enemy move and the user's move provided via input.
    ///
    /// # Warning
    ///
    /// Requires `get_user_move` to be used for user input.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rock_paper_scissors::{PlayerMoves, MoveType};
    ///
    /// let player_moves = PlayerMoves {
    ///     user_move: MoveType::Rock,
    ///     enemy_move: MoveType::Scissors,
    /// };
    ///
    /// assert_eq!(player_moves.user_move, MoveType::Rock);
    /// assert_eq!(player_moves.enemy_move, MoveType::Scissors);
    /// ```
    pub fn new() -> PlayerMoves {
        PlayerMoves {
            user_move: MoveType::from_user_input(),
            enemy_move: MoveType::random_move(),
        }
    }

    /// Determines the winner of the round based on the user's and enemy's moves.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rock_paper_scissors::{PlayerMoves, MoveType, Winner};
    ///
    /// let player_moves = PlayerMoves {
    ///     user_move: MoveType::Paper,
    ///     enemy_move: MoveType::Rock,
    /// };
    ///
    /// assert_eq!(player_moves.check_who_wins_round(), Winner::User);
    /// ```
    pub fn check_who_wins_round(&self) -> Winner {
        match (&self.user_move, &self.enemy_move) {
            (MoveType::Rock, MoveType::Rock) | (MoveType::Paper, MoveType::Paper) | (MoveType::Scissors, MoveType::Scissors) => Winner::Tie,
            (MoveType::Rock, MoveType::Scissors) | (MoveType::Paper, MoveType::Rock) | (MoveType::Scissors, MoveType::Paper) => Winner::User,
            _ => Winner::Enemy,
        }
    }
}

/// # Scores struct
///
/// Represents the current scores for both the user and the enemy in a game session.
///
/// - `user_wins`: Number of rounds won by the user.
/// - `enemy_wins`: Number of rounds won by the enemy.
///
/// # Examples
///
/// ```rust
/// use rock_paper_scissors::Scores;
///
/// let mut scores = Scores::new();
/// scores.user_wins += 1;
/// assert_eq!(scores.user_wins, 1);
/// assert_eq!(scores.enemy_wins, 0);
/// ```
#[derive(Debug, PartialEq)]
pub struct Scores {
    pub user_wins: u8,
    pub enemy_wins: u8,
}

impl Scores {
    /// Creates a new `Scores` instance with zero scores.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rock_paper_scissors::Scores;
    ///
    /// let scores = Scores::new();
    /// assert_eq!(scores.user_wins, 0);
    /// assert_eq!(scores.enemy_wins, 0);
    /// ```
    pub fn new() -> Scores {
        Scores {
            user_wins: 0,
            enemy_wins: 0,
        }
    }

    /// Checks if the game has a winner (first to 3 wins).
    ///
    /// If either the user or the enemy has 3 wins, returns the winner as `Ok(Winner)`. Otherwise, returns an error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rock_paper_scissors::{Scores, Winner};
    ///
    /// let scores = Scores {
    ///     user_wins: 3,
    ///     enemy_wins: 2,
    /// };
    ///
    /// assert_eq!(scores.check_for_winner(), Ok(Winner::User));
    /// ```
    pub fn check_for_winner(&self) -> Result<Winner, &str> {
        if self.user_wins == 3 {
            Ok(Winner::User)
        } else if self.enemy_wins == 3 {
            Ok(Winner::Enemy)
        } else {
            Err("No winner yet")
        }
    }

    /// Resets the scores to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rock_paper_scissors::Scores;
    ///
    /// let mut scores = Scores {
    ///     user_wins: 2,
    ///     enemy_wins: 3,
    /// };
    ///
    /// scores.reset();
    /// assert_eq!(scores.user_wins, 0);
    /// assert_eq!(scores.enemy_wins, 0);
    /// ```
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.user_wins = 0;
        self.enemy_wins = 0;
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

        assert_eq!(scores.check_for_winner(), Ok(Winner::User));

        let scores = Scores {
            user_wins: 2,
            enemy_wins: 3
        };

        assert_eq!(scores.check_for_winner(), Ok(Winner::Enemy));

        let scores = Scores {
            user_wins: 1,
            enemy_wins: 0,
        };

        assert_eq!(scores.check_for_winner(), Err("No winner yet"));
    }

    #[test]
    // TODO: Fix this test
    fn check_who_wins_round_works() {
        let player_moves = PlayerMoves {
            user_move: MoveType::Rock,
            enemy_move: MoveType::Rock,
        };

        assert_eq!(player_moves.check_who_wins_round(), Winner::Tie);

        let player_moves = PlayerMoves {
            user_move: MoveType::Paper,
            enemy_move: MoveType::Rock,
        };

        assert_eq!(player_moves.check_who_wins_round(), Winner::User);

        let player_moves = PlayerMoves {
            user_move: MoveType::Paper,
            enemy_move: MoveType::Scissors,
        };

        assert_eq!(player_moves.check_who_wins_round(), Winner::Enemy);
    }

    #[test]
    fn convert_winner_to_string_works() {
        let winner = Winner::Tie;

        assert_eq!(winner.convert_to_string(), "Tie!");

        let winner = Winner::User;

        assert_eq!(winner.convert_to_string(), "You win!");

        let winner = Winner::Enemy;

        assert_eq!(winner.convert_to_string(), "You lose!");
    }

    #[test]
    // TODO: Fix this test
    fn convert_move_to_string_works() {
        let move_type = MoveType::Rock;

        assert_eq!(move_type.convert_to_string(), "Rock");

        let move_type = MoveType::Paper;

        assert_eq!(move_type.convert_to_string(), "Paper");

        let move_type = MoveType::Scissors;

        assert_eq!(move_type.convert_to_string(), "Scissors");
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