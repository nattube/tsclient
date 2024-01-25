/** 
 * This File was generated automagically 🧙‍♂️ 
 * 
 * WARNING: Changes you perform here will probably not persist!
*/

type LocationId = number;
type GroupId = number;
type LocationId = number;
type LocationId = number;
type LocationId = number;
type EventTypeId = number;
type EventTypeId = number;
type GroupId = number;
type GroupId = number;
type GroupId = number;

export type ManageCalendar = "ManageCalendar"


export type ManageUser = "ManageUser"


export type ManageGroups = "ManageGroups"


export type ManageLocation = {
	type: ManageLocation;
	value: LocationId
}


export type ManageGroup = {
	type: ManageGroup;
	value: GroupId
}


export type ViewCalendar = {
	type: ViewCalendar;
	value: LocationId
}


export type CreateCalendarEvent = {
	type: CreateCalendarEvent;
	value: LocationId
}


export type ChangeOtherCalendarEvents = {
	type: ChangeOtherCalendarEvents;
	value: LocationId
}


export type ViewCalendarEventType = {
	type: ViewCalendarEventType;
	value: EventTypeId
}


export type CreateCalendarEventType = {
	type: CreateCalendarEventType;
	value: EventTypeId
}


export type ViewGroupEvent = {
	type: ViewGroupEvent;
	value: GroupId
}


export type CreateGroupEvent = {
	type: CreateGroupEvent;
	value: GroupId
}


export type ChangeOtherGroupEvents = {
	type: ChangeOtherGroupEvents;
	value: GroupId
}


export type None = "None"


export type Abilities = ManageCalendar | ManageUser | ManageGroups | ManageLocation | ManageGroup | ViewCalendar | CreateCalendarEvent | ChangeOtherCalendarEvents | ViewCalendarEventType | CreateCalendarEventType | ViewGroupEvent | CreateGroupEvent | ChangeOtherGroupEvents | None