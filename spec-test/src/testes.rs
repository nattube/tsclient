use serde::{Serialize, Deserialize};
use tsclient::{TypeScript, prelude::*};
use serde_json::Value as JsonValue;


#[derive(Serialize, Deserialize, Clone, TypeScript)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
    pub refresh_pw: bool,
    pub groups: Vec<Group>,
    pub security_version: i32
}

#[derive(Serialize, Deserialize, Clone, TypeScript)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub roles: Vec<Role>,
    pub meta_data: Option<JsonValue>,
}

#[derive(Serialize, Deserialize, Clone, TypeScript)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub group_id: i32,
    pub abilities: Vec<Ability>,
    pub meta_data: Option<JsonValue>,
}

#[derive(Serialize, Deserialize, Clone, TypeScript)]
pub struct Ability {
    pub id: i32,
    pub name: Abilities,
}

pub type LocationId = i32;
pub type EventTypeId = i32;
pub type GroupId = i32;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, TypeScript)]
#[serde(tag = "type", content = "value")]
pub enum Abilities {
    ManageCalendar,
    ManageUser,
    ManageGroups,

    ManageLocation(LocationId),
    ManageGroup(GroupId),
    
    ViewCalendar(LocationId),
    CreateCalendarEvent(LocationId),
    ChangeOtherCalendarEvents(LocationId),

    ViewCalendarEventType(EventTypeId),
    CreateCalendarEventType(EventTypeId),

    ViewGroupEvent(GroupId),
    CreateGroupEvent(GroupId),
    ChangeOtherGroupEvents(GroupId),
    
    None
}