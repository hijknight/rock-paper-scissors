# rock-paper-scissors

`rock-paper-scissors` is an open-source Rust library and interactive game designed for developers who want to create or customize implementations of the classic "Rock, Paper, Scissors" game while adhering to clean design principles, such as **data encapsulation**.

### [rock-paper-scissors docs.rs](https://docs.rs/rock-paper-scissors/0.2.1/rock_paper_scissors/)

## Features

### Library Highlights (Encapsulation-Enhanced)
- **Encapsulated Game Logic**:
    - Game logic is now modular and encapsulated, making it simpler to maintain and extend while protecting key data structures like `Scores` and `PlayerMoves` from misuse.
    - Example: The new `PlayerMoves` struct centralizes the user's and enemy's move management, encouraging cleaner, structured gameplay operations.
- **Winner Determination Logic**:
    - Implements rules for determining the winner for each round in a structured, encapsulated manner, ensuring reusability.
- **Customizable Gameplay**:
    - Functions for random enemy move generation (`MoveType::random_move`) and user input handling (`get_user_move`) now integrate naturally with other encapsulated functionalities.
- **Game State and Round Tracking**:
    - The `Scores` struct allows easy tracking of wins for the user and enemy, with utility functions to check for the overall winner of the game.
- **Utilities for Friendly Output**:
    - Easily convert `MoveType` and `Winner` enums into human-readable strings for a smoother integration into user interfaces.

### Game Highlights
- Play a head-to-head match against a randomly generated opponent (enemy).
- Encapsulated game-loop logic for cleaner and more adaptive gameplay behavior.
- Scores are tracked and reset automatically across rounds.
- User-friendly input validation ensures a seamless experience for both players and developers.

---

## Getting Started

### Prerequisites
- Rust (v1.64 or higher)
- Cargo for dependency and package management.

### Installation
To use the `rock-paper-scissors` library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
rock-paper-scissors = "0.1.0"
```

To play the built-in game:
1. Clone this repository:
   ```bash
   git clone https://github.com/your-repo-name/rock-paper-scissors.git
   cd rock-paper-scissors
   ```
2. Run the game:
   ```bash
   cargo run
   ```

---

## Usage Example: As a Library

Here's how you can use the encapsulated `rock-paper-scissors` library to implement a round of the game:

```rust
use rock_paper_scissors::*;

fn main() {
    // Generate a user move and a random enemy move.
    let player_moves = PlayerMoves {
        user_move: MoveType::Rock, // Or use `get_user_move()` for interactivity.
        enemy_move: MoveType::random_move(),
    };

    let winner = player_moves.check_who_wins_round();

    println!(
        "User Move: {}, Enemy Move: {}, Round Result: {}",
        player_moves.user_move.convert_to_string(),
        player_moves.enemy_move.convert_to_string(),
        winner.convert_to_string(),
    );
}
```

This structure ensures clear encapsulation, so developers can focus only on high-level gameplay operations.

---

## Usage Example: Playing the Game

To play the game interactively, simply run:

```bash
Welcome to Rock Paper Scissors

Choose your move:
   1. Rock
   2. Paper
   3. Scissors
// Player selects a move, and gameplay proceeds based on their input.
```

Gameplay will continue until either the user or the enemy wins 3 rounds.

---

## Encapsulation Enhancements Overview

### Structs

Encapsulation focuses on bundling data and operations in logical units, simplifying the API surface.

#### **`PlayerMoves` Struct**
Manages round-specific user and enemy moves.
- `pub user_move: MoveType`: Represents the user's current move.
- `pub enemy_move: MoveType`: Represents the randomly chosen enemy move.

#### **`Scores` Struct**
Tracks wins for each player.
- `pub user_wins: u8`
- `pub enemy_wins: u8`
- Encapsulated methods:
    - `Scores::new() -> Scores`: Initializes a clean scoresheet.
    - `Scores::check_for_winner() -> Result<Winner, &str>`: Determines overall winner or indicates ongoing gameplay.
    - `Scores::reset()`: Resets the scores to `0`.

### Enums

Match states and move types are represented using enums for clarity and type safety.

#### **`Winner` Enum**
- `Winner::User`: Indicates the user won.
- `Winner::Enemy`: Indicates the enemy won.
- `Winner::Tie`: Indicates the round was a tie.
- Encapsulation utility:
    - `Winner::convert_to_string(&self) -> String`: Produces a human-readable string representation of the winner.

#### **`MoveType` Enum**
- `MoveType::Rock`
- `MoveType::Paper`
- `MoveType::Scissors`
- Encapsulation utilities:
    - `MoveType::random_move() -> MoveType`: Generates a random move.
    - `MoveType::convert_to_string(&self) -> String`: Produces a user-friendly string representation of the move.

---

## Core Game Logic Overview

### Gameplay Loop
The game's internal loop uses encapsulated interactions for user moves, enemy move generation, and winner determination:

```rust
loop {
    let player_moves = PlayerMoves::new();
    let winner = player_moves.check_who_wins_round();
    scores.update(winner);
    if let Ok(game_winner) = scores.check_for_winner() {
        println!("{}", game_winner.convert_to_string());
        break;
    }
}
```

This clean encapsulation ensures the logic remains separate from the interface or any extra layers of complexity.

---

## API Highlights

### Game Logic
- `PlayerMoves::new() -> PlayerMoves`: Initializes round-specific user and enemy moves.
- `PlayerMoves::check_who_wins_round(&self) -> Winner`: Determines the round winner.
- `Scores::check_for_winner(&self) -> Result<Winner, &str>`: Checks if a player has reached the winning condition (first to 3 wins).

### Utilities
- `MoveType::random_move() -> MoveType`: Generates a random enemy move (`Rock`, `Paper`, or `Scissors`).
- `Winner::convert_to_string(&self) -> String`: Returns a human-readable winner message (e.g., `"Tie!"`, `"You win!"`).

---

## Testing

The library includes complete unit tests covering all core features, such as:
- Accurate handling of all potential gameplay outcomes.
- Validation of string conversions for enums.
- Structural correctness and encapsulated methods.

Run the tests with:
```bash
cargo test
```

Example test:
```rust
#[test]
fn test_check_who_wins_round() {
    let player_moves = PlayerMoves {
        user_move: MoveType::Rock,
        enemy_move: MoveType::Scissors,
    };
    assert_eq!(player_moves.check_who_wins_round(), Winner::User);
}
```

---

## Contributing

Contributions are welcome! Follow these steps to contribute:
1. Fork the repository.
2. Create a new feature branch.
3. Commit your changes and push.
4. Open a pull request for review.

---

## License
This project is licensed under the [MIT License](./LICENSE). See the `LICENSE` file for more details.

---

Enjoy coding and playing with `rock-paper-scissors`! 🎮
