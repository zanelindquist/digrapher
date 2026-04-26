use serde::{Deserialize, Serialize};
use yew::Properties;
use std::collections::HashSet;

use crate::render::objects::point::Point;

// TYPES

pub type RawCharPoints = HashSet<String>;
pub type PointVector = Vec<Point>;
// RELATIONS

#[derive(Properties, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RelationProperties {
    pub antisymmetric: bool,
    pub symmetric: bool,
    pub reflexive: bool,
    pub transitive: bool
}
impl Default for RelationProperties {
    fn default() -> Self {
        Self {
            antisymmetric: true,
            symmetric: true,
            reflexive: true,
            transitive: true,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Relation {
    pub values: Vec<(String, String)>,
    pub points: RawCharPoints,
    pub properties: RelationProperties
}


// ERRORS

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub message: String,
}