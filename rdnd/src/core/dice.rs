use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Die {
    pub count: i32,
    pub sides: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HitDie {
    pub die: Die,
    pub count: i32,
}
