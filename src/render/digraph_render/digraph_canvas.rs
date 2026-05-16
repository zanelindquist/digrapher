use yew::prelude::*;

use crate::services::digraph_services::calculate_render::{position_edges, position_points};
use crate::services::digraph_services::types::{CanvasPositioning, DrawObjectSelection, EdgeVector, ObjectSelection, PointVector, Relation};
use crate::render::styles::RenderStyles;
use crate::render::objects::point::Point;


#[derive(Properties, PartialEq)]
pub struct DigraphCanvasProps {
    pub position: CanvasPositioning,
    pub relation: Relation,
    #[prop_or_default]
    pub styles: RenderStyles,
    #[prop_or_default]
    pub class: Classes,
    pub object_selection: UseStateHandle<ObjectSelection>,
    pub interrupt_graph_scrolling: UseStateHandle<bool>
}

#[function_component(DigraphCanvas)]
pub fn canvas(props: &DigraphCanvasProps) -> Html {
    // Sort the text of the points first so we have a same order every time
    let mut sorted_points: Vec<String> = props.relation.points.clone().into_iter().collect();
    sorted_points.sort();

    // The actual points live here
    let points: UseStateHandle<PointVector> = use_state(|| position_points(sorted_points.clone()));
    let edges: UseStateHandle<EdgeVector> = use_state(|| position_edges(props.relation.values.clone(), (*points).clone()));
    let styles = props.styles;

    // Point selecton and moving
    let selected_point: UseStateHandle<Option<Point>> = use_state(|| None);
    let last_pos = use_state(|| (0,0));

    // Functions

    let on_pointer_down = {
        let selected_point = selected_point.clone();
        let last_pos = last_pos.clone();
        let points = points.clone();
        let radius = styles.point.radius.clone();
        let interrupt = props.interrupt_graph_scrolling.clone();
        Callback::from(move |e: PointerEvent| {
            let x = e.client_x();
            let y = e.client_y();
            // Map through each point and see if the pointer is on it
            for point in points.iter() {
                // If the mouse is clicking on a point
                if point.clone().in_proximity(x as f32, y as f32, radius) {
                    // Interrup the graph's scrolling
                    interrupt.set(true);
                    // Set the point as our selected point
                    selected_point.set(Some(point.clone()));
                    // Set the last_position as the mouse's current position
                    last_pos.set((x, y));
                }
            }
        })
    };
    
    let on_pointer_up = {
        let interrupt = props.interrupt_graph_scrolling.clone();
        let selected_point = selected_point.clone();
        Callback::from(move |_: PointerEvent| {
            interrupt.set(false);
            selected_point.set(None);
        })
    };

    let on_pointer_move = {

        Callback::from(move |_| {

        })
    };


    html!{
        <div
            class={classes!("canvas", props.class.clone())}
            onpointerdown={on_pointer_down}
            onpointermove={on_pointer_move}
            onpointerup={on_pointer_up}
        >
            <svg
                class="canvas__svg"
                width={props.position.width.to_string()}
                height={props.position.height.to_string()}
            >
                { for (*edges).iter().map(|edge| {
                    // Pass is_selected to the draw function based on:
                    let is_selected = matches!(
                        &props.object_selection.selection,
                        // If we have a selection that is valid AND is an Edge
                        Some(DrawObjectSelection::Edge(label))
                            // AND the labels on the start and end match the selection
                            if edge.start.label == label.0 && edge.end.label == label.1
                    );
                    // Draw the edge on the digraph
                    edge.clone().draw(styles, props.position, is_selected)
                })}
                { for (*points).iter().map(|point| {
                    // Pass is_selected to the draw function based on:
                    let is_selected = matches!(
                        &props.object_selection.selection,
                        // If we have a defined selection that is a point
                        Some(DrawObjectSelection::Point(pt))
                            // AND the point's string label is equal to this point
                            if point.label == *pt
                    );
                    // Draw the point on the digraph
                    point.clone().draw(styles, props.position, is_selected)
                })}
            </svg>
        </div>
    }
}