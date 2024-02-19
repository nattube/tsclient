use core::hash;
use std::{any::TypeId, collections::hash_map::DefaultHasher, hash::{Hash as _, Hasher as _}};

use crate::types::{builder::{GlobalTypeRegistry, HasIndexed}, TypescriptType, model::{ComponentReference, Component, Type, EnumRepresentation, InnerType}};

use super::{boilerplate_simple_definition, boilerplate_simple_hash, ts_simple, ts_array, ts_tuple};

impl<T: TypescriptType + 'static, E: TypescriptType + 'static> TypescriptType for Result<T, E> {
    fn get_definition(registry: &mut GlobalTypeRegistry) -> HasIndexed {
        let type_id = TypeId::of::<Self>();
        if let Some(existing) = registry.return_existing(type_id) {
            return existing
        }

        registry.start(type_id);

        let ok = ComponentReference {
            id: T::get_definition(registry),
            renamed: None,
        };
        
        let err = ComponentReference {
            id: E::get_definition(registry),
            renamed: None,
        };

        let hash = Self::hash(registry); 

        let component = Component {
            name: format!("Result"),
            typ: Type::Enum(
                EnumRepresentation::Default, 
                vec![(String::from("Ok"), InnerType::NewType(ok)), (String::from("Err"), InnerType::NewType(err))]
            ),
            hash
        };

        return registry.finalize(type_id, component)
    }

    fn hash(registry: &mut GlobalTypeRegistry) -> u64 {
        let type_id = ::std::any::TypeId::of::<Self>();

        if let Some(h) = registry.start_hash(type_id) {
            return h
        }

        let mut hasher = DefaultHasher::new();
        "enum".hash(&mut hasher);
        Self::name().hash(&mut hasher);
        T::name().hash(&mut hasher);
        T::hash(registry).hash(&mut hasher);
        E::name().hash(&mut hasher);
        E::hash(registry).hash(&mut hasher);

        let hash = hasher.finish();

        registry.finalize_hash(type_id, hash);

        return hash;
    }

    fn name() -> String {
        format!("Option<{},{}>", T::name(), E::name())
    }

    fn ts_name() -> String {
        String::from("Result")
    }
}

impl<T: TypescriptType + 'static> TypescriptType for Option<T> {
    fn get_definition(registry: &mut GlobalTypeRegistry) -> HasIndexed {
        let type_id = TypeId::of::<Self>();
        if let Some(existing) = registry.return_existing(type_id) {
            return existing
        }

        registry.start(type_id);

        let some = ComponentReference {
            id: T::get_definition(registry),
            renamed: None,
        };

        let hash = Self::hash(registry); 

        let component = Component {
            name: format!("Option"),
            typ: Type::Enum(
                EnumRepresentation::Untagged, 
                vec![(String::from("Ok"), InnerType::NewType(some)), (String::from("None"), InnerType::Null)]
            ),
            hash
        };

        return registry.finalize(type_id, component)
    }

    fn hash(registry: &mut GlobalTypeRegistry) -> u64 {
        let type_id = ::std::any::TypeId::of::<Self>();

        if let Some(h) = registry.start_hash(type_id) {
            return h
        }

        let mut hasher = DefaultHasher::new();
        "enum".hash(&mut hasher);
        Self::name().hash(&mut hasher);
        T::name().hash(&mut hasher);
        T::hash(registry).hash(&mut hasher);

        let hash = hasher.finish();

        registry.finalize_hash(type_id, hash);

        return hash;
    }

    fn name() -> String {
       format!("Option")
    }

    fn ts_name() -> String {
        T::name() + " | null"
    }
}

ts_simple!(String, "String", "string");
ts_simple!(&str, "&str", "string");

ts_simple!((), "()", "null");

ts_simple!(bool, "bool", "boolean");

ts_simple!(u8, "u8", "number");
ts_simple!(u16, "u16", "number");
ts_simple!(u32, "u32", "number");
ts_simple!(u64, "u64", "number");

ts_simple!(i8, "i8", "number");
ts_simple!(i16, "i16", "number");
ts_simple!(i32, "i32", "number");
ts_simple!(i64, "i64", "number");

ts_simple!(f32, "f32", "number");
ts_simple!(f64, "f64", "number");

ts_array!(Vec<T>, "Vec", "");
ts_array!([T; C], "[", "]", C);
ts_array!(&[T], "[", "]");

ts_tuple!(T1, T2);
ts_tuple!(T1, T2, T3);
ts_tuple!(T1, T2, T3, T4);
ts_tuple!(T1, T2, T3, T4, T5);
ts_tuple!(T1, T2, T3, T4, T5, T6);
ts_tuple!(T1, T2, T3, T4, T5, T6, T7);