use rustc_hash::FxHashSet;

#[derive(Debug)]
pub struct Prerequisites<'a> {
    pub prerequisites: FxHashSet<&'a str>,
}

impl<'de: 'a, 'a> serde::de::Deserialize<'de> for Prerequisites<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Prerequisites<'a>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a> {
            _marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> serde::de::Visitor<'de> for Visitor<'a> {
            type Value = Prerequisites<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "an empty struct, a struct with prerequisites, or an array of prerequisites",
                )
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut prerequisites = FxHashSet::default();
                while let Some(value) = seq.next_element()? {
                    prerequisites.insert(value);
                }
                Ok(Prerequisites { prerequisites })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut prerequisites = FxHashSet::default();
                while let Some((key, value)) = map.next_entry()? {
                    let _key: &'a str = key;
                    prerequisites.insert(value);
                }
                Ok(Prerequisites { prerequisites })
            }
        }

        deserializer.deserialize_any(Visitor {
            _marker: std::marker::PhantomData,
        })
    }
}
