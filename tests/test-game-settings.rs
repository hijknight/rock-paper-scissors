use rock_paper_scissors::*;

#[test]
fn test_game_settings_new() {
    let game_settings = GameSettings::new();

    assert_eq!(game_settings, GameSettings {
        first_to: 1
    });
}

#[test]
fn test_game_settings_from_first_to() {
    let game_settings = GameSettings::from_first_to(3);

    assert_eq!(game_settings, GameSettings {
        first_to: 3
    });
}