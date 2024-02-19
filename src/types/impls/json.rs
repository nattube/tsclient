use std::any::TypeId;

use serde_json::Value;

use crate::types::{TypescriptType, builder::{GlobalTypeRegistry, HasIndexed}, model::{ComponentReference, Component, EnumRepresentation, Type, InnerType}};

impl TypescriptType for Value {
    fn get_definition(registry: &mut GlobalTypeRegistry) -> HasIndexed {
        let type_id = TypeId::of::<Self>();
        if let Some(existing) = registry.return_existing(type_id) {
            return existing
        }

        registry.start(type_id);

        let component = Component::any();

        return registry.finalize(type_id, component)
    }

    fn hash(_registry: &mut GlobalTypeRegistry) -> u64 {
        Component::any().hash
    }

    fn name() -> String {
       String::from("any")
    }

    fn ts_name() -> String {
        String::from("any")
    }
}