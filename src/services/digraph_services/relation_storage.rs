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

use gloo_storage::{LocalStorage, Storage, errors::StorageError};
use std::{fmt};

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
    if let Ok(mut relations) = get_stored_relations() {
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
        if let Ok(_) = LocalStorage::set("stored_relations", &relations){
            return Ok(relations);
        };
    }

    Err(RelationStorageErr::StorageWrite)
}

pub fn remove_relation(id: i32) -> Result<Vec<StoredRelation>, RelationStorageErr> {
    let mut relations = get_stored_relations()?;

    if let Ok(_) = clear_all_relations() {
        relations = relations.iter().filter(|rel| rel.id != id).map(|v: &StoredRelation| (*v).clone()).collect::<Vec<StoredRelation>>();

        if let Ok(_) = LocalStorage::set("stored_relations", &relations) {
            return Ok(relations)
        }
    }
    Err(RelationStorageErr::StorageWrite)

}

pub fn clear_all_relations() -> Result<Vec<StoredRelations>, RelationStorageErr> {
    match LocalStorage::set("stored_relations", "[]") {
        Ok(_) => Ok(vec![]),
        Err(_) => {
            Err(RelationStorageErr::StorageWrite)
        }
    }
}

// Returns the string from localstorage for the recipient to process
pub fn get_stored_relations() -> Result<Vec<StoredRelation>, RelationStorageErr> {
    match LocalStorage::get::<Vec<StoredRelation>>("stored_relations") {
        Ok(relations) => Ok(relations),
        // If the key is not found, then we need to set the storage as []
        Err(StorageError::KeyNotFound(_)) => {
            if let Ok(_) = clear_all_relations() {
                Ok(vec![])
            } else {
                Err(RelationStorageErr::StorageWrite)
            }
        }
        Err(_) => {
            Err(RelationStorageErr::StorageRead)
        }
    }
}

const CURATED_RELATIONS: &'static str = include_str!("../../assets/digraph_assets/curated_relations.json");

pub fn get_stored_relations_from_json() -> Result<Vec<StoredRelation>, RelationStorageErr> {
    match serde_json::from_str::<Vec<StoredRelation>>(CURATED_RELATIONS) {
        Ok(relations) => Ok(relations),
        Err(e) => {
            web_sys::console::log_1(
                &format!("Storage read error: {:?}", e).into()
            );
            Err(RelationStorageErr::StorageRead)
        }
    }
}