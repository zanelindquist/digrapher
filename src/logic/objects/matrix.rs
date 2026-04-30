use gloo_console::log;
use yew::prelude::*;

use crate::{logic::types::{EdgeVector, MatrixData, PointVector}, render::{objects::point::Point, styles::MatrixStyle}};

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
    
    pub fn draw(self, style: MatrixStyle, center: Point) -> Html {
        let cell_size = style.cell_size; // spacing between cells
        let total_w = self.cols * cell_size;
        let total_h = self.rows * cell_size;
        let serif_w = cell_size / 3;

        // Upper left coordinates
        let ul_x = center.x as i32 - total_w / 2;
        let ul_y = center.y as i32 - total_h / 2;

        html! {
            <g>
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
                                    font-size={style.font.size.to_string()}
                                    fill={style.font.fill}
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
                    stroke={style.stroke}
                    stroke-width={style.stroke_width.to_string()}
                />
                <path
                    d={format!("M {} {} L {} {} L {} {} L {} {} ",
                        ul_x + total_w - serif_w, ul_y,
                        ul_x + total_w, ul_y,
                        ul_x + total_w, ul_y + total_h,
                        ul_x + total_w - serif_w, ul_y + total_h
                    )}
                    fill="none"
                    stroke={style.stroke}
                    stroke-width={style.stroke_width.to_string()}
                />
            </g>
        }
    }
}
