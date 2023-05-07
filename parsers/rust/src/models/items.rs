#[allow(dead_code)]
#[derive(Debug)]
pub enum Currency {
    Copper,
    Silver,
    Electrum,
    Gold,
    Platinum,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ItemValue {
    value: u32,
    unit: Currency,
}
