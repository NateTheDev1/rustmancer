use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Sense {
    /// The maximum distance at which the sense can be used.
    pub distance: f32,
    // The name of the sense (e.g. Darkvision, Blindsight, etc.).
    pub name: String,
}
