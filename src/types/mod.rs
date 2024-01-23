use std::{any::TypeId, marker::PhantomData};

use self::{model::{Component, AnyType}, builder::{GlobalTypeRegistry, HasIndexed}};

pub mod model;
pub mod builder;
pub mod impls;

pub trait GetDefinition<T> {
    fn get_definition(self, registry: &mut GlobalTypeRegistry) -> HasIndexed;
    fn hash(self, registry: &mut GlobalTypeRegistry) -> u64;
    fn name(self) -> String;
    fn ts_name(self) -> String;
}

pub trait TypescriptType {
    fn get_definition(registry: &mut GlobalTypeRegistry) -> HasIndexed;
    fn hash(registry: &mut GlobalTypeRegistry) -> u64;
    fn name() -> String;
    fn ts_name() -> String;
}

impl<T> GetDefinition<T> for &TypeHolder<T> {
    fn get_definition(self, registry: &mut GlobalTypeRegistry) -> HasIndexed {
        let id = TypeId::of::<AnyType>();
        let result = if !registry.has(id) {
            registry.start(id);
            registry.finalize(id, Component::any())
        } else {
            registry.finalize(id, Component::any())
        };

        return result
    }
    fn name(self) -> String {
        String::from("any")
    }
    fn ts_name(self) -> String {
        String::from("any")
    }
    fn hash(self, registry: &mut GlobalTypeRegistry) -> u64 {
        Component::any().hash
    }
}

impl<T: TypescriptType> GetDefinition<T> for &TypeHolderStrict<T> {
    fn get_definition(self, registry: &mut GlobalTypeRegistry) -> HasIndexed {
        T::get_definition(registry)
    }
    fn name(self) -> String {
        T::name()
    }
    fn hash(self, registry: &mut GlobalTypeRegistry) -> u64 {
        T::hash(registry)
    }
    fn ts_name(self) -> String {
        T::ts_name()
    }
}
impl<T: TypescriptType> GetDefinition<T> for &mut &TypeHolder<T> {
    fn get_definition(self, registry: &mut GlobalTypeRegistry) -> HasIndexed {
        T::get_definition(registry)
    }
    fn name(self) -> String {
        T::name()
    }
    fn hash(self, registry: &mut GlobalTypeRegistry) -> u64 {
        T::hash(registry)
    }
    fn ts_name(self) -> String {
        T::ts_name()
    }
}


pub struct TypeHolder<T> {
    _phantom: PhantomData<T>
}

pub struct TypeHolderStrict<T> {
    _phantom: PhantomData<T>
}

impl<T> TypeHolder<T> {
    pub fn new() -> Self {
        TypeHolder { _phantom: PhantomData }
    }
}
impl<T> TypeHolderStrict<T> {
    pub fn new() -> Self {
        TypeHolderStrict { _phantom: PhantomData }
    }
}