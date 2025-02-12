# rock-paper-scissors

[![Crates.io Version](https://img.shields.io/crates/v/rock-paper-scissors)](https://crates.io/crates/rock-paper-scissors)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/hijknight/rock-paper-scissors/rust.yml)](https://github.com/hijknight/rock-paper-scissors/actions)
[![docs.rs](https://img.shields.io/docsrs/rock-paper-scissors)](https://docs.rs/rock-paper-scissors/0.3.3/rock_paper_scissors/)
![Rust](https://img.shields.io/badge/rust-1.84.1-blue)

`rock-paper-scissors` is an open-source Rust library and interactive game designed for developers to create or customize implementations of the classic "Rock, Paper, Scissors" game. It adheres to **clean design principles**, offering modular functionality, safe initialization, and robust error handling.

---

## Features

### Library Highlights
- **Customizable Game Logic**:
    - Enums and structs encapsulate game logic for modularity and easy customization.
    - New `MoveType::None` variant ensures safe initialization for moves.
    - Methods like `MoveType::random_move` simplify randomization, while `MoveType::from_user_input` handles inputs gracefully.
- **Winner Determination**:
    - Built-in logic determines the winner of each round with clear rules (`PlayerMoves::check_who_wins_round`).
- **Score Management**:
    - Tracks user and enemy wins during the game with a `Scores` struct.
    - Allows checking the game's end (`Scores::check_for_winner`) or resetting (`Scores::reset`).
- **Safe Initialization**:
    - The `PlayerMoves::new` method initializes moves (`user_move` and `enemy_move`) as `MoveType::None`, reducing errors during uninitialized states.
- **Friendly Output Utilities**:
    - Human-readable string conversion for enums like `MoveType` and `Winner` provides better readability and interaction.

### Game Highlights
- Interactive gameplay through `MoveType::from_user_input()` for user moves.
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
rock-paper-scissors = "0.4.0"
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
  Example output: `"You win!"` or `"You lose!"`.

2. **MoveType**
- Represents available moves (`Rock`, `Paper`, `Scissors`) and includes a new variant:
    - **`MoveType::None`**: Represents an uninitialized state or invalid move, ensuring safe initialization.
- Key methods:
    - **`MoveType::random_move`**: Generates a random move for non-interactive gameplay.
    - **`MoveType::from_user_input`**: Validates and converts user input to move types.
    - **`MoveType::convert_to_string`**: Converts moves (like `MoveType::Rock`) into a human-readable format (`"Rock"`).

Example:
```rust
use rock_paper_scissors::{MoveType, Winner};

// Generate random enemy move
let enemy_move = MoveType::random_move();
assert!(matches!(enemy_move, MoveType::Rock | MoveType::Paper | MoveType::Scissors));

// Handle uninitialized moves
let unset_move = MoveType::None;
assert_eq!(unset_move.convert_to_string(), "None");
```

#### **Structs**

1. **PlayerMoves**
- Bundles the user's and the enemy's moves for a single round.
- Features:
    - **`PlayerMoves::new`**: Initializes moves as `MoveType::None` for safe usage.
    - **`PlayerMoves::check_who_wins_round`**: Determines the round winner.
    - **`PlayerMoves::build`**: Completes initialization with user input and a random enemy move.

Example:
```rust
use rock_paper_scissors::{MoveType, PlayerMoves, Winner};

// Initialize moves
let moves = PlayerMoves::new();
assert_eq!(moves.user_move, MoveType::None);
assert_eq!(moves.enemy_move, MoveType::None);

// Check round winner
let player_moves = PlayerMoves {
user_move: MoveType::Rock,
enemy_move: MoveType::Scissors,
};
assert_eq!(player_moves.check_who_wins_round(), Winner::User);
```

2. **Scores**
- Tracks the cumulative wins for both the user and the enemy.
- Features:
    - **`Scores::check_for_winner`**: Determines if a player has reached the game's win condition (e.g., first to 3 wins).
    - **`Scores::reset`**: Resets scores for a new game.

Example:
```rust
use rock_paper_scissors::{Scores, Winner};

let mut scores = Scores::new();
scores.user_wins += 3;
assert_eq!(scores.check_for_winner(3), Ok(Winner::User)); // user should win

// Reset scores
scores.reset();
assert_eq!(scores.user_wins, 0);
assert_eq!(scores.enemy_wins, 0);
```

---

## Playing the Game

Here's an interactive example of how to play a game of Rock-Paper-Scissors using the library:

```rust
use rock_paper_scissors::{PlayerMoves, Scores, Winner, MoveType};

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
    let game_winner = scores.check_for_winner(3).unwrap();
    println!("Game over! {}", game_winner.convert_to_string());
}
```

---

## Error Handling

### Key Error Scenarios
- **Invalid user input**:
    - If the player enters invalid input (e.g., letters, out-of-range numbers), the game reprompts them via `MoveType::from_user_input`.
- **Uninitialized states**:
    - The `MoveType::None` variant prevents invalid states during custom logic or gameplay setup.

---

## Contributing

We welcome contributions! To contribute:

1. Fork this repository.
2. Create a new branch for your feature or bug fix.
3. Push your changes and open a Pull Request.

For feature requests or issues, please open a GitHub issue.

---

## License

This project is distributed under the MIT License. See the `LICENSE` file for details.