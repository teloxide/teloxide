use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Store {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl Store {
    pub fn new() -> Self {
        Store { map: HashMap::new() }
    }

    pub fn insert<T>(&mut self, data: T)
    where
        T: 'static,
    {
        self.map.insert(TypeId::of::<T>(), Box::new(data));
    }

    pub fn get<T>(&self) -> Option<&T>
    where
        T: 'static,
    {
        let item = self.map.get(&TypeId::of::<T>());
        item.map(|b| {
            b.as_ref()
                .downcast_ref()
                .expect("We add items by TypeId, so if we get the item it must be expected type")
        })
    }
}
