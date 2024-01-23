macro_rules! ts_simple {
    ($typ:ty,$typ_name:literal,$ts_typ:literal) => { 
        impl TypescriptType for $typ {
            fn get_definition(registry: &mut GlobalTypeRegistry) -> HasIndexed {
                boilerplate_simple_definition::<$typ>(crate::types::model::Type::SimpleType(String::from($ts_typ)), registry)
            }
            fn name() -> String {
                String::from($typ_name)
            }
            fn ts_name() -> String {
                String::from($ts_typ)
            }
            fn hash(registry: &mut GlobalTypeRegistry) -> u64 {
                boilerplate_simple_hash::<$typ>(registry)
            }
        }
    }
}

macro_rules! ts_array_base {
    ($typ:ty, $ts_typ:expr, $typ_name:expr) => {
        fn get_definition(registry: &mut GlobalTypeRegistry) -> HasIndexed {
            let inner = T::get_definition(registry);

            let comp = crate::types::model::ComponentReference {
                id: inner,
                renamed: None
            };
        
            boilerplate_simple_definition::<$typ>(crate::types::model::Type::Array(comp), registry)
        }
        fn name() -> String {
            $typ_name
        }
        fn ts_name() -> String {
            String::from($ts_typ)
        }
        fn hash(registry: &mut GlobalTypeRegistry) -> u64 {
            boilerplate_simple_hash::<$typ>(registry)
        }
    }
}

macro_rules! ts_array {
    ($typ:ty, $typ_start:literal, $typ_end:literal, $const:ident) => {
        impl<T: TypescriptType + 'static, const $const: usize> TypescriptType for $typ {
            crate::types::impls::ts_array_base![$typ, format!("Array<{}>", T::name()), format!("{}{}; {}{}", $typ_start, T::name(), $const, $typ_end)];
        }
    };
    ($typ:ty, $typ_start:literal, $typ_end:literal) => { 
        impl<T: TypescriptType + 'static> TypescriptType for $typ {
            crate::types::impls::ts_array_base![$typ, format!("Array<{}>",T::name()), format!("{}{}{}", $typ_start, T::name(), $typ_end)];
        }
    }
}