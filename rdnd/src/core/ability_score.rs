use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AbilityScore {
    pub identifier: String,
    pub value: i32,
}
