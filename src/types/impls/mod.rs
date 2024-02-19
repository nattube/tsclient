use ::std::{any::TypeId, collections::hash_map::DefaultHasher, hash::{ Hash as _, Hasher as _}};

use super::{TypescriptType, builder::{GlobalTypeRegistry, HasIndexed}, model::{Component, Type}};

pub mod std;
pub mod chrono;
#[cfg(not(target_family = "wasm"))]
pub mod router;
pub mod json;

#[macro_use]
pub mod macros;

pub(crate) use ts_simple;
pub(crate) use ts_array;
pub(crate) use ts_array_base;
pub(crate) use ts_tuple;

pub fn boilerplate_simple_definition<T: TypescriptType + 'static>(typ: Type, registry: &mut GlobalTypeRegistry) -> HasIndexed {
    let type_id = TypeId::of::<T>();
    if registry.has(type_id) {
        return registry.finalize(type_id, Component::any())
    }
    
    let hash = T::hash(registry);
    let name = T::name();

    registry.finalize(type_id, Component {
        name,
        typ,
        hash,
    })
}

pub fn boilerplate_simple_hash<T: TypescriptType + 'static>(registry: &mut GlobalTypeRegistry) -> u64 {
    let type_id = TypeId::of::<T>();
    if let Some(h) = registry.start_hash(type_id) {
        return h;
    }

    let mut __hasher = DefaultHasher::new();
    T::name().hash(&mut __hasher);
    let hash = __hasher.finish();

    registry.finalize_hash(type_id, hash);

    return hash;
}