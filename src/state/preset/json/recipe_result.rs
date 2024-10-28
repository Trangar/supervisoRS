#[derive(Debug)]
pub struct RecipeResult<'a> {
    pub ty: &'a str,
    pub item: &'a str,
    pub amount: Option<f32>,
    pub probability: Option<f32>,
    pub amount_min: Option<f32>,
    pub amount_max: Option<f32>,
    pub fluidbox_index: Option<u8>,
    pub catalyst_amount: Option<f32>,
    pub temperature: Option<f32>,
}

impl<'a, 'de: 'a> serde::de::Deserialize<'de> for RecipeResult<'a> {
    fn deserialize<D>(deserializer: D) -> Result<RecipeResult<'a>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a> {
            _marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'a, 'de: 'a> serde::de::Visitor<'de> for Visitor<'a> {
            type Value = RecipeResult<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map of recipe results")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let name = seq
                    .next_element::<&'de str>()?
                    .ok_or_else(|| serde::de::Error::missing_field("result seq name"))?;
                let amount = seq
                    .next_element::<f32>()?
                    .ok_or_else(|| serde::de::Error::missing_field("result seq amount"))?;
                Ok(RecipeResult {
                    ty: "item",
                    item: name,
                    amount: Some(amount),
                    probability: None,
                    fluidbox_index: None,
                    catalyst_amount: None,
                    amount_min: None,
                    amount_max: None,
                    temperature: None,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut result_type = None;
                let mut name = None;
                let mut amount = None;
                let mut fluidbox_index = None;
                let mut catalyst_amount = None;
                let mut probability = None;
                let mut amount_min = None;
                let mut amount_max = None;
                let mut temperature = None;
                while let Some(key) = map.next_key::<&'de str>()? {
                    match key {
                        "type" => {
                            result_type = Some(map.next_value()?);
                        }
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "amount" => {
                            amount = Some(map.next_value()?);
                        }
                        "fluidbox_index" => {
                            fluidbox_index = Some(map.next_value()?);
                        }
                        "catalyst_amount" => {
                            catalyst_amount = Some(map.next_value()?);
                        }
                        "probability" => {
                            probability = Some(map.next_value()?);
                        }
                        "amount_min" => {
                            amount_min = Some(map.next_value()?);
                        }
                        "amount_max" => {
                            amount_max = Some(map.next_value()?);
                        }
                        "temperature" => {
                            temperature = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                key,
                                &[
                                    "type",
                                    "name",
                                    "amount",
                                    "fluidbox_index",
                                    "catalyst_amount",
                                    "probability",
                                    "amount_min",
                                    "amount_max",
                                    "temperature",
                                ],
                            ));
                        }
                    }
                }

                Ok(RecipeResult {
                    ty: result_type.unwrap_or("item"),
                    item: name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    amount,
                    fluidbox_index,
                    catalyst_amount,
                    probability,
                    amount_min,
                    amount_max,
                    temperature,
                })
            }
        }

        deserializer.deserialize_any(Visitor {
            _marker: std::marker::PhantomData,
        })
    }
}
