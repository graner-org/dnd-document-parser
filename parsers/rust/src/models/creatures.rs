use crate::utils::traits::To5etools;
use serde_json::{json, Value};

#[cfg(test)]
mod tests;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl To5etools for Size {
    fn to_5etools_base(&self) -> Value {
        use Size::{Gargantuan, Huge, Large, Medium, Small, Tiny};
        Value::String(
            match self {
                Tiny => "T",
                Small => "S",
                Medium => "M",
                Large => "L",
                Huge => "H",
                Gargantuan => "G",
            }
            .to_owned(),
        )
    }
}
