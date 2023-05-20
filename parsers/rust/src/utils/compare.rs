use std::format;

use itertools::{EitherOrBoth, Itertools};
use serde_json::Value;

use super::error::JSONDiffError;

pub fn json_compare(json1: Value, json2: Value) -> Result<(), JSONDiffError> {
    fn json_compare_helper(
        json1: Value,
        json2: Value,
        json_path: String,
    ) -> Result<(), JSONDiffError> {
        use Value::{Array, Bool, Null, Number, Object};
        match (json1, json2) {
            (Null, Null) => Ok(()),
            (Bool(val1), Bool(val2)) => {
                if val1 == val2 {
                    Ok(())
                } else {
                    Err(JSONDiffError {
                        json1: Bool(val1),
                        json2: Bool(val2),
                        json_path,
                    })
                }
            }
            (Number(val1), Number(val2)) => {
                if val1 == val2 {
                    Ok(())
                } else {
                    Err(JSONDiffError {
                        json1: Number(val1),
                        json2: Number(val2),
                        json_path,
                    })
                }
            }
            (Value::String(val1), Value::String(val2)) => {
                if val1 == val2 {
                    Ok(())
                } else {
                    Err(JSONDiffError {
                        json1: Value::String(val1),
                        json2: Value::String(val2),
                        json_path,
                    })
                }
            }
            (Array(val1), Array(val2)) => val1
                .into_iter()
                .zip_longest(val2.into_iter())
                .enumerate()
                .find_map(|(i, zip_val)| {
                    use EitherOrBoth::*;
                    let new_json_path = format!("{json_path}[{i}]");
                    let comparison = match zip_val {
                        Both(val1, val2) => json_compare_helper(val1, val2, new_json_path),
                        Left(val1) => Err(JSONDiffError {
                            json1: val1,
                            json2: Null,
                            json_path: new_json_path,
                        }),
                        Right(val2) => Err(JSONDiffError {
                            json1: Null,
                            json2: val2,
                            json_path: new_json_path,
                        }),
                    };
                    match comparison {
                        Ok(_) => None,
                        error => Some(error),
                    }
                })
                .unwrap_or(Ok(())),
            (Object(map1), Object(map2)) => {
                let mut zipped = map1.into_iter().zip_longest(map2.into_iter());
                zipped
                    .find_map(|zip_val| {
                        use EitherOrBoth::*;
                        let comparison = match zip_val {
                            Both((k1, v1), (k2, v2)) => match k1 == k2 {
                                true => json_compare_helper(v1, v2, format!("{json_path}.{k1}")),
                                false => Err(JSONDiffError {
                                    json1: v1,
                                    json2: Null,
                                    json_path: format!("{json_path}.{k1}"),
                                }),
                            },
                            Left((k1, v1)) => Err(JSONDiffError {
                                json1: v1,
                                json2: Null,
                                json_path: format!("{json_path}.{k1}"),
                            }),
                            Right((k2, v2)) => Err(JSONDiffError {
                                json1: Null,
                                json2: v2,
                                json_path: format!("{json_path}.{k2}"),
                            }),
                        };
                        match comparison {
                            Ok(_) => None,
                            error => Some(error),
                        }
                    })
                    .unwrap_or(Ok(()))
            }
            // different enum types
            (json1, json2) => Err(JSONDiffError {
                json1,
                json2,
                json_path,
            }),
        }
    }
    json_compare_helper(json1, json2, "<root>".to_owned())
}
