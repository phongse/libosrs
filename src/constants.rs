use crate::gamemode::Gamemode;

pub const HISCORE_URL: &str =
    "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player=";

pub const HISCORE_URL_IRONMAN: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_ironman/index_lite.ws?player=";

pub const HISCORE_URL_HARDCORE_IRONMAN: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_hardcore_ironman/index_lite.ws?player=";

pub const HISCORE_URL_ULTIMATE: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_ultimate/index_lite.ws?player=";

pub const HISCORE_URL_DEADMAN: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_deadman/index_lite.ws?player=";

pub const HISCORE_URL_SEASONAL: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_seasonal/index_lite.ws?player=";

pub const HISCORE_URL_TOURNAMENT: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_tournament/index_lite.ws?player=";

pub fn get_gamemode(gamemode: Gamemode) -> &'static str {
    match gamemode {
        Gamemode::Regular => HISCORE_URL,
        Gamemode::Ironman => HISCORE_URL_IRONMAN,
        Gamemode::Hardcore => HISCORE_URL_HARDCORE_IRONMAN,
        Gamemode::Ultimate => HISCORE_URL_ULTIMATE,
        Gamemode::Deadman => HISCORE_URL_DEADMAN,
        Gamemode::Seasonal => HISCORE_URL_SEASONAL,
        Gamemode::Tournament => HISCORE_URL_TOURNAMENT,
        _ => HISCORE_URL,
    }
}
