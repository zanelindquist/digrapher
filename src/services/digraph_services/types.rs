use std::cmp::min;
use std::fmt;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use std::{collections::HashSet};

use crate::{render::objects::{edge::Edge, point::Point}, services::digraph_services::classify_relation::GraphTheoryRelationManager};

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
pub type NodeId = i64;
pub type DigestedValuesResult = Result<Relation, ParseError>;
pub type ProcessedRelationResult = Result<GraphTheoryRelationManager, ParseError>;

// ENUMS

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum RelationProperty {ANTISYMMETRIC, SYMMETRIC, REFLEXIVE, TRANSITIVE}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum PointRenderSymbol{CIRCLE, TRIANGLE}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum GraphModes{DIGRAPH, MATRIX}
pub enum RelationExplorerModes{EDGES, POINTS}
#[derive(Clone, Copy, PartialEq)]
pub enum GraphTooltips{MOVE, NEW_POINT, CONNECT_EDGE, EDIT_LABEL, DELETE_POINT}
impl GraphTooltips {
    pub fn from_i32(value: i32) -> Self {
        match value {
            0 => GraphTooltips::MOVE,
            1 => GraphTooltips::NEW_POINT,
            2 => GraphTooltips::CONNECT_EDGE,
            3 => GraphTooltips::EDIT_LABEL,
            4 => GraphTooltips::DELETE_POINT,
            _ => GraphTooltips::NEW_POINT, // fallback
        }
    }
    pub fn to_i32(&self) -> i32 {
        match self {
            GraphTooltips::MOVE => 0,
            GraphTooltips::NEW_POINT => 1,
            GraphTooltips::CONNECT_EDGE => 2,
            GraphTooltips::EDIT_LABEL => 3,
            GraphTooltips::DELETE_POINT => 4,
        }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum GraphTheoryTypes {TREE, CIRCULAR, CLIQUE, NETWORK, LAYERED_NETWORK, CHAIN, DISCONNECTED, ISOLATED_POINT}
impl fmt::Display for GraphTheoryTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            GraphTheoryTypes::TREE => "tree",
            GraphTheoryTypes::CIRCULAR => "circular",
            GraphTheoryTypes::CLIQUE => "clique",
            GraphTheoryTypes::NETWORK => "network",
            GraphTheoryTypes::LAYERED_NETWORK => "layered_network",
            GraphTheoryTypes::CHAIN => "chain",
            GraphTheoryTypes::DISCONNECTED => "disconnected",
            GraphTheoryTypes::ISOLATED_POINT => "point"
        };

        write!(f, "{}", msg)
    }
}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum NodeType {ROOT, NORMAL, END, CIRCLE_ROOT}

#[derive(PartialEq, Clone)]
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
        let center_vx = (self.width as f32) / 2.0 + self.offset_x as f32;
        let center_vy = (self.height as f32) / 2.0 + self.offset_y as f32;
        let v_over_l = self.zoom * (min(self.width, self.height) as f32) / SCALING_CONSTANT;

        (center_vx + lx * v_over_l, center_vy + ly * v_over_l)
    }
    pub fn logical_to_visual_scalar(self, l: f32) -> f32 {
        self.zoom * (min(self.width, self.height) as f32) / SCALING_CONSTANT
    }

    pub fn visual_to_logical_xy(self, vx: f32, vy: f32) -> (f32, f32) {
        let center_vx = (self.width as f32) / 2.0 + self.offset_x as f32;
        let center_vy = (self.height as f32) / 2.0 + self.offset_y as f32;
        let l_over_v = SCALING_CONSTANT / ((min(self.width, self.height) as f32) * self.zoom);

        ((vx - center_vx) * l_over_v, (vy - center_vy) * l_over_v)
    }
    pub fn visual_to_logical_scalar(self, l: f32) -> f32 {
        l / ((min(self.width, self.height) as f32) / SCALING_CONSTANT * self.zoom)
    }

    pub fn pointer_to_logical_xy(self, vx: f32, vy: f32) -> (f32, f32) {
        let center_vx = (self.width as f32) / 2.0 + self.offset_x as f32;
        let center_vy = (self.height as f32) / 2.0 + self.offset_y as f32;
        let l_over_v = SCALING_CONSTANT / ((min(self.width, self.height) as f32) * self.zoom);

        ((vx - self.dom_element_offset_x - center_vx) * l_over_v, (vy - self.dom_element_offset_y - center_vy) * l_over_v)
    }
}


#[derive(PartialEq, Clone)]
pub struct ObjectSelection {
    pub inspect_selection: Option<DrawObjectSelection>,
    pub edge_connection_selection_point: Option<PointLabel>
}
impl ObjectSelection {
    // Intake a raw tuple string pairing and set it as the selected object type
    pub fn from_edge(paring: EdgePair) -> Self {
        Self{
            inspect_selection: Option::from(DrawObjectSelection::Edge(paring)),
            edge_connection_selection_point: None
        }
    }
    // Intake a raw string and set it as the selected object type
    pub fn from_point(point: PointLabel) -> Self {
        Self{
            inspect_selection: Option::from(DrawObjectSelection::Point(point)),
            edge_connection_selection_point: None
        }
    }
    pub fn default() -> Self {
        Self {
            inspect_selection: None,
            edge_connection_selection_point: None
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct PointInteraction {
    pub is_selected: bool,
    pub is_hovered: bool,
    pub is_info: bool
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
impl Relation {
    pub fn empty() -> Self {
        Self {
            values: HashSet::new(),
            points: HashSet::new(),
            properties: RelationProperties::default()
        }
    }
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

#[derive(Properties, PartialEq, Clone)]
pub struct GraphEditCallbacks {
    pub on_edit_point: Callback<(PointLabel, f32, f32)>,
    pub on_point_create: Callback<(PointLabel, f32, f32)>,
    pub on_point_delete: Callback<PointLabel>,
    pub on_edge_connection: Callback<(PointLabel, PointLabel)>,
    pub on_point_create_and_connect: Callback<(PointLabel, f32, f32, PointLabel)>
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

#[derive(Debug, PartialEq, Clone)]
pub struct PointManagementError {
    pub message: String,
}
impl PointManagementError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}


