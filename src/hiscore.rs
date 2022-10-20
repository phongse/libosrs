use serde::Serialize;

use crate::skill::{Boss, Minigame, Skill};
use crate::util;

#[derive(Default, Debug, Serialize)]
pub struct Skills {
    pub overall: Skill,
    pub attack: Skill,
    pub defence: Skill,
    pub strength: Skill,
    pub hitpoints: Skill,
    pub ranged: Skill,
    pub prayer: Skill,
    pub magic: Skill,
    pub cooking: Skill,
    pub woodcutting: Skill,
    pub fletching: Skill,
    pub fishing: Skill,
    pub firemaking: Skill,
    pub crafting: Skill,
    pub smithing: Skill,
    pub mining: Skill,
    pub herblore: Skill,
    pub agility: Skill,
    pub thieving: Skill,
    pub slayer: Skill,
    pub farming: Skill,
    pub runecrafting: Skill,
    pub hunter: Skill,
    pub construction: Skill,
}

impl Skills {
    pub fn build_skills(skills: Vec<Skill>) -> Skills {
        Skills {
            overall: skills[0],
            attack: skills[1],
            defence: skills[2],
            strength: skills[3],
            hitpoints: skills[4],
            ranged: skills[5],
            prayer: skills[6],
            magic: skills[7],
            cooking: skills[8],
            woodcutting: skills[9],
            fletching: skills[10],
            fishing: skills[11],
            firemaking: skills[12],
            crafting: skills[13],
            smithing: skills[14],
            mining: skills[15],
            herblore: skills[16],
            agility: skills[17],
            thieving: skills[18],
            slayer: skills[19],
            farming: skills[20],
            runecrafting: skills[21],
            hunter: skills[22],
            construction: skills[23],
        }
    }

    pub fn get_overall_xp(self) -> u64 {
        self.overall.xp
    }
}

#[derive(Default, Debug, Serialize)]
pub struct Minigames {
    pub league_points: Minigame,
    pub bounty_hunter_hunter: Minigame,
    pub bounty_hunter_rogue: Minigame,
    pub clue_scrolls_all: Minigame,
    pub clue_scrolls_beginner: Minigame,
    pub clue_scrolls_easy: Minigame,
    pub clue_scrolls_medium: Minigame,
    pub clue_scrolls_hard: Minigame,
    pub clue_scrolls_elite: Minigame,
    pub clue_scrolls_master: Minigame,
    pub lms_rank: Minigame,
    pub pvp_arena_rank: Minigame,
    pub soul_wars_zeal: Minigame,
    pub rifts_closed: Minigame,
}

