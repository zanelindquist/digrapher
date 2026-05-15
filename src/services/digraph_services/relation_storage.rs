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

// Store a new relation
pub fn store_new_relation(new_relation: &Relation) -> Result<StoredRelations, RelationStorageErr> {
    // Get all current relations
    match get_stored_relations() {
        // If we successfully fetch all relations
        Ok(mut relations) => {
            // Create a new stored relation object
            let stored_new_relation = StoredRelation {
                // No name for now, maybe enable renaming later
                name: String::new(),
                // Set its id
                id: relations.len() as i32,
                // Parse the relation into the form of {(a, b), (c, d)}...
                raw_text: format!(
                    "{{{}}}",
                    new_relation.values
                        .iter()
                        .map(|(a, b)| format!("({}, {})", a, b))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                // No date for now
                date_saved: String::new()
            };
            // Add this relation to the stored relations we retrieved
            relations.push(stored_new_relation);
            // Set the local storage as the updated relations
            match LocalStorage::set("stored_relations", &relations) {
                Ok(_) => {
                    Ok(relations)
                },
                Err(_) => {
                    Err(RelationStorageErr::StorageWrite)
                }
            }
        },
        // Handle errors
        Err(RelationStorageErr::StorageRead) => {
            Err(RelationStorageErr::StorageRead)
        }
        _ => {
            Err(RelationStorageErr::StorageWrite)

        }
    }

}

// Remove a relation
pub fn remove_relation(id: i32) -> Result<Vec<StoredRelation>, RelationStorageErr> {
    // Get all of the relations
    match get_stored_relations() {
        Ok(mut relations) => {
            // Filter out the removed relation
            relations = relations.iter().filter(|rel| rel.id != id).map(|v: &StoredRelation| (*v).clone()).collect::<Vec<StoredRelation>>();

            // Set the storage
            if let Ok(_) = LocalStorage::set("stored_relations", &relations) {
                return Ok(relations)
            } else {
                Err(RelationStorageErr::StorageWrite)
            }            
        },
        // Handle storage fetching errors
        _ => {
            Err(RelationStorageErr::StorageRead)
        }
    }

}

// Set the relation storage as an empty vector
pub fn clear_all_relations() -> Result<Vec<StoredRelations>, RelationStorageErr> {
    match LocalStorage::set("stored_relations", &Vec::<StoredRelation>::new()) {
        // If successful, return an empty vector
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
            // clear_all_relations sets the storage as an empty vector, even though it might look counter-intuitive here
            if let Ok(_) = clear_all_relations() {
                Ok(vec![])
            } else {
                Err(RelationStorageErr::StorageWrite)
            }
        }
        // Pass reading errors
        Err(e) => {
            Err(RelationStorageErr::StorageRead)
        }
    }
}

// Load the curated relations from a json file into the stack
const CURATED_RELATIONS: &'static str = include_str!("../../assets/digraph_assets/curated_relations.json");
// Get stored relations by parsing a json string
pub fn get_stored_relations_from_json() -> Result<Vec<StoredRelation>, RelationStorageErr> {
    // Use serde to parse the json string into our StoredRelation object
    match serde_json::from_str::<Vec<StoredRelation>>(CURATED_RELATIONS) {
        // Return the relations
        Ok(relations) => Ok(relations),
        // On failure reutrn a JsonParse error
        Err(e) => {
            web_sys::console::log_1(
                &format!("Storage JSON error: {:?}", e).into()
            );
            Err(RelationStorageErr::JsonParse)
        }
    }
}