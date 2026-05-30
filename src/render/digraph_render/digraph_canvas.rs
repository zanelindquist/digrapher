use yew::prelude::*;

use crate::services::digraph_services::classify_relation::GraphTheoryRelationManager;
use crate::services::digraph_services::point_layout::create_edges;
use crate::services::digraph_services::types::{CanvasPositioning, DrawObjectSelection, EdgeVector, GraphEditCallbacks, GraphTooltips, ObjectSelection, PointInteraction, PointLabel, PointVector};
use crate::render::styles::RenderStyles;


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
    pub graph_edit_callbacks: GraphEditCallbacks,
    pub graph_editing_mode: UseStateHandle<Option<GraphTooltips>>
}

#[function_component(DigraphCanvas)]
pub fn canvas(props: &DigraphCanvasProps) -> Html {
    // The actual points live here
    let points = props.points.clone();
    let edges: UseStateHandle<EdgeVector> = use_state(|| create_edges(&props.processed_relation.relation.values, &(*points)));
    let styles = props.styles;

    // Point selecton and moving
    let selected_point: UseStateHandle<Option<PointLabel>> = use_state(|| None);
    let hovered_point: UseStateHandle<Option<PointLabel>> = use_state(|| None);

    // Update edges when the points change
    {
        let points = points.clone();
        let edges = edges.clone();
        let relation = props.processed_relation.relation.clone();
        let values = relation.values.clone();
        use_effect_with((points.clone(), values.clone()), move |_| {
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
        let graph_editing_mode = props.graph_editing_mode.clone();
        Callback::from(move |e: PointerEvent| {
            let x = e.client_x();
            let y = e.client_y();

            let mut clicked_point = false;
            // Map through each point and see if the pointer is on it
            for point in points.iter() {
                // If the mouse is clicking on a point
                if point.clone().pointer_by(x as f32, y as f32, radius, canvas_pos) {
                    clicked_point = true;

                    // Check and see if we are on an edge connect right now with an unconnected edge
                    if let Some(mode) = *graph_editing_mode {
                        if mode == GraphTooltips::CONNECT_EDGE {
                            return;
                        }
                    }

                    // Interrup the graph's scrolling
                    interrupt.set(true);
                    // Set the point as our selected point
                    selected_point.set(Some(point.label.clone()));
                    // Set the info selection 
                    object_selection.set(ObjectSelection {
                        inspect_selection: Some(DrawObjectSelection::Point(point.label.clone())),
                        edge_connection_selection_point: None
                    });
                    break;
                }
            }

            // Clear info object selection
            // Don't clear editing selection because we want to handle those in mouseup
            if !clicked_point {
                let mut os = (*object_selection).clone();
                os.inspect_selection = None;
                object_selection.set(os);
            }
        })
    };
    
    let on_pointer_up = {
        let interrupt = props.interrupt_graph_scrolling.clone();
        let selected_point = selected_point.clone();
        let canvas_pos = props.position.clone();
        let radius = styles.point.radius.clone();
        let points = points.clone();
        let graph_editing_mode = props.graph_editing_mode.clone();
        let edit_callbacks =  props.graph_edit_callbacks.clone();
        let object_selection = props.object_selection.clone();
        Callback::from(move |e: PointerEvent| {
            let x = e.client_x();
            let y = e.client_y();
            
            // Our response to this click depends on if graph editing is engaged

            if let Some(edit_mode) = *graph_editing_mode {
                match edit_mode {
                    GraphTooltips::NEW_POINT => {
                        // Get the coordinates
                        let (lx, ly) = canvas_pos.pointer_to_logical_xy(x as f32, y as f32);
                        // Create the point
                        edit_callbacks.on_point_create.emit(((points.len() + 1).to_string(), lx, ly));
                        // Turn the mode to edge connection so we must set an edge?
                        // graph_editing_mode.set(Some(GraphTooltips::CONNECT_EDGE));
                    },
                    GraphTooltips::CONNECT_EDGE => {
                        // Map through each point and see if the pointer is on it
                        let mut found_point = false;
                        for point in points.iter() {
                            // If the mouse is on a point
                            if point.clone().pointer_by(x as f32, y as f32, radius, canvas_pos) {
                                // If there is a previously selected point, we need to link the edges
                                if let Some(from_label) = &object_selection.edge_connection_selection_point {
                                    edit_callbacks.on_edge_connection.emit((from_label.clone(), point.label.clone()));

                                    // Clear the conenction selection now
                                    let mut new_obj_selection = (*object_selection).clone();
                                    new_obj_selection.edge_connection_selection_point = None;
                                    object_selection.set(new_obj_selection);
                                }
                                // If there is no selected edge connection point yet, set one and return
                                else {
                                    let mut new_obj_selection = (*object_selection).clone();
                                    new_obj_selection.edge_connection_selection_point = Some(point.label.clone());
                                    object_selection.set(new_obj_selection);
                                }

                                // We found what we want, we can break now
                                found_point = true;
                                break;
                            }
                        }

                        // If we didn't click on a point, we need to create a new point
                        if !found_point {
                            // First let's make sure that we have a draw object selection
                            if let Some(from_label) = &object_selection.edge_connection_selection_point {
                                let (lx, ly) = canvas_pos.pointer_to_logical_xy(x as f32, y as f32);
                                let new_label = (points.len() + 1).to_string();
                                // Create the point and link the edge in one fresh update
                                edit_callbacks.on_point_create_and_connect.emit((new_label.clone(), lx, ly, from_label.clone()));
                                // Set the new point as the new connection selection point
                                let mut new_obj_selection = (*object_selection).clone();
                                new_obj_selection.edge_connection_selection_point = Some(new_label);
                                object_selection.set(new_obj_selection);
                            }      
                        }
                    },
                    GraphTooltips::DELETE_POINT => {

                    },
                    GraphTooltips::EDIT_LABEL => {

                    },
                    _ => {

                    }
                }
            } else {

            }

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
            let x = e.client_x();
            let y = e.client_y();

            // DRAW LINE TO EDGE SELECTION

            // HOVERING ON A POINT

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
                on_edit_point.emit((modified_point, offset_lx, offset_ly));
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
                        &props.object_selection.inspect_selection,
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
                        &props.object_selection.inspect_selection,
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
