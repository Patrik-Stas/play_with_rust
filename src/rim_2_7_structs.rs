use std::collections::HashMap;

#[derive(Debug)]
struct Monster {
    hp: u8,
    sp: u8,
    friends: Vec<Friend>,
}

#[derive(Debug)]
struct Friend {
    loyalty: u8
}

impl Monster {
    pub fn final_breath(&mut self) {
        if let Some(friend) = self.friends.first() {
            self.heal();
            // the heal method take mutable reference to all self. It assumes that heal method might modify
            // friends as well. But friends is already borrowed immutably, and we are using that immutable
            // reference on the next line. While a "thing" is borrowed immutably, it cannot be
            // borrowed mutably (which indirectly happens through using self mutably)
            println!("Healing for {}.", friend.loyalty)
        }
    }

    pub fn heal(&mut self) {
        self.hp += 10;
        self.sp -= 10;
    }
}

pub fn run()
{
    let f1 = Friend {loyalty: 10};
    let mut monster = Monster { hp: 15, sp: 12, friends: vec![f1]};
    monster.final_breath()
}