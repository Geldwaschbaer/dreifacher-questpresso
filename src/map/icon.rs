use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
#[repr(u8)]
pub enum MapIcon {
    Boss,
    Endboss,
    Enemy,
    Mystery,
    Shop,
    Start,
    Exit,
}

impl MapIcon {
    pub fn ordinal(&self) -> usize {
        *self as usize
    }
}
