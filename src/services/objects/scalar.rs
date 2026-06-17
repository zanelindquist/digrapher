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
    pub value: f64,
}
impl Scalar {
    pub fn from_f64(f: f64) -> Self {
        Self {
            value: f
        }
    }

    pub fn width(&self) -> f32 {
        (self.value as f32).to_string().len() as f32 * 0.5 + 0.3
    }
    pub fn height(&self) -> f32 {
        1.0
    }

    pub fn draw(&self, style: &ScalarStyle, scalar_pos: &ScalarPositioning, canvas_pos: &CanvasPositioning) -> Html {
        html! {
            <text
                x={(canvas_pos.width / 2 + canvas_pos.offset_x + scalar_pos.offset_vx).to_string()}
                y={(canvas_pos.height / 2 + canvas_pos.offset_y + scalar_pos.offset_vy).to_string()}
                font-size={(style.font.size * canvas_pos.zoom).to_string()}
                fill={style.font.fill}
            >
                { self.value.to_string() }
            </text>
        }
    }
}