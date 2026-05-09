use yew::prelude::*;

use crate::logic::calculate_render::{position_edges, position_points};
use crate::logic::types::{CanvasPositioning, DrawObjectSelection, EdgeVector, ObjectSelection, PointVector, Relation};
use crate::render::styles::RenderStyles;


#[derive(Properties, PartialEq)]
pub struct DigraphCanvasProps {
    pub position: CanvasPositioning,
    pub relation: Relation,
    #[prop_or_default]
    pub styles: RenderStyles,
    #[prop_or_default]
    pub class: Classes,
    pub object_selection: UseStateHandle<ObjectSelection>
}

#[function_component(DigraphCanvas)]
pub fn canvas(props: &DigraphCanvasProps) -> Html {
    // Sort the points first so we have a same order every time
    let mut sorted_points: Vec<String> = props.relation.points.clone().into_iter().collect();
    sorted_points.sort();

    let points: PointVector = position_points(sorted_points, props.position);
    let edges: EdgeVector = position_edges(props.relation.values.clone(), points.clone());
    let styles = props.styles;

    html!{
        <div
            class={classes!("canvas", props.class.clone())}
        >
            <svg
                class="canvas__svg"
                width={props.position.width.to_string()}
                height={props.position.height.to_string()}
            >
                { for edges.iter().map(|edge| {
                    let is_selected = matches!(
                        &props.object_selection.selection,
                        Some(DrawObjectSelection::Edge(label))
                            if edge.start.label == label.0 && edge.end.label == label.1
                    );
                    edge.clone().draw(styles, is_selected)
                })}
                { for points.iter().map(|point| {
                    let is_selected = matches!(
                        &props.object_selection.selection,
                        Some(DrawObjectSelection::Point(pt))
                            if point.label == *pt
                    );
                    point.clone().draw(styles, is_selected)
                })}
            </svg>
        </div>
    }
}