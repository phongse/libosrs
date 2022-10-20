use crate::hiscore::Skills;
// Calculate combat level
// https://oldschool.runescape.wiki/w/Combat_levels

fn base(defence: f32, hitpoints: f32, prayer: f32) -> f32 {
    (1f32 / 4f32) * (defence + hitpoints + (prayer * (1 / 2) as f32).floor())
}

fn melee(attack: f32, strength: f32) -> f32 {
    (13f32 / 40f32) * (attack + strength)
}

fn ranged(ranged: f32) -> f32 {
    (13f32 / 40f32) * (ranged * (3 / 2) as f32).floor()
}

fn magic(magic: f32) -> f32 {
    (13f32 / 40f32) * (magic * (3 / 2) as f32).floor()
}

pub fn combatlevel(skills: &Skills) -> u16 {
    let baselevel: f32 = base(
        skills.defence.level as f32,
        skills.hitpoints.level as f32,
        skills.prayer.level as f32,
    );
    println!("base: {}", baselevel);

    let meleelevel: f32 = melee(skills.attack.level as f32, skills.strength.level as f32);
    println!("melee: {}", meleelevel);

    let rangedlevel: f32 = ranged(skills.ranged.level as f32);
    println!("ranged: {}", rangedlevel);

    let magiclevel: f32 = magic(skills.magic.level as f32);
    println!("magic: {}", magiclevel);

    let not_melee;
    if rangedlevel >= magiclevel {
        not_melee = rangedlevel;
    } else {
        not_melee = magiclevel;
    }
    println!("{}", (baselevel + f32::max(meleelevel, not_melee)).floor());

    return (baselevel + f32::max(meleelevel, not_melee)).floor() as u16;
}
