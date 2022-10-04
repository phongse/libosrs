use serde::Serialize;

#[derive(Default, Debug, Copy, Clone, Serialize)]
pub struct Skill {
    pub rank: u64,
    pub level: u64,
    pub xp: u64,
}

#[derive(Default, Debug, Copy, Clone, Serialize)]
pub struct Minigame {
    pub rank: u64,
    pub score: u64,
}

#[derive(Default, Debug, Copy, Clone, Serialize)]
pub struct Boss {
    pub rank: u64,
    pub score: u64,
}
