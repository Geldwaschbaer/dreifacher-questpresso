use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub enum Stat {
    Str,
    Dex,
    Con,
    Int,
}
