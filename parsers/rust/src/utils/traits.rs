use std::collections::HashMap;

use serde_json::Value;

#[cfg(test)]
mod tests;

pub trait To5etools {
    fn to_5etools_base(&self) -> Value;
    fn to_5etools_spell(&self) -> Value {
        self.to_5etools_base()
    }
    fn to_5etools_creature(&self) -> Value {
        self.to_5etools_base()
    }
}

macro_rules! impl_to5etools_string {
    ($($t:ty),*) => {
        $(
        impl To5etools for $t {
            fn to_5etools_base(&self) -> Value {
                Value::String(self.to_string())
            }
        }
        )*
    };
}

macro_rules! impl_to5etools_number {
    ($($t:ty),*) => {
        $(
        impl To5etools for $t {
            fn to_5etools_base(&self) -> Value {
                Value::Number(serde_json::value::Number::from(*self))
            }
        }
        )*
    };
}

impl_to5etools_string!(&str, String);
impl_to5etools_number!(u8, u16, u32, i8, i16, i32);

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

    fn to_5etools_creature(&self) -> Value {
        self.iter()
            .map(To5etools::to_5etools_creature)
            .collect::<Value>()
    }
}

impl<K: To5etools, V: To5etools> To5etools for HashMap<K, V> {
    fn to_5etools_base(&self) -> Value {
        Value::Object(
            self.iter()
                .map(|(key, value)| {
                    (
                        key.to_5etools_base()
                            .as_str()
                            .expect("HashMap key did not serialize to a string.")
                            .to_owned(),
                        value.to_5etools_base(),
                    )
                })
                .collect::<serde_json::Map<String, Value>>(),
        )
    }
    fn to_5etools_spell(&self) -> Value {
        Value::Object(
            self.iter()
                .map(|(key, value)| {
                    (
                        key.to_5etools_spell()
                            .as_str()
                            .expect("HashMap key did not serialize to a string.")
                            .to_owned(),
                        value.to_5etools_spell(),
                    )
                })
                .collect::<serde_json::Map<String, Value>>(),
        )
    }
    fn to_5etools_creature(&self) -> Value {
        Value::Object(
            self.iter()
                .map(|(key, value)| {
                    (
                        key.to_5etools_creature()
                            .as_str()
                            .expect("HashMap key did not serialize to a string.")
                            .to_owned(),
                        value.to_5etools_creature(),
                    )
                })
                .collect::<serde_json::Map<String, Value>>(),
        )
    }
}
