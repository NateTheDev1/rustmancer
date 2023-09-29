use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SavingThrow {
    /// The name of the ability that this saving throw is based on.
    pub name: String,
    /// The difficulty class for this saving throw.
    pub difficulty: i32,
}
