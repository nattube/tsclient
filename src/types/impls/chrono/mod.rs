use chrono::{Utc, DateTime, Local, NaiveDate};

use crate::types::{builder::{GlobalTypeRegistry, HasIndexed}, TypescriptType, model::{ComponentReference, Component, Type, EnumRepresentation, InnerType}};

use super::{boilerplate_simple_definition, boilerplate_simple_hash, ts_simple, ts_array};


ts_simple!(DateTime<Utc>, "DateTime<Utc>", "string");
ts_simple!(DateTime<Local>, "DateTime<Local>", "string");
ts_simple!(NaiveDate, "NaiveDate", "string");