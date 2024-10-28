#[derive(Debug)]
pub struct VecOrMap<T>(pub Vec<T>);

impl<T> std::ops::Deref for VecOrMap<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de, T> serde::de::Deserialize<'de> for VecOrMap<T>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<VecOrMap<T>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<T> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: serde::de::Deserialize<'de>,
        {
            type Value = VecOrMap<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence or an empty map")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(value) = seq.next_element()? {
                    vec.push(value);
                }
                Ok(VecOrMap(vec))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let vec = Vec::new();
                if map.next_key::<&'de str>()?.is_some() {
                    return Err(serde::de::Error::custom("expected an empty map"));
                }
                Ok(VecOrMap(vec))
            }
        }

        deserializer.deserialize_any(Visitor {
            _marker: std::marker::PhantomData,
        })
    }
}
