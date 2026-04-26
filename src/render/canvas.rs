use yew::prelude::*;

use crate::logic::digest_values::Relation;
use crate::render::objects::point::{Point};

#[derive(Clone, Copy, PartialEq)]
pub struct CanvasPositioning {
    pub offset_x: i16,
    pub offset_y: i16,
    pub zoom: f32,
}

#[derive(Properties, PartialEq)]
pub struct CanvasProps {
    pub position: CanvasPositioning,
    pub relation: Relation,
}

#[function_component(Canvas)]
pub fn canvas(props: &CanvasProps) -> Html {

    html!{
        <div>

        </div>
    }
}