use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Die {
    pub count: i32,
    pub sides: i32,
}
