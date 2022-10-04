pub const HISCORE_URL: &str =
    "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player=";

pub const HISCORE_URL_IRONMAN: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_ironman/index_lite.ws?player=";

pub const HISCORE_URL_HARDCORE_IRONMAN: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_hardcore_ironman/index_lite.ws?player=";

pub const HISCORE_URL_ULTIMATE: &str =
    "https://secure.runescape.com/m=hiscore_oldschool_ultimate/index_lite.ws?player=";

pub fn get_gamemode(gamemode: &str) -> &str {
    match gamemode {
        "regular" => HISCORE_URL,
        "ironman" => HISCORE_URL_IRONMAN,
        "hardcore" => HISCORE_URL_HARDCORE_IRONMAN,
        "ultimate" => HISCORE_URL_ULTIMATE,
        _ => HISCORE_URL,
    }
}
