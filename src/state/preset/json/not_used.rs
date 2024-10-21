#[derive(Debug, Default)]
pub struct NotUsed;

impl<'de> serde::de::Deserialize<'de> for NotUsed {
    fn deserialize<D>(deserializer: D) -> Result<NotUsed, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NotUsed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("nothing")
            }

            fn visit_string<E>(self, _: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_map<A>(self, mut map: A) -> Result<NotUsed, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                while (map.next_key::<&'de str>()?).is_some() {
                    map.next_value::<NotUsed>()?;
                }

                Ok(NotUsed)
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_i128<E>(self, _: i128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_u128<E>(self, _: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_char<E>(self, _: char) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_byte_buf<E>(self, _: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_any(Visitor)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NotUsed)
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_any(Visitor)?;
                Ok(NotUsed)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                while seq.next_element::<NotUsed>()?.is_some() {}
                Ok(NotUsed)
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::EnumAccess<'de>,
            {
                data.variant::<NotUsed>()?;
                Ok(NotUsed)
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}
