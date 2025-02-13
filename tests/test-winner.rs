use rock_paper_scissors::*;

#[test]
fn convert_to_string_works() {
    assert_eq!(Winner::User.convert_to_string(), "User");

    assert_eq!(Winner::Enemy.convert_to_string(), "Enemy");

    assert_eq!(Winner::Tie.convert_to_string(), "Tie");
}

