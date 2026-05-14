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
use std::fmt;

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

pub fn store_new_relation(new_relation: Relation) -> Result<StoredRelations, RelationStorageErr> {
    let relations = get_stored_relations();

    match &relations {
        Ok(relations) => {
            // Build new relation
            let stored_new_relation = StoredRelation {
                name: String::from(""),
                id: 0,
                // Formats like {(a, b), (c, d)}
                raw_text: format!("{{{}}}", new_relation.values.iter().map(|(a, b)| format!("({}, {})", a, b)).collect::<Vec<_>>().join(", ")),
                date_saved: time_format::strftime_local("%m-%d-%Y", time_format::now().unwrap()).unwrap()
            };
            let mut new_vec = relations.clone();
            // Add new relation
            new_vec.push(stored_new_relation);

            match LocalStorage::set("stored_relations", serde_json::to_string(&new_vec).unwrap_or_default()) {
                Ok(_) => Ok(relations.to_vec()),
                Err(_) => Err(RelationStorageErr::StorageWrite)
            }
            
        },
        Err(e) => {
            Err(e.clone())
        }
    }
}

// Returns the string from localstorage for the recipient to process
pub fn get_stored_relations() -> Result<StoredRelations, RelationStorageErr> {
    match LocalStorage::get::<String>("stored_relations") {
        Ok(raw_text) => {
            match serde_json::from_str::<Vec<StoredRelation>>(raw_text.as_str()) {
                Ok(relations) => Ok(relations),
                Err(_) => Err(RelationStorageErr::JsonParse)
            }
        },
        // If it can't read it, then we want to set stored_relations to []
        Err(_) => {
            match LocalStorage::set("stored_relations", "[]") {
                Ok(_) => get_stored_relations(),
                Err(_) => Err(RelationStorageErr::StorageWrite)
            }
        }
    }
}