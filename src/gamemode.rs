use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Gamemode {
    Auto,
    Regular,
    Ironman,
    Hardcore,
    Ultimate,
    Deadman,
    Seasonal,
    Tournament,
}
