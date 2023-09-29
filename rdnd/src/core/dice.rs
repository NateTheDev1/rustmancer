use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Die {
    sides: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Dice {
    dice: Vec<Die>,
}

impl Die {
    pub fn new(sides: u8) -> Self {
        Self { sides }
    }
}

impl Dice {
    pub fn new() -> Self {
        Self { dice: Vec::new() }
    }

    pub fn add_die(&mut self, die: Die) {
        self.dice.push(die);
    }
}
