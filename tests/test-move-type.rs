use rock_paper_scissors::*;

#[test]
fn random_move_works() {
    let random_move = MoveType::random_move();
    assert!(matches!(random_move, MoveType::Rock | MoveType::Paper | MoveType::Scissors));
}

#[test]
fn convert_to_string_works() {
    let move_type = MoveType::Rock;

    assert_eq!(move_type.convert_to_string(), "Rock");

    let move_type = MoveType::None;

    assert_eq!(move_type.convert_to_string(), "None");

    let move_type = MoveType::Scissors;

    assert_eq!(move_type.convert_to_string(), "Scissors");

    let move_type = MoveType::Paper;

    assert_eq!(move_type.convert_to_string(), "Paper");
}