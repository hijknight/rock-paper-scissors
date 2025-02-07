# rock-paper-scissors

`rock-paper-scissors` is an open-source Rust library and game for implementing and playing custom variations of the classic "Rock, Paper, Scissors" game. This project is both a standalone game and a flexible API for developers curious about creating their own implementations or extending the logic.

## Features

### Library Highlights
- **Winner Determination Logic**:
    - Implements rules for determining the winner of each round (user wins, enemy wins, or tie).
- **Customizable Gameplay**:
    - The library exposes functionality for generating random enemy moves and handling user input to create custom rules.
- **Game State Tracking**:
    - Provides a `Scores` struct to track the user's and enemy's wins.
- **Utilities**:
    - Convert moves and winners to readable strings for user-friendly interfaces.

### Game Highlights
- Play against a randomly selected opponent (enemy).
- Scores are tracked across rounds.
- Gameplay loop continues until there’s a winner (first to 3 wins).
- Input validation and error handling for invalid moves.

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

To run the game:
1. Clone this repository:
   ```bash
   git clone https://github.com/your-repo-name/rock-paper-scissors.git
   cd rock-paper-scissors
   ```
2. Run the game:
   ```bash
   cargo run
   ```

## Usage Example: As a Library

Here’s how you can use the `rock-paper-scissors` library to create your own game:

```rust
use rock_paper_scissors::*;

fn main() {
    let user_move = MoveType::Rock; // User selects Rock
    let enemy_move = enemy_move(); // Randomly generate enemy's move

    let winner = check_who_wins_round(&user_move, &enemy_move);

    println!(
        "User's Move: {}, Enemy's Move: {}, Result: {}",
        convert_move_to_string(&user_move),
        convert_move_to_string(&enemy_move),
        convert_winner_to_string(&winner)
    );
}
```

Simply build on these API functions to extend the game or integrate the logic into your application.

## Usage Example: Playing the Game

Run the game for an interactive experience:

```bash
Welcome to Rock Paper Scissors

Choose your move:
   1. Rock
   2. Paper
   3. Scissors
// User chooses a number
```

### Winning Logic
- **Ties:** Same move for both players.
- **Winning Combinations:**
    - Rock beats Scissors.
    - Paper beats Rock.
    - Scissors beat Paper.

### Ending the Game
The game ends when either the user or the enemy wins 3 rounds.

## API Overview

Below are the key functionalities offered by the library:

### Enums
#### **Winner**
- `Winner::User`: Indicates the user won.
- `Winner::Enemy`: Indicates the enemy won.
- `Winner::Tie`: Indicates it’s a tie.

#### **MoveType**
- `MoveType::Rock`
- `MoveType::Paper`
- `MoveType::Scissors`

### Structs
#### **Scores**
Tracks the user’s and enemy’s wins:
- `pub user_wins: u8`
- `pub enemy_wins: u8`

### Functions
#### Game Logic
- `check_who_wins_round(user_move: &MoveType, enemy_move: &MoveType) -> Winner`: Determine the winner of a round.
- `check_for_winner(scores: &Scores) -> Result<Winner, &str>`: Check if there is an overall winner.

#### Utilities
- `convert_move_to_string(move_type: &MoveType) -> String`: Convert moves to a user-friendly string.
- `convert_winner_to_string(winner: &Winner) -> String`: Convert winner to a user-friendly string.

#### Enemy Move
- `enemy_move() -> MoveType`: Randomly generates the enemy’s move.

#### User Input
- `get_user_move() -> MoveType`: Handle and validate user input for selecting a move.

## Testing
The library includes unit tests to verify its functionality. Run the tests using:

```bash
cargo test
```

Example tests:
- `check_for_winner` ensures the winner logic is properly evaluated.
- `check_who_wins_round` tests all combinations to determine the correct round winner.
- Utility functions like `convert_move_to_string` and `convert_winner_to_string` are also tested.

## Contributing
Contributions to improve the library or game are welcome. To contribute:
1. Fork the repository.
2. Create a new feature branch.
3. Commit your changes.
4. Open a pull request with a detailed description of your changes.

## License
This project is licensed under the [MIT License](./LICENSE). Feel free to use, modify, and distribute. See the `LICENSE` file for more details.

## Acknowledgment
Special thanks to the `rand` crate for random number generation.

---

Enjoy coding and playing with `rock-paper-scissors`! 🎮