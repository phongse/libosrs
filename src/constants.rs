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

pub fn get_gamemode(gamemode: &str) -> &str {
    match gamemode {
        "regular" => HISCORE_URL,
        "ironman" => HISCORE_URL_IRONMAN,
        "hardcore" => HISCORE_URL_HARDCORE_IRONMAN,
        "ultimate" => HISCORE_URL_ULTIMATE,
        "deadman" => HISCORE_URL_DEADMAN,
        "seasonal" => HISCORE_URL_SEASONAL,
        "tournament" => HISCORE_URL_TOURNAMENT,
        _ => HISCORE_URL,
    }
}
