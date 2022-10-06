# libosrs

An Oldschool Runescape API library written in Rust. Can serialize the output into JSON format.

## Examples

```rust
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

```

## Game modes

```
"auto"
"regular"
"ironman"
"hardcore"
"ultimate"
"deadman"
"seasonal"
"tournament"
```
`auto` will try to fetch a players current gamemode from `regular`, `ironman`, `hardcore` and `ultimate`, i.e. a player who used to be Ironman but converted to regular will output their regular game stats. Only works if they gained Xp or Boss scores after convertion.
Any other values for gamemode will fallback to `regular`.

## Struct names
See the [docs.rs](https://docs.rs/libosrs/0.1.1/libosrs/hiscore/index.html) page.

## Todo
- [ ] Blocking IO
- [x] Serialize to json without re-calling Runescape API
- [ ] Formatters
- [ ] Grand Exchange
