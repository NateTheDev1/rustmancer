use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Health {
    current: i32,
    max: i32,
    temporary: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self {
            current: max,
            max,
            temporary: 0,
        }
    }

    pub fn current(&self) -> i32 {
        self.current
    }

    pub fn max(&self) -> i32 {
        self.max
    }

    pub fn temporary(&self) -> i32 {
        self.temporary
    }

    pub fn set_current(&mut self, current: i32) {
        self.current = current;
    }

    pub fn set_max(&mut self, max: i32) {
        self.max = max;
    }

    pub fn set_temporary(&mut self, temporary: i32) {
        self.temporary = temporary;
    }

    pub fn damage(&mut self, damage: i32) {
        self.current -= damage;
    }

    pub fn heal(&mut self, heal: i32) {
        self.current += heal;
    }
}
