#[derive(Debug, serde::Serialize, Clone, Copy)]
pub struct Unit {
    amount: f32,
    unit_type: UnitType,
}

impl<'de> serde::de::Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Unit, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Unit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string unit")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut amount = None;
                let mut unit_type = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "amount" => amount = Some(map.next_value()?),
                        "unit_type" => unit_type = Some(map.next_value()?),
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                key,
                                &["amount", "unit_type"],
                            ))
                        }
                    }
                }

                match (amount, unit_type) {
                    (Some(amount), Some(unit_type)) => Ok(Unit { amount, unit_type }),
                    _ => Err(serde::de::Error::missing_field("amount or unit_type")),
                }
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let idx = v
                    .find(|c: char| !c.is_ascii_digit() && c != '.')
                    .unwrap_or(v.len());

                let amount = v[..idx].parse().map_err(serde::de::Error::custom)?;
                let unit_type = v[idx..].parse().map_err(|_| {
                    serde::de::Error::unknown_variant(v[idx..].trim(), UnitType::all())
                })?;

                Ok(Unit { amount, unit_type })
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UnitType {
    J,
    KJ,
    MJ,
    GJ,
    YJ,

    W,
    KW,
    MW,
    GW,
}

impl UnitType {
    pub fn all() -> &'static [&'static str] {
        &["J", "kJ", "MJ", "GJ", "YJ", "W", "kW", "MW", "GW"]
    }
}

impl std::str::FromStr for UnitType {
    type Err = ();

    fn from_str(s: &str) -> Result<UnitType, ()> {
        match s.trim().to_uppercase().as_str() {
            "J" => Ok(UnitType::J),
            "KJ" => Ok(UnitType::KJ),
            "MJ" => Ok(UnitType::MJ),
            "GJ" => Ok(UnitType::GJ),
            "YJ" => Ok(UnitType::YJ),
            "W" => Ok(UnitType::W),
            "KW" => Ok(UnitType::KW),
            "MW" => Ok(UnitType::MW),
            "GW" => Ok(UnitType::GW),
            _ => Err(()),
        }
    }
}
