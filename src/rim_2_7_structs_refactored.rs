use std::collections::HashMap;

#[derive(Debug)]
struct Stats {
    hp: u8,
    sp: u8,
}

#[derive(Debug)]
struct Monster {
    stats: Stats,
    friends: Vec<Friend>,
}

#[derive(Debug)]
struct Friend {
    loyalty: u8
}

impl Monster {
    pub fn final_breath(&mut self) {
        if let Some(friend) = self.friends.first() {
            self.stats.heal();
            println!("Healing for {}.", friend.loyalty)
        }
    }
}

impl Stats {
    pub fn heal(&mut self) {
        self.hp += 10;
        self.sp -= 10;
    }
}


pub fn run()
{
    let f1 = Friend {loyalty: 10};
    let mut monster = Monster { stats: Stats { hp: 15, sp: 12 }, friends: vec![f1]};
    monster.final_breath()
}