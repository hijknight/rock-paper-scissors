# rock-paper-scissors

[![Crates.io Version](https://img.shields.io/crates/v/rock-paper-scissors)](https://crates.io/crates/rock-paper-scissors)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/hijknight/rock-paper-scissors/rust.yml)](https://github.com/hijknight/rock-paper-scissors/actions)
[![docs.rs](https://img.shields.io/docsrs/rock-paper-scissors)](https://docs.rs/rock-paper-scissors/0.4.2/rock_paper_scissors/)
![Rust](https://img.shields.io/badge/rust-1.84.1-blue)

`rock-paper-scissors` is an open-source Rust library and interactive game designed for developers to create or customize implementations of the classic "Rock, Paper, Scissors" game. It adheres to **clean design principles**, offering modular functionality, safe initialization, and robust error handling.

---

## What's New in the Latest Update?

- **Enhancements to `MoveType`**:
  - Improved handling of edge cases for move validation.
  - Streamlined methods for better performance when generating random moves.

- **Improved User Messaging**:
  - Enhanced text clarity for both game results and input prompts.

- **Game Customization**:
  - Added support for additional game-winning conditions beyond "first to X wins."
  - Updated `GameSettings` to support user-defined conditions with enhanced flexibility.

- **Error Handling Overhaul**:
  - Cleaner and more detailed error messages for invalid user inputs or unexpected states.

- **Code Quality Improvements**:
  - Internal optimizations for better performance and reduced memory usage.

---

## Features

### Library Highlights
- **Customizable Game Logic**:
  - Enums and structs encapsulate game logic for modularity and easy customization.
  - Safe initialization with robust handling of invalid states or inputs.
- **Winner Determination**:
  - Built-in logic determines the winner of each round with clear rules (`PlayerMoves::check_who_wins_round`).
- **Score Management**:
  - Tracks user and enemy wins during the game with a `Scores` struct.
  - Allows checking the game's end (`Scores::check_for_winner`) or resetting (`Scores::reset`).
- **Game Settings**:
  - Enhanced capabilities for configuring win conditions and gameplay mechanics.
  - Prebuilt configurations like `GameSettings::first_to(first_to)` for quick usage.
- **User-Friendly Output**:
  - Updated message templates for better clarity and user engagement.
  - Human-readable string conversions for enums like `MoveType` and `Winner`.

### Game Highlights
- Dynamic and interactive gameplay with robust input validation.
- Enhanced game flexibility with customizable rule settings.
- Detailed round-by-round results and score updates.

---

## Getting Started

### Prerequisites

To use or play the `rock-paper-scissors` library, ensure you have the following installed:

- Rust (v1.84 or higher)
- Cargo, for building and running the library or game.

### Installation

To use the `rock-paper-scissors` library in your project, include it in your `Cargo.toml`:

```toml
[dependencies]
rock-paper-scissors = "0.4.2"
```

For the interactive game:

1. Clone the repository:
   ```bash
   git clone https://github.com/hijknight/rock-paper-scissors.git
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
  - Converts results into user-readable messages (e.g., `"You win!"`).

2. **MoveType**
  - Represents available moves and includes a variant for uninitialized states (`None`).
  - Methods for generating random moves and handling user input:
    - `random_move`: Randomly selects a valid move.
    - `from_user_input`: Validates and converts user input to moves.

#### **Structs**

1. **PlayerMoves**
  - Tracks moves for user and enemy, ensuring safe initialization and comparison.
  - Determines round winners and completes setup with helper methods (`build`).

2. **Scores**
  - Tracks user and enemy wins and checks for game-winning conditions.

3. **GameSettings**
  - Configures customized or predefined game-winning conditions.
  - Now supports more flexible rulesets for advanced customization.

---

## Playing the Game

Here’s an example of how to play a game of Rock-Paper-Scissors:

```rust
use rock_paper_scissors::{PlayerMoves, Scores, Winner, MoveType, GameSettings};

fn main() {
  let mut scores = Scores::new();
  let game_settings = GameSettings::first_to(5); // Play until one player wins 5 rounds.

  println!("Welcome to Rock-Paper-Scissors!");

  // Game loop
  while scores.check_for_winner(&game_settings).is_err() {
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

  let winner = scores.check_for_winner(&game_settings).unwrap();
  println!("Game over! {}", winner.convert_to_string());
}
```

---

## Error Handling

The library includes robust error handling:

### Key Error Scenarios
- **Invalid User Input**:
  - If the player enters invalid input (e.g., non-recognizable commands), the game reprompts using detailed error messages.
- **Edge Cases**:
  - Custom validations prevent unexpected behaviors in uninitialized or edge states.

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