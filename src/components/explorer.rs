use std::any::Any;

use yew::prelude::*;

use crate::{components::toggle::{Toggle, ToggleOption}, logic::types::{DrawObjectSelection, EdgePair, ObjectSelection, Relation, RelationExplorerModes}};

#[derive(Properties, PartialEq)]
pub struct ExplorerProps {
    pub relation: Relation,
    pub object_selection: UseStateHandle<ObjectSelection>
}

#[function_component(Explorer)]
pub fn explorer(props: &ExplorerProps) -> Html {
    // Our own internal way of keeping track of the selection for scrolling
    let edges: Vec<_> = props.relation.values.iter().cloned().collect();
    let points: Vec<_> = props.relation.points.iter().cloned().collect();

    let selected_index = use_state(|| 0);
    let display_mode = use_state(|| RelationExplorerModes::EDGES);

    let toggle_mode = {
        let dm = display_mode.clone();
        let selected_index = selected_index.clone();
        Callback::from(move |int| {
            // Reset the selected index so we don't go out of bounds
            selected_index.set(0);
            match int {
                0 => {
                    dm.set(RelationExplorerModes::EDGES);
                },
                1 => {
                    dm.set(RelationExplorerModes::POINTS);
                },
                _ => {}
            }
        })
    };

    let selected = {
        let selection = props.object_selection.clone();
        let selected_index = selected_index.clone();
        Callback::from(move |(obj_sel, index)| {
            selected_index.set(index as i32);
            selection.set(obj_sel)
        })
    };

    let onkeydown = {
        let selected_index = selected_index.clone();
        let selected = selected.clone();
        let edges = edges.clone();
        let points = points.clone();
        let display_mode = display_mode.clone();

        Callback::from(move |e: KeyboardEvent| {
            if edges.is_empty() && points.is_empty() {
                return;
            }

            let mut index = *selected_index;

            match e.key().as_str() {
                "ArrowUp" => {
                    e.prevent_default();
                    index -= 1;
                }
                "ArrowDown" => {
                    e.prevent_default();
                    index += 1;
                }
                _ => return,
            }

            match *display_mode {
                RelationExplorerModes::EDGES => {
                    let len = edges.len() as i32;
                    let index = (index + len) % len;

                    selected_index.set(index);

                    let pairing = edges[index as usize].clone();
                    selected.emit((ObjectSelection::from_edge(pairing), index));
                },
                RelationExplorerModes::POINTS => {
                    let len = points.len() as i32;
                    let index = (index + len) % len;

                    selected_index.set(index);

                    let point = points[index as usize].clone();
                    selected.emit((ObjectSelection::from_point(point), index));
                }
            }
        })
    };

    html! {
        <div
            class="explorer"
            tabindex="0" // makes it focusable
            {onkeydown}
        >
            <Toggle
                class="explorer__toggle"
                onchange={toggle_mode}
                size={20}
            >
                <ToggleOption icon="edge" size={20}/>
                <ToggleOption icon="point" size={20}/>
            </Toggle>

            {match *display_mode {
                // Display edges    
                RelationExplorerModes::EDGES => html!{
                    <>{for props.relation.values.iter().enumerate().map(|(index, (a, b))| {
                        let selected = selected.clone();
                        let pairing = (a.clone(), b.clone());

                        html!{
                            <button
                                class={classes!(
                                        "explorer__row",
                                        match &props.object_selection.selection {
                                            Some(DrawObjectSelection::Edge(pair)) => {
                                                if pair.0 == *a && pair.1 == *b {
                                                    String::from("explorer__row--selected")
                                                } else {
                                                    String::default()
                                                }
                                            },
                                            _ => {String::default()}
                                        }
                                    )
                                }
                                onclick={
                                    Callback::from(move |_| {
                                        selected.emit((ObjectSelection::from_edge(pairing.clone()), index as i32));
                                    })
                                }
                            >
                                <code>{format!("({}, {})", a, b)}</code>
                            </button>
                        }
                    })}</>
                },
                RelationExplorerModes::POINTS => html!{
                    <>{for props.relation.points.iter().enumerate().map(|(index, point)| {
                        let selected = selected.clone();
                        let point = point.clone();
                        let point_borrow = point.clone();
                        
                        html!{
                            <button
                                class={classes!(
                                        "explorer__row",
                                        match &props.object_selection.selection {
                                            Some(DrawObjectSelection::Point(p)) => {
                                                if *p == *point {
                                                    String::from("explorer__row--selected")
                                                } else {
                                                    String::default()
                                                }
                                            },
                                            _ => {String::default()}
                                        }
                                    )
                                }
                                onclick={
                                    Callback::from(move |_| {
                                        selected.emit((ObjectSelection::from_point(point_borrow.clone()), index as i32));
                                    })
                                }
                            >
                                <code>{format!("{}", point)}</code>
                            </button>
                        }
                    })}</>
                }
            }}

        </div>
    }
}