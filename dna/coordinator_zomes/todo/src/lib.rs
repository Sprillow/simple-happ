use hdk::prelude::{*, holo_hash::EntryHashB64};
use todo_integrity::{EntryTypes, Task, LinkTypes};

#[hdk_extern]
pub fn create_item(
    item: String
) -> ExternResult<EntryHash> {
    let task = Task(item.clone());
    create_entry(EntryTypes::Task(task.clone()))?;
    let entry_hash = hash_entry(task)?;
    create_link(
        all_tasks_typed_path()?.path_entry_hash()?,
        entry_hash.clone(),
        LinkTypes::AllTasks,
        (),
    )?;
    Ok(entry_hash)
}

#[hdk_extern]
pub fn get_item(
    entry_hash: EntryHashB64
) -> ExternResult<Option<Record>> {
    let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;
    Ok(maybe_element)
}

#[hdk_extern]
pub fn get_all_items(_: ()) -> ExternResult<Vec<EntryHash>> {
    let all_tasks_path = all_tasks_typed_path()?;
    let links = get_links(all_tasks_path.path_entry_hash()?, LinkTypes::AllTasks, None)?;
    Ok(links.into_iter().map(|link| link.target.try_into().unwrap()).collect())
}

pub fn all_tasks_typed_path() -> ExternResult<TypedPath> {
    Ok(Path::from(format!("{}", "all")).typed(LinkTypes::AllTasks)?)
}