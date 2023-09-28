pub struct HitDice {
    pub hit_dice_type: HitDieType,
    pub quantity: u8,
}

pub enum HitDieType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}
