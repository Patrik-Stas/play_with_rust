use std::collections::HashMap;

#[derive(Debug)]
struct Player {
    score: i32
}

impl Player {
    pub fn set_score(&mut self, new_score: i32) {
        self.score += new_score
    }

    pub fn get_score(&self) -> i32  {
        self.score
    }

    pub fn new() -> Player {
        Player { score: 0 }
    }
}


pub fn run()
{
    let mut p = Player::new();
    p.set_score(p.get_score() + 1);
    println!("Player status = {:?}", p);

}