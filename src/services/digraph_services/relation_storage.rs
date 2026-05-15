/*
    Relations are to be stored in a JSON format like
    [
        {
            "name": "Name",
            "id": 1,
            "raw_text": "...",
            "date_saved": "5/14/2026"
        }
    ]

*/

use gloo_storage::{LocalStorage, Storage};
use std::{fmt, ops::Deref};
use gloo_console::log;
use web_sys::console;

use crate::services::digraph_services::types::{Relation, StoredRelation, StoredRelations};

#[derive(Debug, Clone)]
pub enum RelationStorageErr {
    StorageRead,
    StorageWrite,
    JsonParse,
}
impl fmt::Display for RelationStorageErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            RelationStorageErr::StorageRead => "Failed to read from storage",
            RelationStorageErr::StorageWrite => "Failed to write to storage",
            RelationStorageErr::JsonParse => "Failed to parse JSON",
        };

        write!(f, "{}", msg)
    }
}

pub fn store_new_relation(new_relation: &Relation) -> Result<StoredRelations, RelationStorageErr> {
    let mut relations = get_stored_relations()?;

    let stored_new_relation = StoredRelation {
        name: String::new(),
        id: relations.len() as i32,
        raw_text: format!(
            "{{{}}}",
            new_relation.values
                .iter()
                .map(|(a, b)| format!("({}, {})", a, b))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        date_saved: String::new()
    };

    relations.push(stored_new_relation);
    LocalStorage::set("stored_relations", &relations).map_err(|_| RelationStorageErr::StorageWrite)?;

    Ok(relations)
}

pub fn remove_relation(id: i32) -> Result<Vec<StoredRelation>, RelationStorageErr> {
    let mut relations = get_stored_relations()?;

    clear_all_relations();

    relations = relations.iter().filter(|rel| rel.id != id).map(|v: &StoredRelation| (*v).clone()).collect::<Vec<StoredRelation>>();

    LocalStorage::set("stored_relations", &relations).map_err(|_| RelationStorageErr::StorageWrite)?;

    Ok(relations)
}

pub fn clear_all_relations() {
    LocalStorage::set("stored_relations", "[]");
}

// Returns the string from localstorage for the recipient to process
pub fn get_stored_relations() -> Result<Vec<StoredRelation>, RelationStorageErr> {
    match LocalStorage::get::<Vec<StoredRelation>>("stored_relations") {
        Ok(relations) => Ok(relations),
        Err(e) => {
            web_sys::console::log_1(
                &format!("Storage read error: {:?}", e).into()
            );
            Err(RelationStorageErr::StorageRead)
        }
    }
}