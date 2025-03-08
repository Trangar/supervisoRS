use itertools::Itertools;
use rustc_hash::FxHashMap;
use serde::{ser::SerializeSeq, Deserialize};

pub fn serialize<S, K, V>(map: &FxHashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    K: serde::Serialize + Ord + Copy,
    V: serde::Serialize,
{
    let mut seq = serializer.serialize_seq(Some(map.len()))?;
    for (_, value) in map.iter().sorted_by_key(|(key, _)| *key) {
        seq.serialize_element(value)?;
    }
    seq.end()
}

pub fn deserialize<'de, S, K, V>(deserializer: S) -> Result<FxHashMap<K, V>, S::Error>
where
    S: serde::de::Deserializer<'de>,
    K: serde::de::Deserialize<'de> + std::hash::Hash + Eq,
    V: serde::de::Deserialize<'de> + Keyable<K>,
{
    let values = Vec::<V>::deserialize(deserializer)?;
    let mut map = FxHashMap::default();
    for value in values {
        let key = value.key();
        map.insert(key, value);
    }
    Ok(map)
}

pub trait Keyable<K> {
    fn key(&self) -> K;
}
