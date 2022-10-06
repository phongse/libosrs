use std::error::Error;

use async_recursion::async_recursion;
use csv::Reader;
use reqwest::StatusCode;

mod constants;
pub mod hiscore;
pub mod skill;
use crate::hiscore::{Bosses, Hiscore, Minigames, Skills};
use crate::skill::{Boss, Minigame, Skill};

pub struct ClientOSRS {
    req_client: reqwest::Client,
}
impl ClientOSRS {
    pub fn new() -> ClientOSRS {
        let req_client = reqwest::Client::new();
        let client = ClientOSRS { req_client };
        client
    }

    pub async fn get_player_gamemode(&self, name: &str) -> &str {
        let regular = self.get_hiscore(name, "regular").await.unwrap().skills;
        let regular_xp = Skills::get_overall_xp(regular);

        let ironman = self.get_hiscore(name, "ironman").await.unwrap().skills;
        let ironman_xp = Skills::get_overall_xp(ironman);

        if ironman_xp < regular_xp {
            return "regular";
        }

        let hardcore = self.get_hiscore(name, "hardcore").await.unwrap().skills;
        let hardcore_xp = Skills::get_overall_xp(hardcore);

        if hardcore_xp >= ironman_xp {
            return "hardcore";
        }

        let ultimate = self.get_hiscore(name, "ultimate").await.unwrap().skills;
        let ultimate_xp = Skills::get_overall_xp(ultimate);

        if ultimate_xp >= ironman_xp {
            return "ultimate";
        }

        // if neither hardcore or ultimate, then fallback to ironman
        "ironman"
    }

    fn read_csv(res: &String) -> Reader<&[u8]> {
        csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(res.as_bytes())
    }

    async fn parse_records(
        mut rdr: Reader<&[u8]>,
    ) -> Result<(Vec<Skill>, Vec<Minigame>, Vec<Boss>), Box<dyn Error>> {
        let mut skills: Vec<Skill> = Vec::new();
        let mut minigames: Vec<Minigame> = Vec::new();
        let mut bosses: Vec<Boss> = Vec::new();

        for (index, result) in rdr.records().enumerate() {
            let row = result?;
            match index {
                0..=23 => {
                    let skill: Skill = Skill {
                        rank: match (&row[0]).parse::<u64>() {
                            Ok(x) => x,
                            Err(_) => 0,
                        },
                        level: match (&row[1]).parse::<u64>() {
                            Ok(x) => x,
                            Err(_) => 0,
                        },
                        xp: match (&row[2]).parse::<u64>() {
                            Ok(x) => x,
                            Err(_) => 0,
                        },
                    };
                    skills.push(skill);
                }
                24..=37 => {
                    let minigame: Minigame = Minigame {
                        rank: match (&row[0]).parse::<u64>() {
                            Ok(x) => x,
                            Err(_) => 0,
                        },
                        score: match (&row[1]).parse::<u64>() {
                            Ok(x) => x,
                            Err(_) => 0,
                        },
                    };
                    minigames.push(minigame);
                }
                38..=87 => {
                    let boss: Boss = Boss {
                        rank: match (&row[0]).parse::<u64>() {
                            Ok(x) => x,
                            Err(_) => 0,
                        },
                        score: match (&row[1]).parse::<u64>() {
                            Ok(x) => x,
                            Err(_) => 0,
                        },
                    };
                    bosses.push(boss);
                }
                _ => panic!("A skill, minigame or boss has been added to Oldschool Runescape. Libosrs needs to updated."),
            }
        }
        Ok((skills, minigames, bosses))
    }

    #[async_recursion]
    pub async fn get_hiscore(&self, name: &str, gamemode: &str) -> Result<Hiscore, Box<dyn Error>> {

        let gamemode: &str = match gamemode {
            "auto" => self.get_player_gamemode(name).await,
            _ => gamemode,
        };

        let url = format!("{}{}", constants::get_gamemode(gamemode), name);
        let response = self.req_client.get(url).send().await?;

        match response.status() {
            StatusCode::OK => {
                let res = response.text().await?;

                let rdr = Self::read_csv(&res);
                let (skills, minigames, bosses) = Self::parse_records(rdr).await?;

                Ok(Hiscore::build_hiscore(
                    Skills::build_skills(skills),
                    Minigames::build_minigames(minigames),
                    Bosses::build_bosses(bosses),
                ))
            }
            _ => Ok(Hiscore {
                ..Default::default()
            }),
        }
    }

    pub async fn get_hiscore_json(&self, name: &str, gamemode: &str) -> String {
        let hiscore = match self.get_hiscore(name, gamemode).await {
            Ok(hiscore) => hiscore,
            Err(_) => Hiscore {
                ..Default::default()
            },
        };

        hiscore.to_json()
    }
}
