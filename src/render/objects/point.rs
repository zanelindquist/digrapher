use std::f32::consts::PI;

use gloo_console::log;
use serde::{Serialize, Deserialize};
use yew::prelude::*;

use crate::{render::styles::RenderStyles, services::digraph_services::types::{CanvasPositioning, PointRenderSymbol}};

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
    
    pub fn distance_to(self, x: f32, y: f32) -> f32 {
        ((x - self.x).powi(2) + (y - self.y).powi(2)).sqrt()
    }

    // Takes in visual data to make a comparison
    pub fn pointer_by(self, pointer_vx: f32, pointer_vy: f32, v_radius: f32, canvas_pos: CanvasPositioning) -> bool {
        // Translate visual to logical
        // Pointer x and y are absolute with relation to the screen, so we need to get the pixel value in relation to the canvas
        let (lx, ly) = canvas_pos.visual_to_logical_xy(pointer_vx - canvas_pos.dom_element_offset_x, pointer_vy - canvas_pos.dom_element_offset_y);
        let l_radius = canvas_pos.visual_to_logical_scalar(v_radius);

        // Translate the the radius to logical units
        self.distance_to(lx, ly) < l_radius
    }

    pub fn draw(self, styles: RenderStyles, canvas_pos: CanvasPositioning, is_selected: bool) -> Html {
        // 
        let (x, y) = canvas_pos.logical_to_visual_xy(self.x, self.y);

        let triangle_points: Vec<(f32, f32)> = vec![
            (x + self.bearing.cos() * styles.point.radius, y + self.bearing.sin() * styles.point.radius),
            (x + (self.bearing + 2.0/3.0 * PI).cos() * styles.point.radius, y + (self.bearing + 2.0/3.0 * PI).sin() * styles.point.radius),
            (x + (self.bearing + 4.0/3.0 * PI).cos() * styles.point.radius, y + (self.bearing + 4.0/3.0 * PI).sin() * styles.point.radius),
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
                        x={(x + self.bearing.cos() * (styles.font.size + 10.0)).to_string()}
                        y={(y + self.bearing.sin() * (styles.font.size + 10.0)).to_string()}
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
                        cx={x.to_string()}
                        cy={y.to_string()}
                        r={styles.point.radius.to_string()}
                        fill={fill_color.clone()}
                        stroke={styles.point.stroke}
                        stroke-width={styles.point.stroke_width.to_string()}
                    />
                    <text
                        x={(x + self.bearing.cos() * (styles.font.size + 10.0)).to_string()}
                        y={(y + self.bearing.sin() * (styles.font.size + 10.0)).to_string()}
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