//! # Rock-Paper-Scissors Game Library
//!
//! Welcome to the **Rock-Paper-Scissors** game library, a simple and flexible Rust-based crate for implementing the classic "Rock, Paper, Scissors" game.
//! This crate is designed to simplify the development of both console-based and programmatic versions of the game, featuring customizable rules,
//! user input handling, game settings, and score tracking utilities.
//!
//! ## Key Features
//!
//! - **Move Management**: Enum-based moves (`Rock`, `Paper`, `Scissors`) for safe and clear gameplay logic.
//! - **Game Logic**: Easily determine winners for rounds and track scores across an entire game session.
//! - **Settings and Customization**: Includes prebuilt and customizable configurations, such as "first to X wins".
//! - **Error Handling**: Built-in validations to ensure valid moves and game states.
//! - **Randomization**: Utilities for generating random enemy moves.
//!
//! ## Core Components
//!
//! 1. **MoveType Enum**
//!    Represents the possible game moves: `Rock`, `Paper`, or `Scissors`. Includes utilities for generating randomized moves and representing moves as strings.
//!
//! 2. **Winner Enum**
//!    Encapsulates the result of each round, allowing easily distinguishable states: `User`, `Enemy`, or `Tie`.
//!
//! 3. **PlayerMoves Struct**
//!    Captures the moves made by the user and the opponent in a single round. Provides functionality to determine the winner of that round.
//!
//! 4. **Scores Struct**
//!    Tracks the cumulative scores of a session and provides methods to check if there's an overall winner based on predefined game settings.
//!
//! 5. **GameSettings Struct**
//!    Offers customizable configurations for game-winning conditions, such as "first to 3 wins" or other scenarios.
//!
//! ## Example Usage
//! Create and run a short game loop of "Rock, Paper, Scissors":
//!
//! ```no_run
//! use rock_paper_scissors::{PlayerMoves, Scores, Winner, GameSettings};
//!
//! fn main() {
//!     let mut scores = Scores::new();
//!     let game_settings = GameSettings::first_to_3();
//!
//!     while scores.check_for_winner(&game_settings).is_err() {
//!         let player_moves = PlayerMoves::build();
//!         let round_winner = player_moves.check_who_wins_round();
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
//!     let game_winner = scores.check_for_winner(&game_settings).unwrap();
//!     println!("Game Winner: {}", game_winner.convert_to_string());
//! }
//! ```
//!
//! ## Crate Philosophy
//! This library focuses on simplicity and flexibility, making it perfect for both beginner and intermediate Rust developers learning game development. It
//! provides clear interfaces and utilities while ensuring essential safeguards to avoid runtime errors.
//!
//! ## Contributing
//! Contributions such as bug fixing, feature additions, and code improvements are welcome! Please read the [contribution guidelines](#) for more details.

use rand::Rng;
use std::io;

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
/// assert_eq!(winner.convert_to_string(), "User");
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
    /// assert_eq!(winner.convert_to_string(), String::from("Enemy"));
    /// ```
    pub fn convert_to_string(&self) -> String {
        match self {
            Self::Tie => "Tie".to_string(),
            Self::User => "User".to_string(),
            Self::Enemy => "Enemy".to_string(),
        }
    }
}

/// # MoveType Enum
///
/// Represents the possible moves in the game, providing safety and clarity in game logic.
/// Now includes a `None` variant for uninitialized or invalid move states.
///
/// ## Variants
///
/// - `MoveType::Rock`: The "Rock" move.
/// - `MoveType::Paper`: The "Paper" move.
/// - `MoveType::Scissors`: The "Scissors" move.
/// - `MoveType::None`: A default state to handle uninitialized or invalid moves.
///
/// ## Key Features
///
/// - **Randomized Moves**: Generates a random move using `random_move()`.
/// - **String Conversion**: Converts moves to a user-friendly `String` using `convert_to_string()`.
/// - **Controlled User Input**: Converts valid input into a `MoveType` using `from_user_input()`.
///
/// ## Examples
///
/// ### Create a MoveType
///
/// ```rust
/// use rock_paper_scissors::MoveType;
///
/// let move_type = MoveType::Rock;
/// assert_eq!(move_type.convert_to_string(), "Rock");
/// ```
///
/// ### Generate a Random Move
///
/// ```rust
/// use rock_paper_scissors::MoveType;
///
/// let random_move = MoveType::random_move();
/// assert!(matches!(random_move, MoveType::Rock | MoveType::Paper | MoveType::Scissors));
/// ```
///
/// ### Handle Uninitialized Moves
///
/// The `None` variant is helpful when moves are not immediately set:
///
/// ```rust
/// use rock_paper_scissors::MoveType;
///
/// let move_type = MoveType::None;
/// assert_eq!(move_type.convert_to_string(), "None");
/// ```
///
/// ### Convert User Input
///
/// ```no_run
/// use rock_paper_scissors::MoveType;
///
/// println!("Enter your move: 1 (Rock), 2 (Paper), or 3 (Scissors)");
/// let user_move = MoveType::from_user_input();
///
/// match user_move {
///     Ok(move_type) => println!("You chose: {}", move_type.convert_to_string()),
///     Err(err) => println!("Error: {}", err),
/// }
/// ```
#[derive(Debug, PartialEq)]
pub enum MoveType {
    Rock,
    Paper,
    Scissors,
    None,
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
            Self::None => "None".to_string(),
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
    /// assert!(matches!(user_move, Ok(MoveType::Rock) | Ok(MoveType::Paper) | Ok(MoveType::Scissors)));
    /// ```
    ///
    /// # Behavior
    ///
    /// When the user provides invalid input (e.g., letters or numbers outside the valid range),
    /// the function will re-prompt for valid input until it is received
    pub fn from_user_input() -> Result<MoveType, String> {
        println!("Enter your move: (1 = Rock, 2 = Paper, 3 = Scissors)");
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        match user_input.trim().parse::<u8>() {
            Ok(1) => Ok(MoveType::Rock),
            Ok(2) => Ok(MoveType::Paper),
            Ok(3) => Ok(MoveType::Scissors),
            _ => Err("Invalid input. Please enter 1, 2, or 3.".to_string()),
        }
    }
}


