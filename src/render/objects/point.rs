use serde::{Serialize, Deserialize};
use yew::prelude::*;

use crate::{logic::types::PointRenderSymbol, render::styles::RenderStyles};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub bearing: f32, // Direction away from the centerself
    pub label: String,
    pub symbol: PointRenderSymbol
}

impl Point {
    pub fn new(x: f32, y: f32, bearing: f32, label: String, symbol: PointRenderSymbol) -> Self {
        Self { x, y, bearing, label, symbol }
    }
    
    pub fn distance_to(self, other: Point) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

    pub fn draw(self, styles: RenderStyles) -> Html {
        html! {
        <>
            <circle
                cx={self.x.to_string()}
                cy={self.y.to_string()}
                r={styles.dot.radius.to_string()}
                fill={styles.dot.fill}
                stroke={styles.dot.stroke}
                stroke-width={styles.dot.stroke_width.to_string()}
            />
            <text
                x={(self.x + self.bearing.cos() * (styles.font.size + 10.0)).to_string()}
                y={(self.y + self.bearing.sin() * (styles.font.size + 10.0)).to_string()}
                font-family={styles.font.family}
                font-size={styles.font.size.to_string()}
                fill={styles.font.fill}
            >
                { self.label.to_string() }
            </text>
        </>
        }
    }
}