use std::marker::PhantomData;

#[derive(Default)]
pub struct IdGenerator<'a, T> {
    names: Vec<&'a str>,
    _phantom: PhantomData<T>,
}

impl<'a, T> IdGenerator<'a, T>
where
    T: From<u32>,
{
    pub fn add(&mut self, name: &'a str) {
        match self.names.binary_search(&name) {
            Ok(_) => {
                panic!("Name already exists: {}", name);
            }
            Err(pos) => self.names.insert(pos, name),
        }
    }

    pub fn freeze(self) -> FrozenIdGenerator<'a, T> {
        FrozenIdGenerator {
            names: self.names,
            _phantom: PhantomData,
        }
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
