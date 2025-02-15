# rock-paper-scissors

[![Crates.io Version](https://img.shields.io/crates/v/rock-paper-scissors)](https://crates.io/crates/rock-paper-scissors)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/hijknight/rock-paper-scissors/rust.yml)](https://github.com/hijknight/rock-paper-scissors/actions)
[![docs.rs](https://img.shields.io/docsrs/rock-paper-scissors)](https://docs.rs/rock-paper-scissors/0.5.0/rock_paper_scissors/)
![Rust](https://img.shields.io/badge/rust-1.84.1-blue)

`rock-paper-scissors` is an open-source Rust library and interactive game designed for developers to create or customize implementations of the classic "Rock, Paper, Scissors" game. It adheres to **clean design principles**, offering modular functionality, safe initialization, and robust error handling.

---

## What's New in the Latest Update?

- **Enhancements to `MoveType`**:
  - Improved handling of edge cases for move validation.
  - Streamlined methods for better performance when generating random moves.

- **User Experience Improvements**:
  - Enhanced messaging for both game results and input prompts to improve clarity.

- **Expanded Customization**:
  - Support for custom game-winning conditions, allowing advanced configurations beyond "first to X wins."
  - `GameSettings` now supports user-defined rules with increased flexibility.

- **Error Handling Overhaul**:
  - Enhanced error messages providing clear guidance for invalid user inputs or unexpected situations.

- **Performance & Code Quality**:
  - Internal optimizations for faster processing and reduced memory overhead.

- **Library Version Update**:
  - Now compatible with Rust 1.84.1 for better performance and reliability.

---

## Features

### Library Highlights
- **Customizable Game Logic**:
  - Enums and structs encapsulate game logic for modularity and easy customization.
  - Safe initialization with robust handling of invalid states or inputs.

- **Flexible Winner Determination**:
  - Built-in logic to evaluate round outcomes with the `PlayerMoves::check_who_wins_round` method.

- **Score Tracking**:
  - Tracks user and enemy wins with the `Scores` struct.
  - Supports checking game-winning conditions (`Scores::check_for_winner`) or resetting scores (`Scores::reset`).

- **Game Settings**:
  - New support for user-defined gameplay rules and conditions.
  - Prebuilt configurations like `GameSettings::first_to(first_to)` for quick and easy implementation.

- **Output Improvements**:
  - Human-readable string conversions for enums, such as `MoveType` and `Winner`.
  - Updated message templates ensure intuitive and engaging player interactions.

### Game Highlights
- Interactive gameplay with robust input validation.
- Flexible rules, supporting custom winning conditions and gameplay mechanics.
- Detailed round-by-round results and score summaries for an engaging player experience.

---

## Getting Started

### Prerequisites

To use or play the `rock-paper-scissors` library, ensure the following are installed:

- **Rust** (v1.84.1 or higher)
- **Cargo**, for building and running the library or game.

### Installation

To use the library in your Rust project, add the following line to your `Cargo.toml`:

```toml
[dependencies]
rock-paper-scissors = "0.5.0"
```

To play the game directly:

1. Clone the repository:
   ```bash
   git clone https://github.com/hijknight/rock-paper-scissors.git
   cd rock-paper-scissors
   ```

2. Run the game with Cargo:
   ```bash
   cargo run
   ```

---

## Library Overview

### Core Components

#### **Enums**

1. **`Winner`**
  - Represents round outcomes: `User`, `Enemy`, or `Tie`.
  - Converts results into user-readable messages.

2. **`MoveType`**
  - Represents available moves (e.g., Rock, Paper, Scissors) with an uninitialized state (`None`).
  - Utility methods:
    - `random_move`: Generates a random move.
    - `from_user_input`: Validates and converts strings to moves.

#### **Structs**

1. **`PlayerMoves`**
  - Tracks user and enemy moves.
  - Provides helper methods for initialization (`build`) and winner determination.

2. **`Scores`**
  - Records scores and validates game-winning conditions.

3. **`GameSettings`**
  - Allows configuration of gameplay rules, such as custom win conditions.

---

## Playing the Game

Here's a simple example of playing Rock-Paper-Scissors programmatically:

```rust
use rock_paper_scissors::{PlayerMoves, Scores, Winner, MoveType, GameSettings};

fn main() {
  let mut scores = Scores::new();
  let game_settings = GameSettings::first_to(3); // First to 3 wins.

  println!("Welcome to Rock-Paper-Scissors!");

  while scores.check_for_winner(&game_settings).is_err() {
    let player_moves = PlayerMoves::build();

    let round_winner = player_moves.check_who_wins_round();
    println!(
      "You chose: {}, Enemy chose: {}.",
      player_moves.user_move.convert_to_string(),
      player_moves.enemy_move.convert_to_string(),
    );

    println!("Result: {}", round_winner.convert_to_string());

    match round_winner {
      Winner::User => scores.user_wins += 1,
      Winner::Enemy => scores.enemy_wins += 1,
      Winner::Tie => (),
    }

    println!(
      "Current Scores -> User: {}, Enemy: {}",
      scores.user_wins, scores.enemy_wins
    );
  }

  let final_winner = scores.check_for_winner(&game_settings).unwrap();
  println!("Game Over! {}", final_winner.convert_to_string());
}
```

---

## Error Handling

The library provides robust error management:

- **Invalid Input Handling**:
  - Detects and reprompts users when invalid inputs (e.g., invalid string commands) are entered.

- **Edge Case Management**:
  - Ensures safe behavior in uninitialized or unexpected game states.

---

## Contributing

Do you have ideas, bug fixes, or improvements? Contributions are welcome! Here's how to get involved:

1. Fork the repository.
2. Create a new branch for your feature.
3. Commit and push your updates.
4. Submit a Pull Request for review.

For issues and feature requests, please open a ticket on GitHub.

---

## License

Distributed under the MIT License. See the `LICENSE` file for more information.