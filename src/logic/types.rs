use serde::{Deserialize, Serialize};
use yew::Properties;
use std::collections::HashSet;

use crate::render::objects::{edge::Edge, point::Point};

// TYPES
pub type RawCharPoints = HashSet<String>;
pub type SortedCharPoints = Vec<String>;
pub type RawEdgePairs = HashSet<(String, String)>;
pub type PointVector = Vec<Point>;
pub type EdgeVector = Vec<Edge>;

// ENUMS

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum RelationProperty {ANTISYMMETRIC, SYMMETRIC, REFLEXIVE, TRANSITIVE}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum PointRenderSymbol{CIRCLE, TRIANGLE}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum GraphModes{DIGRAPH, MATRIX}
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

#[derive(Properties, PartialEq, Clone)]
pub struct Relation {
    pub values: RawEdgePairs,
    pub points: RawCharPoints,
    pub properties: RelationProperties
}


// ERRORS
#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub message: String,
}
impl ParseError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

pub type DigestedValuesResult = Result<Relation, ParseError>;

