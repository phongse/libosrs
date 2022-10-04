# libosrs

An Oldschool Runescape API library written in Rust. Can serialize the output into JSON format.

## Examples

```rust
use libosrs;

#[tokio::main]
async fn main() {
    let client = libosrs::ClientOSRS::new();

    let soupshi = &client.get_hiscore("Soupshi", "regular").await.unwrap();

    println!("Overall: {:#?}", soupshi.skills.overall);

    println!("Agility level: {}", soupshi.skills.agility.level);
    println!("Agility xp: {}", soupshi.skills.agility.xp);

    println!("Vorkath rank: {}", soupshi.bosses.vorkath.rank);
    println!("Vorkath kills: {}", soupshi.bosses.vorkath.score);

    println!(
        "JSON output: {}",
        &client.get_hiscore_json("Soupshi", "regular").await
    );
}
```

## Game modes

```
"regular"
"ironman"
"hardcore"
"ultimate"
```
Anything else will default to regular game mode.

## Todo
- [ ] Blocking IO
- [ ] Serialize to json without re-calling Runescape API
- [ ] Formatters
- [ ] Grand Exchange
