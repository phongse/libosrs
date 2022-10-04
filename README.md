# libosrs

An Oldschool Runescape API library written in Rust. Can serialize the output into JSON format.

## Examples

```Rust
use libosrs;

#[tokio::main]
async fn main() {
    // create a new client
    let client = libosrs::ClientOSRS::new();

    // get player hiscore
    let soupshi = &client.get_hiscore("Soupshi").await.unwrap();

    println!("Overall: {:#?}", soupshi.skills.overall);

    println!("Agility level: {}", soupshi.skills.agility.level);
    println!("Agility xp: {}", soupshi.skills.agility.xp);

    println!("Vorkath rank: {}", soupshi.bosses.vorkath.rank);
    println!("Vorkath kills: {}", soupshi.bosses.vorkath.score);

    // get player hiscore in json
    let soupshi_json = &client.get_hiscore_json("Soupshi").await;
    println!("JSON output: {}", soupshi_json);
}

```

## Todo
- [ ] Blocking IO
- [ ] Serialize to json without re-calling Runescape API
- [ ] Formatters
- [ ] Grand Exchange
