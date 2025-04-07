use super::FluidBox;

#[derive(Debug, Default)]
pub struct FluidBoxes<'a> {
    pub boxes: Vec<FluidBox<'a>>,
    pub off_when_no_fluid_recipe: bool,
}

impl<'de: 'a, 'a> serde::de::Deserialize<'de> for FluidBoxes<'a> {
    fn deserialize<D>(deserializer: D) -> Result<FluidBoxes<'a>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a> {
            _marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> serde::de::Visitor<'de> for Visitor<'a> {
            type Value = FluidBoxes<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence or a map of fluid boxes")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut boxes = Vec::new();
                while let Some(value) = seq.next_element()? {
                    boxes.push(value);
                }
                Ok(FluidBoxes {
                    boxes,
                    off_when_no_fluid_recipe: false,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut result = FluidBoxes::default();
                while let Some(key) = map.next_key::<&'de str>()? {
                    if key == "off_when_no_fluid_recipe" {
                        result.off_when_no_fluid_recipe = map.next_value::<bool>()?;
                        continue;
                    }
                    if key.parse::<i32>().is_err() {
                        panic!("Unknown key {key:?}");
                    }
                    let value = map.next_value::<FluidBox<'a>>()?;
                    result.boxes.push(value);
                }
                Ok(result)
            }
        }

        deserializer.deserialize_any(Visitor {
            _marker: std::marker::PhantomData,
        })
    }
}
