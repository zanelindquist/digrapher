use yew::prelude::*;

use crate::services::digraph_services::types::{CanvasPositioning, DrawObjectSelection, EdgeVector, MatrixData, ObjectSelection, PointVector};
use crate::render::{styles::MatrixStyle};

pub struct MatrixPositioning {
    pub offset_vx: i32,
    pub offset_vy: i32,
}
impl MatrixPositioning {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            offset_vx: x as i32,
            offset_vy: y as i32
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Matrix {
    pub data: MatrixData,
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
            data[edge.start.index as usize][edge.end.index as usize] = 1.0;
        };
        Self {rows: size, cols: size, data, labels}
    }
    pub fn from_values((rows, cols): (i32, i32), values: Vec<f64>) -> Self {
        let mut data = Matrix::initialize_matrix(rows, cols);
        // Crash the program if the matrix is improperly filled
        assert_eq!(rows * cols, values.len() as i32);
        let mut index = 0;
        for row in data.iter_mut() {
            for value in row.iter_mut() {
                *value = values[index];
                index += 1;
            }            
        }

        Self { data, labels: vec![], rows, cols }
    }

    fn initialize_matrix(rows: i32, cols: i32) -> MatrixData {
        vec![vec![0.0; cols as usize]; rows as usize]
    }
    
    pub fn draw(self, style: &MatrixStyle, matrix_pos: &MatrixPositioning, canvas_pos: &CanvasPositioning, object_selection: &ObjectSelection) -> Html {
        // Cell size in visual units
        let cell_size = (style.cell_size as f32 * canvas_pos.zoom) as i32; // spacing between cells
        let total_w = self.cols * cell_size;
        let total_h = self.rows * cell_size;
        let serif_w = cell_size / 3;

        // Upper left coordinates in visual units
        let center_vx = canvas_pos.offset_x + canvas_pos.width / 2;
        let center_vy = canvas_pos.offset_y + canvas_pos.height / 2;
        let ul_vx = center_vx - total_w / 2 + matrix_pos.offset_vx;
        let ul_vy = center_vy - total_h / 2 + matrix_pos.offset_vy;

        html! {
            <g>
                // Legend
                {
                    for self.data.iter().enumerate().map(|(i, _)| {
                        let x1 = ul_vx - cell_size / 2;
                        let y1 = ul_vy + i as i32 * cell_size + cell_size / 2;
                        let x2 = ul_vx + i as i32 * cell_size + cell_size / 2;
                        let y2 = ul_vy - cell_size / 2;
                        html! {
                            <>
                                <text
                                    x={x1.to_string()}
                                    y={y1.to_string()}
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    font-size={style.legend_font.size.to_string()}
                                    fill={style.legend_font.fill}
                                >
                                    { self.labels[i].clone() }
                                </text>
                                <text
                                    x={x2.to_string()}
                                    y={y2.to_string()}
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    font-size={style.legend_font.size.to_string()}
                                    fill={style.legend_font.fill}
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
                        let object_selection = object_selection.clone();
                        let labels = self.labels.clone();
                        row.iter().enumerate().map(move |(j, val)| {
                            let x = j as i32 * cell_size + cell_size / 2 + ul_vx;
                            let y = i as i32 * cell_size + cell_size / 2 + ul_vy;
                            
                            // See if this is the selected relation
                            let is_selected = matches!(
                                &object_selection.inspect_selection,
                                Some(DrawObjectSelection::Edge(edge))
                                    if labels
                                        .iter()
                                        .position(|l| *l == edge.0)
                                        .is_some_and(|edge_i| edge_i == i)
                                    &&
                                    labels
                                        .iter()
                                        .position(|l| *l == edge.1)
                                        .is_some_and(|edge_j| edge_j == j)
                            );

                            let text = if *val != 0.0 { "1" } else { "0" };
                            let text_fill = if is_selected {style.selected_text_color} else {style.font.fill};

                            html! {
                                <>
                                    {match is_selected {
                                        // Draw a box around this item if it is selected
                                        true => html! {
                                            <path
                                                d={format!("M {} {} L {} {} L {} {} L {} {} Z",
                                                    ul_vx + j as i32 * cell_size, ul_vy + i as i32 * cell_size,
                                                    ul_vx + j as i32 * cell_size, ul_vy + (i as i32 + 1) * cell_size,
                                                    ul_vx + (j as i32 + 1) * cell_size, ul_vy + (i as i32 + 1) * cell_size,
                                                    ul_vx + (j as i32 + 1) * cell_size, ul_vy + i as i32 * cell_size,
                                                )}
                                                fill="none"
                                                stroke={style.selected_outline_color}
                                                stroke-width={style.selected_stroke_width.to_string()}
                                            />
                                        },
                                        false => html!{}
                                    }}
                                    <text
                                        x={x.to_string()}
                                        y={y.to_string()}
                                        text-anchor="middle"
                                        dominant-baseline="middle"
                                        font-size={style.font.size.to_string()}
                                        fill={text_fill}
                                    >
                                        { text }
                                    </text>
                                </>
                            }
                        })
                    })
                }
                // Left bracket
                <path
                    d={format!("M {} {} L {} {} L {} {} L {} {} ",
                        ul_vx + serif_w, ul_vy,
                        ul_vx, ul_vy,
                        ul_vx, ul_vy + total_h,
                        ul_vx + serif_w, ul_vy + total_h
                    )}
                    fill="none"
                    stroke={style.stroke}
                    stroke-width={style.stroke_width.to_string()}
                />
                <path
                    d={format!("M {} {} L {} {} L {} {} L {} {} ",
                        ul_vx + total_w - serif_w, ul_vy,
                        ul_vx + total_w, ul_vy,
                        ul_vx + total_w, ul_vy + total_h,
                        ul_vx + total_w - serif_w, ul_vy + total_h
                    )}
                    fill="none"
                    stroke={style.stroke}
                    stroke-width={style.stroke_width.to_string()}
                />
            </g>
        }
    }
}
