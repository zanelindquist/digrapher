use std::cmp::min;
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
const SCALING_CONSTANT: f32 = 4.0;
#[derive(Clone, Copy, PartialEq)]
pub struct CanvasPositioning {
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: i32,
    pub height: i32,
    pub zoom: f32,
    pub dom_element_offset_x: f32,
    pub dom_element_offset_y: f32,
}
impl CanvasPositioning {
    pub fn new() -> Self {
        Self {
            offset_x: 0,
            offset_y: 0,
            width: 300,
            height: 300,
            zoom: 1.0,
            dom_element_offset_x: 0.0,
            dom_element_offset_y: 0.0
        }
    }
    pub fn create(offset_x: i32, offset_y: i32, width: i32, height: i32, zoom: f32, dom_element_offset_x: f32, dom_element_offset_y: f32) -> Self {
        Self {
            offset_x,
            offset_y,
            width,
            height,
            zoom,
            dom_element_offset_x,
            dom_element_offset_y
        }
    }
    pub fn from(self, other: &CanvasPositioning) -> CanvasPositioning {
        CanvasPositioning::create(self.offset_x, self.offset_y, self.width, self.height, self.zoom, self.dom_element_offset_x, self.dom_element_offset_y)
    }
    // Takes in logial x, y and returns visual x, y
    pub fn logical_to_visual_xy(self, lx: f32, ly: f32) -> (f32, f32) {
        // Involves shifting and scaling
        // Logical points are in a -1 to 1 x, y plane
        let center_vx = (self.width as f32) / 2.0 + self.offset_x as f32;
        let center_vy = (self.height as f32) / 2.0 + self.offset_y as f32;
        let v_over_l = self.zoom * (min(self.width, self.height) as f32) / SCALING_CONSTANT;

        (center_vx + lx * v_over_l, center_vy + ly * v_over_l)
    }
    pub fn logical_to_visual_scalar(self, l: f32) -> f32 {
        self.zoom * (min(self.width, self.height) as f32) / SCALING_CONSTANT
    }

    pub fn visual_to_logical_xy(self, vx: f32, vy: f32) -> (f32, f32) {
        // Logical points are in a -1 to 1 x, y plane
        let center_vx = (self.width as f32) / 2.0 + self.offset_x as f32;
        let center_vy = (self.height as f32) / 2.0 + self.offset_y as f32;
        let l_over_v = SCALING_CONSTANT / ((min(self.width, self.height) as f32) * self.zoom);

        ((vx - center_vx) * l_over_v, (vy - center_vy) * l_over_v)
    }
    pub fn visual_to_logical_scalar(self, l: f32) -> f32 {
        l / ((min(self.width, self.height) as f32) / SCALING_CONSTANT * self.zoom)
    }

    pub fn pointer_to_logical_xy(self, vx: f32, vy: f32) -> (f32, f32) {
        // Logical points are in a -1 to 1 x, y plane
        let center_vx = (self.width as f32) / 2.0 + self.offset_x as f32;
        let center_vy = (self.height as f32) / 2.0 + self.offset_y as f32;
        let l_over_v = SCALING_CONSTANT / ((min(self.width, self.height) as f32) * self.zoom);

        ((vx - self.dom_element_offset_x - center_vx) * l_over_v, (vy - self.dom_element_offset_y - center_vy) * l_over_v)
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

