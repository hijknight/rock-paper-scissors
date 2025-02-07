# rock-paper-scissors

`rock-paper-scissors` is an open-source Rust library and interactive game designed for developers to create or customize implementations of the classic "Rock, Paper, Scissors" game. It adheres to **clean design principles**, offering modular functionality and robust error handling.

### [rock-paper-scissors Documentation on docs.rs](https://docs.rs/rock-paper-scissors/)

---

## Features

### Library Highlights
- **Customizable Game Logic**:
  - Enums and structs encapsulate game logic for modularity and easy customization.
  - Functions like `MoveType::random_move` and `MoveType::from_user_input` simplify both randomness and interactivity.
- **Winner Determination**:
  - Built-in logic determines the winner of each round with clear rules (`PlayerMoves::check_who_wins_round`).
- **Score Management**:
  - Tracks user and enemy wins during the game with a `Scores` struct.
  - Allows checking the game's end (`Scores::check_for_winner`) or resetting (`Scores::reset`).
- **Friendly Output Utilities**:
  - Human-readable string conversion for enums like `MoveType` and `Winner` provides better UI/UX.

### Game Highlights
- Interactive gameplay through `MoveType::from_user_input` for user moves.
- Head-to-head matches with a randomly generated opponent.
- Input validation ensures smooth, error-free gameplay.

---

## Getting Started

### Prerequisites

To use or play the `rock-paper-scissors` library, ensure you have the following installed:

- Rust (v1.64 or higher)
- Cargo, for building and running the library or game.

### Installation

To use the `rock-paper-scissors` library in your project, include it in your `Cargo.toml`:

```toml
[dependencies]
rock-paper-scissors = "0.1.0"
```

For the interactive game:

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo-name/rock-paper-scissors.git
   cd rock-paper-scissors
   ```
2. Run the game using Cargo:
   ```bash
   cargo run
   ```

---

## Library Overview

### Core Components

#### **Enums**

1. **Winner**
  - Represents round outcomes: `User`, `Enemy`, or `Tie`.
  - Includes `Winner::convert_to_string` for clear textual output.  
    Example output: `"You win!"` or `"You lose!"`

2. **MoveType**
  - Represents available moves (`Rock`, `Paper`, `Scissors`).
  - Key methods:
    - **`MoveType::random_move`**: Generates a random move for non-interactive gameplay.
    - **`MoveType::from_user_input`**: Enables user move input with input validation.
    - **`MoveType::convert_to_string`**: Turns moves (like `MoveType::Rock`) into a human-readable format (`"Rock"`).

   Example:
    ```rust
    use rock_paper_scissors::{MoveType, Winner};

    // Generate random enemy move
    let enemy_move = MoveType::random_move();
    assert!(matches!(enemy_move, MoveType::Rock | MoveType::Paper | MoveType::Scissors));
    ```

#### **Structs**

1. **PlayerMoves**
  - Bundles the user's and the enemy's moves for a single round.
  - Provides `PlayerMoves::check_who_wins_round` to determine the round winner based on rules.

   Example:
    ```rust
    use rock_paper_scissors::{MoveType, PlayerMoves, Winner};

    let player_moves = PlayerMoves {
        user_move: MoveType::Rock,
        enemy_move: MoveType::Scissors,
    };
    assert_eq!(player_moves.check_who_wins_round(), Winner::User);
    ```

2. **Scores**
  - Tracks the cumulative wins for both the user and the enemy.
  - Key methods:
    - **`Scores::check_for_winner`**: Determines if a player has reached the game's win condition (e.g., first to 3 wins).
    - **`Scores::reset`**: Resets scores for a new game.

   Example:
    ```rust
    use rock_paper_scissors::{Scores, Winner};

    let mut scores = Scores::new();
    scores.user_wins += 3;
    assert_eq!(scores.check_for_winner(), Ok(Winner::User));
    ```

---

## Playing the Game

Here is a complete example of how to implement and play a game of Rock-Paper-Scissors using the library:

```rust
use rock_paper_scissors::{PlayerMoves, Scores, Winner, MoveType};

fn main() {
    let mut scores = Scores::new();

    println!("Welcome to Rock-Paper-Scissors!");

    // Game loop
    while scores.check_for_winner().is_err() {
        let player_moves = PlayerMoves::new();

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
    let game_winner = scores.check_for_winner().unwrap();
    println!("Game over! {}", game_winner.convert_to_string());
}
```

---

## Error Handling

### Key Error Scenarios
- Invalid user input is handled gracefully by reprompting until valid input is provided (`MoveType::from_user_input`).
- Functions return result types (`Result` or `Option`) when necessary, ensuring invalid states are never propagated silently.

---

## Testing

This library features comprehensive tests to validate all core functionality. To run tests:

1. Clone the repository.
2. Run Cargo tests:
   ```bash
   cargo test
   ```

Here is an example test implementation:

```rust
#[test]
fn test_round_result() {
    use rock_paper_scissors::{MoveType, PlayerMoves, Winner};

    let moves = PlayerMoves {
        user_move: MoveType::Rock,
        enemy_move: MoveType::Scissors,
    };
    assert_eq!(moves.check_who_wins_round(), Winner::User);
}
```

---

## Contributing

We welcome contributions! Follow these steps to contribute:

1. Fork this repository.
2. Create a new branch for your feature or bug fix.
3. Commit your changes and push them to your fork.
4. Open a pull request describing your changes.

For feature requests or feedback, please open an issue on the GitHub repository.

---

## License

This project is distributed under the MIT License. See the `LICENSE` file for more details.