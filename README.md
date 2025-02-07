# rock-paper-scissors

`rock-paper-scissors` is an open-source Rust library and interactive game designed for developers who want to create or customize implementations of the classic "Rock, Paper, Scissors" game while adhering to clean design principles, such as **data encapsulation**.

### [rock-paper-scissors docs.rs](https://docs.rs/rock-paper-scissors/)

---

## Features

### Library Highlights
- **Reusable Game Logic**:
  - Encapsulated enums and structs for game logic, making it modular and easy to customize.
  - Functions like `MoveType::random_move` and `MoveType::from_user_input` promote both randomness and user interactivity.
- **Winner Determination**:
  - Implements clear rules for determining the winner for each round.
- **Score Tracking**:
  - Tracks wins for the user and enemy throughout the game with helper methods like `Scores::check_for_winner`.
- **Utilities for Friendly Output**:
  - `MoveType` and `Winner` enums are convertible to human-readable text for better integration into user interfaces.

### Game Highlights
- Work interactively using functions like `MoveType::from_user_input`.
- Play a head-to-head match against a randomly generated opponent (enemy).
- Built-in input validation ensures a smooth, error-free experience.

---

## Getting Started

### Prerequisites

To use or play the `rock-paper-scissors` library, you'll need:

- Rust (v1.64 or higher)
- Cargo to compile and run the game.

### Installation

To use the `rock-paper-scissors` library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
rock-paper-scissors = "0.1.0"
```

To run the game interactively:

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo-name/rock-paper-scissors.git
   cd rock-paper-scissors
   ```
2. Use Cargo to run the game:
   ```bash
   cargo run
   ```

---

## Usage Example: Using the New `from_user_input` Method

Here’s an example of how to use the updated `MoveType::from_user_input` method to create a round of Rock-Paper-Scissors:

```rust
use rock_paper_scissors::{MoveType, PlayerMoves, Winner};

fn main() {
    println!("Choose 1 (Rock), 2 (Paper), or 3 (Scissors):");

    // Use `from_user_input` to get a move directly from the user
    let user_move = MoveType::from_user_input();
    let enemy_move = MoveType::random_move();

    // Combine into `PlayerMoves` and evaluate the round winner
    let player_moves = PlayerMoves {
        user_move,
        enemy_move,
    };

    let winner = player_moves.check_who_wins_round();

    println!(
        "User Move: {}, Enemy Move: {}, Round Result: {}",
        player_moves.user_move.convert_to_string(),
        player_moves.enemy_move.convert_to_string(),
        winner.convert_to_string()
    );
}
```

---

## Encapsulation Overview

### New Method: `MoveType::from_user_input`

The `from_user_input` method simplifies interactive gameplay by directly asking the user for input.

- Prompts the user for input (`1` for Rock, `2` for Paper, `3` for Scissors).
- Ensures valid input through looping and re-prompting until the user enters a correct value.
- Returns a corresponding `MoveType`.

### Available Core Types and Methods

#### **Enums**

- **`Winner`**:
  - Represents the outcomes of a round (`User`, `Enemy`, `Tie`).
  - Includes `Winner::convert_to_string()` for readable output.
- **`MoveType`**:
  - Represents the three possible moves (`Rock`, `Paper`, `Scissors`).
  - Includes utility methods:
    - `MoveType::random_move()` for random enemy moves.
    - `MoveType::convert_to_string()` for printing moves.
    - `MoveType::from_user_input()` for user-interactive input (new).

#### **Structs**

- **`PlayerMoves`**:
  - Tracks both user and enemy moves.
  - Includes methods like `PlayerMoves::check_who_wins_round()` to evaluate the round winner.
- **`Scores`**:
  - Tracks overall scores (user wins and enemy wins).
  - Includes methods like `Scores::check_for_winner()` and `Scores::reset()`.

---

## Playing the Full Game

The following example shows the complete game implementation:

```rust
use rock_paper_scissors::{PlayerMoves, Scores, Winner, MoveType};

fn main() {
    let mut scores = Scores::new();

    println!("Welcome to Rock-Paper-Scissors!");

    // Game loop
    while scores.check_for_winner().is_err() {
        let player_moves = PlayerMoves::new();

        let winner = player_moves.check_who_wins_round();
        println!(
            "You chose {}. Enemy chose {}.",
            player_moves.user_move.convert_to_string(),
            player_moves.enemy_move.convert_to_string()
        );
        println!("{}", winner.convert_to_string());

        // Update scores
        match winner {
            Winner::User => scores.user_wins += 1,
            Winner::Enemy => scores.enemy_wins += 1,
            Winner::Tie => (),
        }
        println!("Current Scores -> You: {}, Enemy: {}", scores.user_wins, scores.enemy_wins);
    }

    // End game
    let final_winner = scores.check_for_winner().unwrap();
    println!("Game over! {}", final_winner.convert_to_string());
}
```

---

## Testing

The library includes a comprehensive suite of tests to ensure accurate functionality. Testing can be run as follows:

```bash
cargo test
```

Example of a typical test:

```rust
#[test]
fn test_random_move() {
    use rock_paper_scissors::MoveType;

    // Validate that random_move generates valid results.
    let random_move = MoveType::random_move();
    assert!(matches!(random_move, MoveType::Rock | MoveType::Paper | MoveType::Scissors));
}
```

---

## Contributing

We welcome contributions! If you'd like to improve the library, follow these steps:

1. Fork the repository.
2. Create a branch for your changes.
3. Make your edits and commit your changes.
4. Open a pull request.

For suggestions or feature requests, open an issue, and let us know how we can improve.

---

## License

This project is distributed under the MIT License. See the `LICENSE` file for more information.