use std::f32::consts;

static MAX_HEALTH: i32 = 100;
static GAME_NAME: &str = "Monster Attack";

type MagicPower = u16;

fn main() {
    println!("Game: {game}", game=GAME_NAME);
    println!("Max Health: {health}", health=MAX_HEALTH);
    println!("PI: {}", consts::PI);

    let level_title: &str = "Level 1";
    let mut energy: i32 = 5;
    let dead: bool = false;

    energy = 10;

    let score: i32 = 10;
    let score: &str = "You Win!";

    let run: MagicPower = 7800;
}
