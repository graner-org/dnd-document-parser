use serde_json::Value;

pub trait To5etools {
    fn to_5etools(&self) -> Value;
    fn to_5etools_spell(&self) -> Value {
        self.to_5etools()
    }
}

impl<T: To5etools + Copy> To5etools for Vec<T> {
    fn to_5etools(&self) -> Value {
        self.iter()
            .map(|value| value.to_5etools())
            .collect::<Value>()
            .into()
    }

    fn to_5etools_spell(&self) -> Value {
        self.iter()
            .map(|value| value.to_5etools_spell())
            .collect::<Value>()
            .into()
    }
}
