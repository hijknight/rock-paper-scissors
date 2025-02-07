//! rock-paper-scissors is an open-source game-api for rust that allows users to make custom implementations of rock paper scissors.
use rand::Rng;
use std::io;

/// # Winner enum
///
/// Main winner enum, all determine/check winner functions will return this.
///
/// # Example:
///
/// ```rust
/// use rock_paper_scissors::Winner;
///
/// let winner = Winner::User;
///
/// ```
#[derive(Debug, PartialEq)]
pub enum Winner {
    Tie,
    User,
    Enemy,
}

impl Winner {

    /// Converts Winner type to String
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
/// Ensures only Rock, Paper, or Scissors are the only possible moves.
///
/// # Example:
///
/// ```rust
/// use rock_paper_scissors::MoveType;
///
/// let new_move = MoveType::Paper;
///
/// ```
#[derive(Debug, PartialEq)]
pub enum MoveType {
    Rock,
    Paper,
    Scissors,
}

impl MoveType {
    /// # Creates new random `MoveType`
    ///
    /// Can either be `MoveType::Rock`, `MoveType::Paper`, or `MoveType::Scissors`
    ///
    /// ```rust
    /// use rock_paper_scissors::MoveType;
    ///
    /// let random_move = MoveType::random_move();
    /// ```
    pub fn random_move() -> MoveType {
        let rand_num = rand::rng().random_range(1..=3);

        match rand_num {
            1 => MoveType::Rock,
            2 => MoveType::Paper,
            _ => MoveType::Scissors,
        }
    }

    /// # Converts `MoveType` to printable `String`
    ///
    /// Takes a reference to `MoveType` type, and depending on which type it is, returns a `String` with that type.
    ///
    /// Does important error handling for making sure input is valid.
    ///
    /// ```rust
    /// use rock_paper_scissors::MoveType;
    /// let new_move = MoveType::Paper;
    ///
    /// let converted_move: String = new_move.convert_to_string();
    ///
    /// // will print "Paper"
    pub fn convert_to_string(&self) -> String {
        match self {
            Self::Rock => "Rock".to_string(),
            Self::Paper => "Paper".to_string(),
            Self::Scissors => "Scissors".to_string(),
        }
    }


}

#[derive(Debug, PartialEq)]
pub struct PlayerMoves {
    pub user_move: MoveType,
    pub enemy_move: MoveType,
}

impl PlayerMoves {
    // as of right now, must import entire get_user_move() to use this.
    pub fn new() -> PlayerMoves {
        PlayerMoves {
            user_move: get_user_move(),
            enemy_move: MoveType::random_move(),
        }
    }

    /// # Determines who wins a round
    ///
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
/// Used for keeping track of the game and determining winner.
///
/// # Example:
///
/// ```rust
/// use rock_paper_scissors::Scores;
///
/// let scores = Scores {
///     user_wins: 0,
///     enemy_wins: 3,
/// };
/// ```
#[derive(Debug, PartialEq)]
pub struct Scores {
    pub user_wins: u8,
    pub enemy_wins: u8,
}



impl Scores {
    /// creates an empty Scores struct
    ///
    /// like so:
    ///
    /// ```rust
    /// use rock_paper_scissors::Scores;
    /// let scores = Scores::new();
    ///
    /// assert_eq!(scores, Scores {
    ///     user_wins: 0,
    ///     enemy_wins: 0,
    /// })
    /// ```
    pub fn new() -> Scores {
        Scores {
            user_wins: 0,
            enemy_wins: 0,
        }
    }

    /// # Determines the winner of a game
    ///
    /// Takes a reference to a `Scores` struct and returns a `Result<Winner, &str>`.
    ///
    /// The function checks if there is a winner by seeing if either the User or Enemy has 3 wins. If there is a winner possible, it returns the `Winner` type wrapped in Ok()
    ///
    /// If there is no winner possible it returns an `Err("No winner yet")` this `Err` type can be used in multiple ways.
    ///
    /// ### Example implementation:
    ///
    /// ```
    /// loop {
    ///     use rock_paper_scissors::{
    ///         Winner,
    ///         Scores,
    ///     };
    ///     let scores = Scores {
    ///         user_wins: 3,
    ///         enemy_wins: 2,
    ///     };
    ///
    ///     let winner: Result<Winner, _> = scores.check_for_winner();
    ///      // assert_eq!(Ok(Winner::User), winner)
    /// }
    /// ```
    /// This implementation checks if there is winner with the `if let` statement, and then prints the winner using the `convert_winner_to_string(winner: Winner) {}`
    /// function in the rps library.
    pub fn check_for_winner(&self) -> Result<Winner, &str> {
        if self.user_wins == 3 {
            Ok(Winner::User)
        } else if self.enemy_wins == 3 {
            Ok(Winner::Enemy)
        } else {
            Err("No winner yet")
        }
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.user_wins = 0;
        self.enemy_wins = 0;
    }
}



/// # Gets the User's Move
///
/// Handles user input and parse into an integer then matches to a `MoveType` type.
///
/// ```rust
/// use rock_paper_scissors::{PlayerMoves, MoveType, get_user_move};
///
/// let player_moves = PlayerMoves {
///     user_move: get_user_move(),
///     enemy_move: MoveType::Rock,
/// };
/// ```
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