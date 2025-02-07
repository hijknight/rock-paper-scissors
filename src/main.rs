
use rps::*;


fn main() {
    println!("Welcome to Rock Paper Scissors");
    println!();
    println!("Choose your move:");
    println!("   1. Rock");
    println!("   2. Paper");
    println!("   3. Scissors");

    let mut scores = Scores::new();
    let mut round = 0;

    loop {
        round += 1;
        println!();
        println!("Round {}\n", round);

        let user_move = get_user_move();
        let enemy_move = enemy_move();

        let winner = check_who_wins_round(&user_move, &enemy_move);

        println!("{} vs {} : {}", convert_move_to_string(&user_move), convert_move_to_string(&enemy_move), convert_winner_to_string(&winner));

        match winner {
            Winner::Tie => {
                println!("Tie! Replaying round {}", round);
                round -= 1;
                continue;
            },
            Winner::User => scores.user_wins += 1,
            Winner::Enemy => scores.enemy_wins += 1,
        }
        println!();
        println!("Scores after round {}:\n", round);
        println!("User: {}", scores.user_wins);
        println!("Enemy: {}", scores.enemy_wins);


        if let Ok(winner) = check_for_winner(&scores) {
            println!("{}", convert_winner_to_string(&winner));
            break;
        }
    }
}