#[derive(Debug)]
pub enum Currency {
    Copper,
    Silver,
    Electrum,
    Gold,
    Platinum,
}

#[derive(Debug)]
pub struct ItemValue {
    value: u32,
    unit: Currency,
}
