use yew::prelude::*;

use crate::services::digraph_services::classify_relation::GraphTheoryRelationManager;
use crate::services::digraph_services::point_layout::create_edges;
use crate::services::digraph_services::types::{CanvasPositioning, DrawObjectSelection, EdgeVector, GraphEditCallbacks, GraphTooltips, ObjectSelection, PointInteraction, PointLabel, PointVector};
use crate::render::styles::RenderStyles;
use crate::render::objects::point::Point;


#[derive(Properties, PartialEq)]
pub struct DigraphCanvasProps {
    pub position: CanvasPositioning,
    pub processed_relation: GraphTheoryRelationManager,
    #[prop_or_default]
    pub styles: RenderStyles,
    #[prop_or_default]
    pub class: Classes,
    pub object_selection: UseStateHandle<ObjectSelection>,
    pub interrupt_graph_scrolling: UseStateHandle<bool>,
    pub points: UseStateHandle<PointVector>,
    pub graph_edit_callbacks: GraphEditCallbacks
    , pub graph_editing_mode: UseStateHandle<Option<GraphTooltips>>
}

#[function_component(DigraphCanvas)]
pub fn canvas(props: &DigraphCanvasProps) -> Html {
    // The actual points live here
    let points = props.points.clone();
    let edges: UseStateHandle<EdgeVector> = use_state(|| create_edges(&props.processed_relation.relation.values, &(*points)));
    let styles = props.styles;

    // Point selecton and moving
    let selected_point: UseStateHandle<Option<Point>> = use_state(|| None);
    let hovered_point: UseStateHandle<Option<PointLabel>> = use_state(|| None);

    // Update edges when the points change
    {
        let points = points.clone();
        let edges = edges.clone();
        let relation = props.processed_relation.relation.clone();
        let values = relation.values.clone();
        use_effect_with(points.clone(), move |_| {
            edges.set(create_edges(&values, &points));
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
            let mut clicked_point = false;
            // Map through each point and see if the pointer is on it
            for point in points.iter() {
                // If the mouse is clicking on a point
                if point.clone().pointer_by(x as f32, y as f32, radius, canvas_pos) {
                    clicked_point = true;
                    // Interrup the graph's scrolling
                    interrupt.set(true);
                    // Set the point as our selected point
                    selected_point.set(Some(point.clone()));
                    // Set the info selection 
                    object_selection.set(ObjectSelection { selection: Some(DrawObjectSelection::Point(point.label.clone())) });
                    break;
                }
            }

            if !clicked_point {
                object_selection.set(ObjectSelection::default());
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
        let on_edit_point = props.graph_edit_callbacks.on_edit_point.clone();
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

            let offset_vx = e.client_x();
            let offset_vy = e.client_y();

            if let Some(modified_point) = (*selected_point).clone() {
                let (offset_lx, offset_ly) = canvas_pos.pointer_to_logical_xy(offset_vx as f32, offset_vy as f32);
                // Emit an event instead
                on_edit_point.emit((modified_point.label, offset_lx, offset_ly));
            }
            
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
