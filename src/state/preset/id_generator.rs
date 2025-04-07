use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use rustc_hash::FxHashSet;

#[derive(Default)]
pub struct IdGenerator<T> {
    names: FxHashSet<String>,
    _phantom: PhantomData<T>,
}

impl<T> IdGenerator<T>
where
    T: From<u64>,
{
    pub fn from_iter(i: impl Iterator<Item = String>) -> FrozenIdGenerator<T> {
        Self {
            names: i.collect(),
            _phantom: PhantomData,
        }
        .freeze()
    }
    pub fn add(&mut self, name: String) {
        self.names.insert(name);
    }

    pub fn freeze(self) -> FrozenIdGenerator<T> {
        let mut generator = FrozenIdGenerator {
            names: self
                .names
                .into_iter()
                .map(|s| {
                    let mut hasher = std::hash::DefaultHasher::new();
                    s.hash(&mut hasher);
                    (s, hasher.finish())
                })
                .collect(),
            _phantom: PhantomData,
        };
        generator.names.sort_by_key(|(name, _)| name.clone());
        generator
    }
}

pub struct FrozenIdGenerator<T> {
    names: Vec<(String, u64)>,
    _phantom: PhantomData<T>,
}

impl<T> FrozenIdGenerator<T>
where
    T: From<u64>,
{
    pub fn get(&self, name: &String) -> T {
        let Ok(index) = self.names.binary_search_by(|(n, _)| n.cmp(name)) else {
            println!("{:?}", self.names);
            panic!("Name was not added: {}", name);
        };
        T::from(self.names[index].1)
    }
}
