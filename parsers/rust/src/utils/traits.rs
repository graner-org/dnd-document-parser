use serde_json::Value;

pub trait To5etools {
    fn to_5etools_base(&self) -> Value;
    fn to_5etools_spell(&self) -> Value {
        self.to_5etools_base()
    }
}

impl<T: To5etools + Copy> To5etools for Vec<T> {
    fn to_5etools_base(&self) -> Value {
        self.iter()
            .map(To5etools::to_5etools_base)
            .collect::<Value>()
    }

    fn to_5etools_spell(&self) -> Value {
        self.iter()
            .map(To5etools::to_5etools_spell)
            .collect::<Value>()
    }
}
