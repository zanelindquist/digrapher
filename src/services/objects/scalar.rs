use yew::prelude::*;

use crate::{render::styles::ScalarStyle, services::{digraph_services::types::CanvasPositioning, objects::matrix::Matrix}};


pub struct ScalarPositioning {
    pub offset_vx: i32,
    pub offset_vy: i32,
}
impl ScalarPositioning {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            offset_vx: x as i32,
            offset_vy: y as i32
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Scalar {
    value: f64,
}
impl Scalar {
    pub fn from_f64(f: f64) -> Self {
        Self {
            value: f
        }
    }

    pub fn add_s(&self, s: &Scalar) -> Scalar {
        Scalar::from_f64(self.value + s.value)
    }
    pub fn add_m(&self, m: &Matrix) -> Matrix {
        let mut matrix = Matrix::create(m.rows, m.cols);
        for row in matrix.data.iter_mut() {
            for val in row.iter_mut() {
                *val += self.value;
            }
        }
        matrix
    }

    pub fn draw(&self, style: &ScalarStyle, scalar_pos: &ScalarPositioning, canvas_pos: &CanvasPositioning) -> Html {
        html! {
            <text
                x={(canvas_pos.offset_x + scalar_pos.offset_vx).to_string()}
                y={(canvas_pos.offset_y + scalar_pos.offset_vy).to_string()}
                text-anchor="middle"
                dominant-baseline="middle"
                font-size={(style.font.size * canvas_pos.zoom).to_string()}
                fill={style.font.fill}
            >
                { self.value.to_string() }
            </text>
        }
    }
}