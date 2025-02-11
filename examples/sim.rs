use rock_paper_scissors::*;

fn main() {
    let loops: u8 = 255;

    let mut scores = Scores::new();

    let mut player_moves = PlayerMoves::new();

    for _ in 0..=loops {
        (player_moves.user_move, player_moves.enemy_move) = (MoveType::random_move(), MoveType::random_move());

        let winner = player_moves.check_who_wins_round();

        match winner {
            Winner::User => scores.user_wins += 1,
            Winner::Enemy => scores.enemy_wins += 1,
            Winner::Tie => (),
        }
    }

    println!("{:?}", scores);
}