/// # PlayerMoves Struct
///
/// Represents the moves made by the user and the opponent in a single round of the game.
///
/// ## Fields
///
/// - `user_move`:
///   - The move chosen by the user (of type `MoveType`).
///   - Defaults to `MoveType::None` when uninitialized.
/// - `enemy_move`:
///   - The move chosen by the opponent (of type `MoveType`).
///   - Defaults to `MoveType::None` when uninitialized.
///
/// ## Methods
///
/// - **`PlayerMoves::new()`**: Safely initializes a `PlayerMoves` instance with both moves set to `MoveType::None`.
/// - **`PlayerMoves::build()`**: Builds a new `PlayerMoves` instance by getting the user's input and randomly generating the enemy's move.
/// - **`PlayerMoves::check_who_wins_round()`**: Determines the winner of the round based on the moves.
///
/// ## Examples
///
/// ### Initialize PlayerMoves with Default Values
/// ```rust
/// use rock_paper_scissors::{PlayerMoves, MoveType};
///
/// let moves = PlayerMoves::new();
/// assert_eq!(moves.user_move, MoveType::None);
/// assert_eq!(moves.enemy_move, MoveType::None);
/// ```
///
/// ### Create and Compare Moves
/// ```rust
/// use rock_paper_scissors::{PlayerMoves, MoveType, Winner};
///
/// let moves = PlayerMoves {
///     user_move: MoveType::Rock,
///     enemy_move: MoveType::Scissors,
/// };
///
/// assert_eq!(moves.check_who_wins_round(), Winner::User);
/// ```
#[derive(Debug, PartialEq)]
pub struct PlayerMoves {
    pub user_move: MoveType,
    pub enemy_move: MoveType,
}

impl PlayerMoves {

    /// Creates a new `PlayerMoves` instance with both the `user_move` and `enemy_move` set to `MoveType::None`.
    ///
    /// This provides a safe starting state where moves can be explicitly set at a later stage.
    /// It is particularly useful for initializing structures in cases where moves are assigned dynamically (e.g., during gameplay).
    ///
    /// ## Examples
    ///
    /// ### Default Initialization
    /// ```rust
    /// use rock_paper_scissors::{PlayerMoves, MoveType};
    ///
    /// let moves = PlayerMoves::new();
    /// assert_eq!(moves.user_move, MoveType::None);
    /// assert_eq!(moves.enemy_move, MoveType::None);
    /// ```
    pub fn new() -> PlayerMoves {
        PlayerMoves {
            user_move: MoveType::None,
            enemy_move: MoveType::None,
        }
    }

