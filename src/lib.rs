use std::error::Error;

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
                        rank: (&row[0]).parse::<u64>().unwrap(),
                        level: (&row[1]).parse::<u64>().unwrap(),
                        xp: (&row[2]).parse::<u64>().unwrap(),
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

    pub async fn get_hiscore(&self, name: &str, gamemode: &str) -> Result<Hiscore, Box<dyn Error>> {
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
            _ => Err("Something went wrong with the request.".into()),
        }
    }

    pub async fn get_hiscore_json(&self, name: &str, gamemode: &str) -> String {
        let hiscore = match self.get_hiscore(name, gamemode).await {
            Ok(hiscore) => hiscore,
            Err(e) => panic!("{}", e),
        };

        let hiscore_json = serde_json::to_string(&hiscore).unwrap();
        hiscore_json
    }
}
