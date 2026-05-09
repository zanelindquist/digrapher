use std::f32::consts::PI;

use serde::{Serialize, Deserialize};
use yew::prelude::*;

use crate::{logic::types::PointRenderSymbol, render::styles::RenderStyles};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub bearing: f32, // Direction away from the centerself
    pub label: String,
    pub symbol: PointRenderSymbol,
    pub index: i32,
}

impl Point {
    pub fn new(x: f32, y: f32, bearing: f32, label: String, symbol: PointRenderSymbol, index: i32) -> Self {
        Self { x, y, bearing, label, symbol, index }
    }
    pub fn from_xy(x: f32, y: f32) -> Point {
        Point::new(x, y, 0.0, String::default(), PointRenderSymbol::CIRCLE, 0)
    }
    
    pub fn distance_to(self, other: Point) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

    pub fn draw(self, styles: RenderStyles, is_selected: bool) -> Html {
        let triangle_points: Vec<(f32, f32)> = vec![
            (self.x + self.bearing.cos() * styles.point.radius, self.y + self.bearing.sin() * styles.point.radius),
            (self.x + (self.bearing + 2.0/3.0 * PI).cos() * styles.point.radius, self.y + (self.bearing + 2.0/3.0 * PI).sin() * styles.point.radius),
            (self.x + (self.bearing + 4.0/3.0 * PI).cos() * styles.point.radius, self.y + (self.bearing + 4.0/3.0 * PI).sin() * styles.point.radius),
        ];

        let fill_color = if is_selected {styles.point.highlighted_stroke.to_string()} else {styles.point.fill.to_string()};
        
        match self.symbol {
            PointRenderSymbol::TRIANGLE => html! {
                <>
                    <polygon
                        points={
                            triangle_points
                            .iter()
                            .map(|pair| format!("{}, {}", pair.0, pair.1).to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                        }
                        fill={fill_color.clone()}
                        stroke={styles.point.stroke}
                        // stroke-width={styles.point.stroke_width.to_string()}
                    />
                    <text
                        x={(self.x + self.bearing.cos() * (styles.font.size + 10.0)).to_string()}
                        y={(self.y + self.bearing.sin() * (styles.font.size + 10.0)).to_string()}
                        font-family={styles.font.family}
                        font-size={styles.font.size.to_string()}
                        fill={fill_color.clone()}
                    >
                        { self.label.to_string() }
                    </text>
                </>
            },
            // Default circle rendering
            _ => html! {
                <>
                    <circle
                        cx={self.x.to_string()}
                        cy={self.y.to_string()}
                        r={styles.point.radius.to_string()}
                        fill={fill_color.clone()}
                        stroke={styles.point.stroke}
                        stroke-width={styles.point.stroke_width.to_string()}
                    />
                    <text
                        x={(self.x + self.bearing.cos() * (styles.font.size + 10.0)).to_string()}
                        y={(self.y + self.bearing.sin() * (styles.font.size + 10.0)).to_string()}
                        font-family={styles.font.family}
                        font-size={styles.font.size.to_string()}
                        fill={fill_color.clone()}
                    >
                        { self.label.to_string() }
                    </text>
                </>
            }
        }
    }
}