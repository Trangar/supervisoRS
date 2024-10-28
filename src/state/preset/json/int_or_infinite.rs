#[derive(Debug)]
pub enum IntOrInfinite {
    Int(usize),
    Infinite,
}

impl<'de> serde::de::Deserialize<'de> for IntOrInfinite {
    fn deserialize<D>(deserializer: D) -> Result<IntOrInfinite, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IntOrInfinite;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer or the string \"infinite\"")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(IntOrInfinite::Int(v as usize))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v == "infinite" {
                    Ok(IntOrInfinite::Infinite)
                } else if let Ok(v) = v.parse() {
                    Ok(IntOrInfinite::Int(v))
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(v),
                        &"an integer or the string \"infinite\"",
                    ))
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}
