use hdi::prelude::*;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def()]
    Task(Task),
}
#[hdk_entry_helper]
#[derive(Clone)]
pub struct Task(pub String);

