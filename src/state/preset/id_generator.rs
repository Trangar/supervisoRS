use std::marker::PhantomData;

use rustc_hash::FxHashSet;

#[derive(Default)]
pub struct IdGenerator<'a, T> {
    names: FxHashSet<&'a str>,
    _phantom: PhantomData<T>,
}

impl<'a, T> IdGenerator<'a, T>
where
    T: From<u32>,
{
    pub fn add(&mut self, name: &'a str) {
        self.names.insert(name);
    }

    pub fn freeze(self) -> FrozenIdGenerator<'a, T> {
        let mut generator = FrozenIdGenerator {
            names: self.names.into_iter().collect(),
            _phantom: PhantomData,
        };
        generator.names.sort();
        generator
    }
}

pub struct FrozenIdGenerator<'a, T> {
    names: Vec<&'a str>,
    _phantom: PhantomData<T>,
}

impl<'a, T> FrozenIdGenerator<'a, T>
where
    T: From<u32>,
{
    pub fn get(&self, name: &str) -> T {
        let Ok(index) = self.names.binary_search(&name) else {
            println!("{:?}", self.names);
            panic!("Name was not added: {}", name);
        };
        T::from(index as u32)
    }
}
