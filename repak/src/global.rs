use std::sync::OnceLock;

static GAME_ID: OnceLock<Option<String>> = OnceLock::new();

pub const GAME_ID_VISIONS_OF_MANA: &str = "VisionsofMana";

pub fn get_game_id(default: Option<String>) -> &'static Option<String> {
    GAME_ID.get_or_init(|| { default })
}