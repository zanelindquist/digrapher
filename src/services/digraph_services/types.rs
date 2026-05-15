use serde::{Deserialize, Serialize};
use yew::prelude::*;
use std::{collections::HashSet};

use crate::render::{objects::{edge::Edge, point::Point}};

// TYPES
pub type PointLabel = String;
pub type EdgePair = (String, String);
pub type RawCharPoints = HashSet<String>;
pub type SortedCharPoints = Vec<String>;
pub type RawEdgePairs = HashSet<EdgePair>;
pub type PointVector = Vec<Point>;
pub type EdgeVector = Vec<Edge>;
pub type MatrixData = Vec<Vec<bool>>;
pub type StoredRelations = Vec<StoredRelation>;

// ENUMS

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum RelationProperty {ANTISYMMETRIC, SYMMETRIC, REFLEXIVE, TRANSITIVE}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum PointRenderSymbol{CIRCLE, TRIANGLE}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum GraphModes{DIGRAPH, MATRIX}
pub enum RelationExplorerModes{EDGES, POINTS}

#[derive(PartialEq)]
pub enum DrawObjectSelection {
    Point(PointLabel),
    Edge(EdgePair),
}

// INFRASTRUCTURE

#[derive(Clone, Copy, PartialEq)]
pub struct CanvasPositioning {
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: i32,
    pub height: i32,
    pub zoom: f32,
}
impl CanvasPositioning {
    pub fn new() -> Self {
        Self {
            offset_x: 0,
            offset_y: 0,
            width: 300,
            height: 300,
            zoom: 1.0,
        }
    }
    pub fn create(offset_x: i32, offset_y: i32, width: i32, height: i32, zoom: f32) -> Self {
        Self {
            offset_x,
            offset_y,
            width,
            height,
            zoom,
        }
    }
    pub fn from(self, other: &CanvasPositioning) -> CanvasPositioning {
        CanvasPositioning::create(self.offset_x, self.offset_y, self.width, self.height, self.zoom)
    }
}

#[derive(PartialEq)]
pub struct ObjectSelection {
    pub selection: Option<DrawObjectSelection>
}
impl ObjectSelection {
    // Intake a raw tuple string pairing and set it as the selected object type
    pub fn from_edge(paring: EdgePair) -> Self {
        Self{
            selection: Option::from(DrawObjectSelection::Edge(paring))
        }
    }
    // Intake a raw string and set it as the selected object type
    pub fn from_point(point: PointLabel) -> Self {
        Self{
            selection: Option::from(DrawObjectSelection::Point(point))
        }
    }
    pub fn default() -> Self {
        Self {
            selection: None
        }
    }
}


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

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct StoredRelation {
    pub name: String,
    pub id: i32,
    pub raw_text: String,
    pub date_saved: String
}
impl Default for StoredRelation {
    fn default() -> Self {
        Self {
            name: String::new(),
            // To indicate if an object exists or not lol idk
            id: -1,
            raw_text: String::new(),
            date_saved: String::new()
        }
    }
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

