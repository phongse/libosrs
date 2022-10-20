# libosrs

An Oldschool RuneScape API library written in Rust. Can serialize the output into JSON format.

## Examples

```rust
use libosrs;
use libosrs::gamemode::Gamemode;

#[tokio::main]
async fn main() {
    let name = "Soupshi";
    let mut gamemode: Gamemode = Gamemode::Auto;

    let client = libosrs::ClientOSRS::new();

    let soupshi = client.get_hiscore(name, gamemode).await.unwrap();

    println!("Overall: {:#?}", soupshi.skills.overall);
    println!("Agility level: {}", soupshi.skills.agility.level);
    println!("Agility xp: {}", soupshi.skills.agility.xp);
    println!("Vorkath rank: {}", soupshi.bosses.vorkath.rank);
    println!("Vorkath kills: {}", soupshi.bosses.vorkath.score);

    // Pass a Hiscore struct and return json
    println!("JSON output: {}", soupshi.to_json());

    // Try to retrieve a players current gamemode
    gamemode = client.get_player_gamemode(name).await;
    println!("{:#?}", gamemode);

    // Request hiscore from API and return json
    println!(
        "JSON output: {}",
        client.get_hiscore_json(name, gamemode).await
    );

    // Get Player struct and Player json
    println!("{:#?}", client.get_player(name, gamemode).await);
    println!(
        "JSON output: {}",
        client.get_player_json(name, gamemode).await
    );
}

```

## Game modes

```
Gamemode::Auto
Gamemode::Regular
Gamemode::Ironman
Gamemode::Hardcore
Gamemode::Ultimate
Gamemode::Deadman
Gamemode::Seasonal
Gamemode::Tournament
```

`Auto` will try to fetch a player's current game mode from `Regular`, `Ironman`, `Hardcore` and `Ultimate`, i.e. a player who used to be Iron Man but converted to regular will output their regular game stats. Only works if they gained Xp or Boss scores after conversion.

## Struct names

See the [docs.rs](https://docs.rs/libosrs/latest/libosrs/hiscore/index.html) page.

## Todo

- [ ] Blocking IO
- [x] Serialize to json without re-calling RuneScape API
- [ ] Formatters
- [ ] Grand Exchange
