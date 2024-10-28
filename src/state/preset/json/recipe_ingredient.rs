#[derive(Debug)]
pub struct RecipeIngredient<'a> {
    pub ty: &'a str,
    pub name: &'a str,
    pub amount: f32,
    pub catalyst_amount: Option<f32>,
    pub minimum_temperature: Option<f32>,
    pub maximum_temperature: Option<f32>,
}

impl<'a, 'de: 'a> serde::de::Deserialize<'de> for RecipeIngredient<'a> {
    fn deserialize<D>(deserializer: D) -> Result<RecipeIngredient<'a>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a> {
            _marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'a, 'de: 'a> serde::de::Visitor<'de> for Visitor<'a> {
            type Value = RecipeIngredient<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map of recipe ingredients")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let item_name = seq.next_element::<&'de str>()?;
                let item_amount = seq.next_element::<f32>()?;
                Ok(RecipeIngredient {
                    ty: "item",
                    name: item_name.ok_or_else(|| serde::de::Error::missing_field("item name"))?,
                    amount: item_amount
                        .ok_or_else(|| serde::de::Error::missing_field("item amount"))?,
                    catalyst_amount: None,
                    minimum_temperature: None,
                    maximum_temperature: None,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut ingredient_type = None;
                let mut name = None;
                let mut amount = None;
                let mut catalyst_amount = None;
                let mut minimum_temperature = None;
                let mut maximum_temperature = None;
                while let Some(key) = map.next_key::<&'de str>()? {
                    match key {
                        "type" => {
                            ingredient_type = Some(map.next_value()?);
                        }
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "amount" => {
                            amount = Some(map.next_value()?);
                        }
                        "catalyst_amount" => {
                            catalyst_amount = Some(map.next_value()?);
                        }
                        "minimum_temperature" => {
                            minimum_temperature = Some(map.next_value()?);
                        }
                        "maximum_temperature" => {
                            maximum_temperature = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                key,
                                &[
                                    "type",
                                    "name",
                                    "amount",
                                    "catalyst_amount",
                                    "minimum_temperature",
                                    "maximum_temperature",
                                ],
                            ));
                        }
                    }
                }
                Ok(RecipeIngredient {
                    ty: ingredient_type.unwrap_or("item"),
                    name: name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    amount: amount.ok_or_else(|| serde::de::Error::missing_field("amount"))?,
                    catalyst_amount,
                    minimum_temperature,
                    maximum_temperature,
                })
            }
        }

        deserializer.deserialize_any(Visitor {
            _marker: std::marker::PhantomData,
        })
    }
}
