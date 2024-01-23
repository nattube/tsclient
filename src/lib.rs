use std::any::TypeId;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

pub mod types;
pub mod axum;
pub mod prelude;
pub mod api;

pub(crate) mod errors;
pub(crate) mod api_router;
pub(crate) mod utils;
pub(crate) mod routing;

use types::TypescriptType;
use types::builder::GlobalTypeRegistry;
use types::builder::HasIndexed;
use types::model::Component;
use types::model::Type;
pub use typescript::TypeScript;
pub use typescript::TypeScriptStrict;

static GLOBAL_TYPE_REGISTRY: Lazy<Mutex<GlobalTypeRegistry>> = Lazy::new(|| {
    Mutex::new(GlobalTypeRegistry::new())
});

pub(crate) const FILE_HEADER: &'static str = 
r#"/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/
"#;

pub struct AnyDef;

pub fn reset() {
    let mut registry = GLOBAL_TYPE_REGISTRY.lock().unwrap();
    registry.reset();
}


struct T1 {}

impl TypescriptType for T1 {
    fn get_definition(registry: &mut GlobalTypeRegistry) -> HasIndexed {
        let type_id = TypeId::of::<T1>();
        if registry.has(type_id) {
            return registry.finalize(type_id, Component::any())
        }

        let mut __hasher = DefaultHasher::new();
        "Object".hash(&mut __hasher);
        let hash = __hasher.finish();

        registry.finalize(type_id, Component {
            name: String::from("T1"),
            typ: Type::SimpleType(String::from("T1")),
            hash,
        })
    }
    fn name() -> String {
        String::from("T1")
    }
    fn ts_name() -> String {
        String::from("T1")
    }
    fn hash(registry: &mut GlobalTypeRegistry) -> u64 {
        let type_id = TypeId::of::<T1>();
        if let Some(h) = registry.start_hash(type_id) {
            return h;
        }

        let mut __hasher = DefaultHasher::new();
        "Object".hash(&mut __hasher);
        let hash = __hasher.finish();

        registry.finalize_hash(type_id, hash);

        return hash;
    }
}


#[cfg(test)]
mod tests {
    use std::{collections::HashMap, hash::{DefaultHasher, Hasher as _}};

    use ::axum::{extract::Query, Json};

    use crate::{api_router::ApiRouter, types::{TypeHolder, GetDefinition, builder::GlobalTypeRegistry}};
    use super::*;

    use crate as tsclient;

    #[test]
    fn test1() {
        let t1 = DefaultHasher::new().finish();
        let t2 = DefaultHasher::new().finish();
        let t3 = DefaultHasher::new().finish();

        println!("{} - {} - {}", t1, t2, t3);
    }

    #[test]
    fn test() {
        //let mut builder = Builder { components: HashMap::new() };
        //(&mut &String::from("value")).build_ts(&mut builder);
        //(&mut &T1 {}).build_ts(&mut builder);

        let mut builder = GlobalTypeRegistry::new();

        let d1 = (&mut &TypeHolder::<String>::new()).get_definition(&mut builder);
        let d2 = (&mut &TypeHolder::<T1>::new()).get_definition(&mut builder);

        println!("{:?}", d1);
        println!("{:?}", d2);

        //tes(&mut &String::from("value"));
        //tes(&mut &T1 {});
    }
}
