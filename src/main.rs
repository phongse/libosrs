use libosrs;

#[tokio::main]
async fn main() {
    let client = libosrs::ClientOSRS::new();

    let soupshi = &client.get_hiscore("Soupshi").await.unwrap();

    println!("Overall: {:#?}", soupshi.skills.overall);

    println!("Agility level: {}", soupshi.skills.agility.level);
    println!("Agility xp: {}", soupshi.skills.agility.xp);

    println!("Vorkath rank: {}", soupshi.bosses.vorkath.rank);
    println!("Vorkath kills: {}", soupshi.bosses.vorkath.score);

    println!("JSON output: {}", &client.get_hiscore_json("Soupshi").await);
}
