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

macro_rules! ts_tuple {
    ($($typ:ident),+) => { 
        impl<$($typ : TypescriptType + 'static),+> TypescriptType for ($($typ),+) {
            fn get_definition(registry: &mut GlobalTypeRegistry) -> HasIndexed {
                let type_id = TypeId::of::<Self>();
                if let Some(existing) = registry.return_existing(type_id) {
                    return existing
                }
            
                registry.start(type_id);
            
                let mut subs = Vec::new();

                $(
                    subs.push(ComponentReference {
                        id: $typ::get_definition(registry),
                        renamed: None,
                    });
                )+
            
                let hash = Self::hash(registry); 
            
                let component = Component {
                    name: format!("Tuple"),
                    typ: Type::Struct(
                        InnerType::Tuple(subs)
                    ),
                    hash
                };
            
                return registry.finalize(type_id, component)
            }
            fn name() -> String {
                let mut names = Vec::new();
                $(
                    names.push($typ::name());
                )+

                format!("({})", names.join(", "))
            }
            fn ts_name() -> String {
                let mut names = Vec::new();
                $(
                    names.push($typ::ts_name());
                )+
                format!("[{}]", names.join(", "))
            }
            fn hash(registry: &mut GlobalTypeRegistry) -> u64 {
                let type_id = ::std::any::TypeId::of::<Self>();

                if let Some(h) = registry.start_hash(type_id) {
                    return h
                }
            
                let mut hasher = DefaultHasher::new();
                "tuple".hash(&mut hasher);
                Self::name().hash(&mut hasher);

                $(
                    $typ::hash(registry).hash(&mut hasher);
                )+
            
                let hash = hasher.finish();
            
                registry.finalize_hash(type_id, hash);
            
                return hash;
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