    /// Builds a new `PlayerMoves` instance with a random enemy move and the user's move provided via input.
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
    /// // let player_moves = PlayerMoves::build();
    ///
    /// // the build function takes user input, so this is an example output.
    ///
    /// let player_moves = PlayerMoves {
    ///     user_move: MoveType::Rock,
    ///     enemy_move: MoveType::Scissors,
    /// };
    ///
    /// assert_eq!(player_moves.user_move, MoveType::Rock);
    /// assert_eq!(player_moves.enemy_move, MoveType::Scissors);
    /// ```
    pub fn build() -> PlayerMoves {
        let user_move = loop {
            match MoveType::from_user_input() {
                Ok(move_type) => break move_type,
                Err(e) => println!("{}", e),
            }
        };

        PlayerMoves {
            user_move,
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

    /// Checks if the game has a winner (first to however many wins).
    ///
    /// If either the user or the enemy has a certain number of specified wins, returns the winner as `Ok(Winner)`. Otherwise, returns an `Err` type.a
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rock_paper_scissors::{Scores, Winner, GameSettings};
    ///
    /// let game_settings: GameSettings = GameSettings::first_to_3();
    ///
    /// let scores = Scores {
    ///     user_wins: 3,
    ///     enemy_wins: 2,
    /// };
    ///
    /// assert_eq!(scores.check_for_winner(&game_settings), Ok(Winner::User));
    /// ```
    pub fn check_for_winner(&self, game_settings: &GameSettings) -> Result<Winner, &str> {
        if self.user_wins == game_settings.first_to {
            Ok(Winner::User)
        } else if self.enemy_wins == game_settings.first_to {
            Ok(Winner::Enemy)
        } else {
            Err("rock-paper-scissors: err: No winner yet")
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

/// # GameSettings Struct
///
/// The `GameSettings` struct provides a simple yet flexible mechanism to configure the win conditions for a "Rock, Paper, Scissors" game session.
/// It allows developers to define the number of wins required to declare an overall game winner.
///
/// ## Fields
///
/// - `first_to`
///   - Specifies the number of round wins required for either the user or opponent to win the game.
///   - This value defaults to `0` when initializing using `GameSettings::new()`.
///
/// ## Methods
///
/// ### `GameSettings::new()`
/// Creates a new `GameSettings` instance with `first_to` set to `0`. This can act as a placeholder until specific settings are defined.
///
/// ```rust
/// use rock_paper_scissors::GameSettings;
///
/// let game_settings = GameSettings::new();
/// assert_eq!(game_settings.first_to, 0);
/// ```
///
/// ### `GameSettings::first_to_3()`
/// Provides a predefined configuration where the game is set to end after 3 wins from either the user or the opponent.
///
/// ```rust
/// use rock_paper_scissors::GameSettings;
///
/// let game_settings = GameSettings::first_to_3();
/// assert_eq!(game_settings.first_to, 3);
/// ```
///
/// ## Examples
///
/// ### Using Custom Win Conditions
/// Developers can define their own win conditions by directly instantiating the `GameSettings` struct:
///
/// ```rust
/// use rock_paper_scissors::GameSettings;
///
/// let custom_game_settings = GameSettings {
///     first_to: 5,
/// };
///
/// assert_eq!(custom_game_settings.first_to, 5);
/// ```
///
/// ### Combining with Scores
/// The `GameSettings` struct is designed to work seamlessly with the `Scores` struct to determine if a game session has reached its end:
///
/// ```rust
/// use rock_paper_scissors::{Scores, GameSettings, Winner};
///
/// let game_settings = GameSettings::first_to_3();
/// let mut scores = Scores::new();
///
/// // Simulate some rounds
/// scores.user_wins = 3;
///
/// // Check for game winner
/// let winner = scores.check_for_winner(&game_settings);
/// assert_eq!(winner, Ok(Winner::User));
/// ```
#[derive(Debug, PartialEq)]
pub struct GameSettings {
    pub first_to: u8,
}

impl GameSettings {
    /// Creates a new game configuration with the default `first_to` value of `0`.
    ///
    /// ## Examples
    /// ```rust
    /// use rock_paper_scissors::GameSettings;
    ///
    /// let settings = GameSettings::new();
    /// assert_eq!(settings.first_to, 0);
    /// ```
    pub fn new() -> GameSettings {
        GameSettings { first_to: 0 }
    }

    /// Prebuilt configuration where the first player to win 3 rounds is declared the winner.
    ///
    /// ## Examples
    /// ```rust
    /// use rock_paper_scissors::GameSettings;
    ///
    /// let settings = GameSettings::first_to_3();
    /// assert_eq!(settings.first_to, 3);
    /// ```
    pub fn first_to_3() -> GameSettings {
        GameSettings { first_to: 3 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_for_winner_works() {

        let game_settings = GameSettings::first_to_3();
        let scores = Scores {
            user_wins: 3,
            enemy_wins: 1,
        };

        assert_eq!(scores.check_for_winner(&game_settings), Ok(Winner::User));

        let scores = Scores {
            user_wins: 2,
            enemy_wins: 3
        };

        assert_eq!(scores.check_for_winner(&game_settings), Ok(Winner::Enemy));

        let scores = Scores {
            user_wins: 1,
            enemy_wins: 0,
        };

        assert_eq!(scores.check_for_winner(&game_settings), Err("rock-paper-scissors: err: No winner yet"));
    }

    #[test]
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

        assert_eq!(winner.convert_to_string(), "Tie");

        let winner = Winner::User;

        assert_eq!(winner.convert_to_string(), "User");

        let winner = Winner::Enemy;

        assert_eq!(winner.convert_to_string(), "Enemy");
    }

    #[test]
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

    #[test]
    fn check_player_moves_new() {
        let player_moves = PlayerMoves::new();

        assert_eq!(player_moves, PlayerMoves {
            user_move: MoveType::None,
            enemy_move: MoveType::None,
        })
    }
}