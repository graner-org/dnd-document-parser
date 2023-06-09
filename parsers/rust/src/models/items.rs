use crate::utils::traits::To5etools;
use serde_json::{json, Value};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Copper,
    Silver,
    Electrum,
    Gold,
    Platinum,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemValue {
    pub value: u32,
    pub unit: Currency,
}

impl To5etools for ItemValue {
    fn to_5etools_base(&self) -> Value {
        use Currency::{Copper, Electrum, Gold, Platinum, Silver};
        json!(
            self.value
                * match self.unit {
                    Copper => 1,
                    Silver => 10,
                    Electrum => 50,
                    Gold => 100,
                    Platinum => 1000,
                }
        )
    }
}
