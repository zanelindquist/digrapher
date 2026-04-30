use gloo_console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use std::{array, cell, collections::HashSet};

use crate::render::{objects::{edge::Edge, point::Point}, styles::RenderStyles};

// TYPES
pub type RawCharPoints = HashSet<String>;
pub type SortedCharPoints = Vec<String>;
pub type RawEdgePairs = HashSet<(String, String)>;
pub type PointVector = Vec<Point>;
pub type EdgeVector = Vec<Edge>;
pub type MatrixData = Vec<Vec<bool>>;

// ENUMS

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum RelationProperty {ANTISYMMETRIC, SYMMETRIC, REFLEXIVE, TRANSITIVE}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum PointRenderSymbol{CIRCLE, TRIANGLE}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum GraphModes{DIGRAPH, MATRIX}

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

pub struct Matrix {
    data: MatrixData,
    labels: Vec<String>,
    pub rows: i32,
    pub cols: i32
} 
impl Matrix {
    pub fn create(rows: i32, cols: i32) -> Self {
        let data = Matrix::initialize_matrix(rows, cols);
        Self {rows, cols, data, labels: vec![String::default(); rows as usize]}
    }
    pub fn from_edges(points: PointVector, edges: EdgeVector) -> Self {
        let size = points.len() as i32;
        let labels = points.iter().map(|p| p.label.clone()).collect();
        let mut data = Matrix::initialize_matrix(size, size);
        for edge in edges {
            data[edge.start.index as usize][edge.end.index as usize] = true;
        };
        Self {rows: size, cols: size, data, labels}
    }
    fn initialize_matrix(rows: i32, cols: i32) -> MatrixData {
        vec![vec![false; cols as usize]; rows as usize]
    }
    
    pub fn draw(self, style: RenderStyles, position: CanvasPositioning) -> Html {
        let cell_size = style.matrix.cell_size; // spacing between cells
        let total_w = self.cols * cell_size;
        let total_h = self.rows * cell_size;
        let serif_w = cell_size / 3;

        // Upper left coordinates
        let ul_x = position.width / 2 - total_w / 2;
        let ul_y = position.height / 2  - total_h / 2;

        log!("center {} {}", ul_x, ul_y);

        html! {
            <svg
                width={position.width.to_string()}
                height={position.height.to_string()}
            >
                // Legend
                {
                    for self.data.iter().enumerate().map(|(i, row)| {
                        let x1 = ul_x - cell_size / 2;
                        let y1 = ul_y + i as i32 * cell_size + cell_size / 2;
                        let x2 = ul_x + i as i32 * cell_size + cell_size / 2;
                        let y2 = ul_y - cell_size / 2;
                        html! {
                            <>
                                <text
                                    x={x1.to_string()}
                                    y={y1.to_string()}
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    font-size={style.matrix.legend_font.size.to_string()}
                                    fill={style.matrix.legend_font.fill}
                                >
                                    { self.labels[i].clone() }
                                </text>
                                <text
                                    x={x2.to_string()}
                                    y={y2.to_string()}
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    font-size={style.matrix.legend_font.size.to_string()}
                                    fill={style.matrix.legend_font.fill}
                                >
                                    { self.labels[i].clone() }
                                </text>
                            </>
                        }
                    })
                }
                // Matrix
                {
                    for self.data.iter().enumerate().flat_map(|(i, row)| {
                        row.iter().enumerate().map(move |(j, val)| {
                            let x = j as i32 * cell_size + cell_size / 2 + ul_x;
                            let y = i as i32 * cell_size + cell_size / 2 + ul_y; 

                            let text = if *val { "1" } else { "0" };

                            html! {
                                <text
                                    x={x.to_string()}
                                    y={y.to_string()}
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    font-size={style.matrix.font.size.to_string()}
                                    fill={style.matrix.font.fill}
                                >
                                    { text }
                                </text>
                            }
                        })
                    })
                }
                // Left bracket
                <path
                    d={format!("M {} {} L {} {} L {} {} L {} {} ",
                        ul_x + serif_w, ul_y,
                        ul_x, ul_y,
                        ul_x, ul_y + total_h,
                        ul_x + serif_w, ul_y + total_h
                    )}
                    fill="none"
                    stroke={style.matrix.stroke}
                    stroke-width={style.matrix.stroke_width.to_string()}
                />
                <path
                    d={format!("M {} {} L {} {} L {} {} L {} {} ",
                        ul_x + total_w - serif_w, ul_y,
                        ul_x + total_w, ul_y,
                        ul_x + total_w, ul_y + total_h,
                        ul_x + total_w - serif_w, ul_y + total_h
                    )}
                    fill="none"
                    stroke={style.matrix.stroke}
                    stroke-width={style.matrix.stroke_width.to_string()}
                />
            </svg>
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