impl Minigames {
    pub fn build_minigames(minigames: Vec<Minigame>) -> Minigames {
        Minigames {
            league_points: minigames[0],
            bounty_hunter_hunter: minigames[1],
            bounty_hunter_rogue: minigames[2],
            clue_scrolls_all: minigames[3],
            clue_scrolls_beginner: minigames[4],
            clue_scrolls_easy: minigames[5],
            clue_scrolls_medium: minigames[6],
            clue_scrolls_hard: minigames[7],
            clue_scrolls_elite: minigames[8],
            clue_scrolls_master: minigames[9],
            lms_rank: minigames[10],
            pvp_arena_rank: minigames[11],
            soul_wars_zeal: minigames[12],
            rifts_closed: minigames[13],
        }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct Bosses {
    // Bosses
    pub abyssal_sire: Boss,
    pub alchemical_hydra: Boss,
    pub barrows_chests: Boss,
    pub bryophyta: Boss,
    pub callisto: Boss,
    pub cerberus: Boss,
    pub chambers_of_xeric: Boss,
    pub chambers_of_xeric_challenge_mode: Boss,
    pub chaos_elemental: Boss,
    pub chaos_fanatic: Boss,
    pub commander_zilyana: Boss,
    pub corporeal_beast: Boss,
    pub crazy_archaeologist: Boss,
    pub dagannoth_prime: Boss,
    pub dagannoth_rex: Boss,
    pub dagannoth_supreme: Boss,
    pub deranged_archaeologist: Boss,
    pub general_graardor: Boss,
    pub giant_mole: Boss,
    pub grotesque_guardians: Boss,
    pub hespori: Boss,
    pub kalphite_queen: Boss,
    pub king_black_dragon: Boss,
    pub kraken: Boss,
    pub kreearra: Boss,        // Kree'arra
    pub kril_tsutsaroth: Boss, // K'ril Tsutsaroth
    pub mimic: Boss,
    pub nex: Boss,
    pub nightmare: Boss,
    pub phosanis_nightmare: Boss, // Phosani's Nightmare
    pub obor: Boss,
    pub sarachnis: Boss,
    pub scorpia: Boss,
    pub skotizo: Boss,
    pub tempoross: Boss,
    pub the_gauntlet: Boss,
    pub the_corrupted_gauntlet: Boss,
    pub theatre_of_blood: Boss,
    pub theatre_of_blood_hard_mode: Boss,
    pub thermonuclear_smoke_devil: Boss,
    pub tombs_of_amascut: Boss,
    pub tombs_of_amascut_expert_mode: Boss,
    pub tzkal_zuk: Boss,
    pub tztok_jad: Boss,
    pub venenatis: Boss,
    pub vetion: Boss, // Vet'ion
    pub vorkath: Boss,
    pub wintertodt: Boss,
    pub zalcano: Boss,
    pub zulrah: Boss,
}

impl Bosses {
    pub fn build_bosses(bosses: Vec<Boss>) -> Bosses {
        Bosses {
            abyssal_sire: bosses[0],
            alchemical_hydra: bosses[1],
            barrows_chests: bosses[2],
            bryophyta: bosses[3],
            callisto: bosses[4],
            cerberus: bosses[5],
            chambers_of_xeric: bosses[6],
            chambers_of_xeric_challenge_mode: bosses[7],
            chaos_elemental: bosses[8],
            chaos_fanatic: bosses[9],
            commander_zilyana: bosses[10],
            corporeal_beast: bosses[11],
            crazy_archaeologist: bosses[12],
            dagannoth_prime: bosses[13],
            dagannoth_rex: bosses[14],
            dagannoth_supreme: bosses[15],
            deranged_archaeologist: bosses[16],
            general_graardor: bosses[17],
            giant_mole: bosses[18],
            grotesque_guardians: bosses[19],
            hespori: bosses[20],
            kalphite_queen: bosses[21],
            king_black_dragon: bosses[22],
            kraken: bosses[23],
            kreearra: bosses[24],
            kril_tsutsaroth: bosses[25],
            mimic: bosses[26],
            nex: bosses[27],
            nightmare: bosses[28],
            phosanis_nightmare: bosses[29],
            obor: bosses[30],
            sarachnis: bosses[31],
            scorpia: bosses[32],
            skotizo: bosses[33],
            tempoross: bosses[34],
            the_gauntlet: bosses[35],
            the_corrupted_gauntlet: bosses[36],
            theatre_of_blood: bosses[37],
            theatre_of_blood_hard_mode: bosses[38],
            thermonuclear_smoke_devil: bosses[39],
            tombs_of_amascut: bosses[40],
            tombs_of_amascut_expert_mode: bosses[41],
            tzkal_zuk: bosses[42],
            tztok_jad: bosses[43],
            venenatis: bosses[44],
            vetion: bosses[45],
            vorkath: bosses[46],
            wintertodt: bosses[47],
            zalcano: bosses[48],
            zulrah: bosses[49],
        }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct Hiscore {
    pub combatlevel: u16,
    pub skills: Skills,
    pub minigames: Minigames,
    pub bosses: Bosses,
}

impl Hiscore {
    pub fn build_hiscore(skills: Skills, minigames: Minigames, bosses: Bosses) -> Hiscore {
        Hiscore {
            combatlevel: util::combatlvl::combatlevel(&skills),
            skills,
            minigames,
            bosses,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
