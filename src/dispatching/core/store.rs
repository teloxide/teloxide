use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
use std::sync::Arc;

pub struct Store {
    map: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Store {
    pub fn new() -> Self {
        Store { map: HashMap::new() }
    }

    pub fn insert<T>(&mut self, data: T)
    where
        T: Send + Sync + 'static,
    {
        self.map.insert(TypeId::of::<T>(), Arc::new(data));
    }

    pub fn get<T>(&self) -> Option<Arc<T>>
    where
        T: Send + Sync + 'static,
    {
        let item = self.map.get(&TypeId::of::<T>());
        item.map(|b| {
            b
                .clone()
                .downcast()
                .expect("We add items by TypeId, so if we get the item it must be expected type")
        })
    }
}
