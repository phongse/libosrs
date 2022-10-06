use libosrs;

#[tokio::main]
async fn main() {
    let name = "Soupshi";
    let mut gamemode = "auto";

    let client = libosrs::ClientOSRS::new();

    let soupshi = &client.get_hiscore(name, gamemode).await.unwrap();

    println!("Overall: {:#?}", soupshi.skills.overall);
    println!("Agility level: {}", soupshi.skills.agility.level);
    println!("Agility xp: {}", soupshi.skills.agility.xp);
    println!("Vorkath rank: {}", soupshi.bosses.vorkath.rank);
    println!("Vorkath kills: {}", soupshi.bosses.vorkath.score);

    // Pass a Hiscore struct and return json
    println!("JSON output: {}", soupshi.to_json());

    // Try to retrieve a players current gamemode
    gamemode = &client.get_player_gamemode(name).await;
    println!("{}", gamemode);

    // Request hiscore from API and return json
    println!(
        "JSON output: {}",
        &client.get_hiscore_json(name, gamemode).await
    );
}
