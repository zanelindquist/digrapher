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
                    // Pass is_selected to the draw function based on:
                    let is_selected = matches!(
                        &props.object_selection.selection,
                        // If we have a selection that is valid AND is an Edge
                        Some(DrawObjectSelection::Edge(label))
                            // AND the labels on the start and end match the selection
                            if edge.start.label == label.0 && edge.end.label == label.1
                    );
                    // Draw the edge on the digraph
                    edge.clone().draw(styles, is_selected)
                })}
                { for points.iter().map(|point| {
                    // Pass is_selected to the draw function based on:
                    let is_selected = matches!(
                        &props.object_selection.selection,
                        // If we have a defined selection that is a point
                        Some(DrawObjectSelection::Point(pt))
                            // AND the point's string label is equal to this point
                            if point.label == *pt
                    );
                    // Draw the point on the digraph
                    point.clone().draw(styles, is_selected)
                })}
            </svg>
        </div>
    }
}