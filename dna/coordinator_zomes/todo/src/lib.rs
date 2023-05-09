use hdk::prelude::*;
use todo_integrity::{EntryTypes, Task};

#[hdk_extern]
pub fn create_item(
    item: String
) -> ExternResult<EntryHash> {
    let task = Task(item.clone());
    create_entry(EntryTypes::Task(task.clone()))?;
    let entry_hash = hash_entry(task)?;
    Ok(entry_hash)
}

#[hdk_extern]
pub fn get_item(
    entry_hash: EntryHash
) -> ExternResult<Option<Record>> {
    let maybe_element = get(entry_hash, GetOptions::default())?;
    Ok(maybe_element)
}