use gloo_console::log;
use yew::prelude::*;

use crate::services::digraph_services::point_layout::{create_edges, position_points};
use crate::services::digraph_services::types::{CanvasPositioning, DrawObjectSelection, EdgeVector, ObjectSelection, PointInteraction, PointLabel, PointVector, Relation};
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
    let points: UseStateHandle<PointVector> = use_state(|| position_points(&sorted_points));
    let edges: UseStateHandle<EdgeVector> = use_state(|| create_edges(&props.relation.values, &(*points)));
    let styles = props.styles;

    // Point selecton and moving
    let selected_point: UseStateHandle<Option<Point>> = use_state(|| None);
    let hovered_point: UseStateHandle<Option<PointLabel>> = use_state(|| None);

    // Update points and edges when the relation changes
    {
        let points = points.clone();
        let edges = edges.clone();
        let relation = props.relation.clone();
        let values = relation.values.clone();
        let rel_pts = relation.points.clone();
        use_effect_with(relation, move |_| {
            let mut sp: Vec<String> = rel_pts.clone().into_iter().collect();
            sp.sort();
            let new_points = position_points(&sp);
            edges.set(create_edges(&values, &new_points));
            points.set(new_points.clone());
        });
    }

    // Functions

    let on_pointer_down = {
        let selected_point = selected_point.clone();
        let canvas_pos = props.position.clone();
        let points = points.clone();
        let radius = styles.point.radius.clone();
        let interrupt = props.interrupt_graph_scrolling.clone();
        let object_selection = props.object_selection.clone();
        Callback::from(move |e: PointerEvent| {
            let x = e.client_x();
            let y = e.client_y();
            // Map through each point and see if the pointer is on it
            for point in points.iter() {
                // If the mouse is clicking on a point
                if point.clone().pointer_by(x as f32, y as f32, radius, canvas_pos) {
                    // Interrup the graph's scrolling
                    interrupt.set(true);
                    // Set the point as our selected point
                    selected_point.set(Some(point.clone()));
                    // Set the info selection 
                    object_selection.set(ObjectSelection { selection: Some(DrawObjectSelection::Point(point.label.clone())) });
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

    // Update the selected point's location 
    let on_pointer_move = {
        let selected_point = selected_point.clone();
        let hovered_point = hovered_point.clone();
        let radius = styles.point.radius.clone();
        let canvas_pos = props.position.clone();
        let points = points.clone();
        let edges = edges.clone();
        let values = props.relation.values.clone();
        Callback::from(move |e: PointerEvent| {
            // HOVERING ON A POINT
            let x = e.client_x();
            let y = e.client_y();
            let mut is_hovering_point = false;
            // Map through each point and see if the pointer is on it
            for point in points.iter() {
                // If we are hovering on a point
                if point.clone().pointer_by(x as f32, y as f32, radius, canvas_pos) {
                    hovered_point.set(Some(point.label.clone()));
                    is_hovering_point = true;
                    break;
                }
            }
            // If we aren't next to a point
            if !is_hovering_point {
                hovered_point.set(None);
            }

            // MOVING A POINT

            // If no point is selected, return
            let Some(mut modified_point) = (*selected_point).clone() else {
                return;
            };

            // Get visual px difference in moving the mouse
            let offset_vx = e.client_x();// - (*last_pos).0;
            let offset_vy = e.client_y();// - (*last_pos).1;

            // Get logical coords
            let (offset_lx, offset_ly) = canvas_pos.pointer_to_logical_xy(offset_vx as f32, offset_vy as f32);

            // Modify the selected point's coordinates
            modified_point.x = offset_lx;
            modified_point.y = offset_ly;

            // Replace old point
            let mut new_points: PointVector = points
                .iter()
                .filter(|pt| pt.index != modified_point.index)
                .cloned()
                .collect();
            new_points.push(modified_point);

            // Update the edges
            edges.set(create_edges(&values, &new_points));
            // Update the points
            points.set(new_points.clone());
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
                    edge.clone().draw(&styles, &props.position, is_selected)
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

                    let point_interaction = PointInteraction {
                        is_selected,
                        is_hovered: hovered_point.as_ref().is_some_and(|p| *p == point.label),
                        is_info: false
                    };

                    // Draw the point on the digraph
                    point.clone().draw(&styles, &props.position, &point_interaction)
                })}
            </svg>
        </div>
    }
}