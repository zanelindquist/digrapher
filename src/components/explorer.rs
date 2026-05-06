use yew::prelude::*;

use crate::{components::toggle::{Toggle, ToggleOption}, logic::types::{DrawObjectSelection, EdgePair, ObjectSelection, Relation, RelationExplorerModes}};

#[derive(Properties, PartialEq)]
pub struct ExplorerProps {
    pub relation: Relation,
    pub object_selection: UseStateHandle<ObjectSelection>
}

#[function_component(Explorer)]
pub fn analytics(props: &ExplorerProps) -> Html {
    let properties = props.relation.properties.clone();
    // Our own internal way of keeping track of the selection for scrolling
    let values: Vec<_> = props.relation.values.iter().cloned().collect();

    let selected_index = use_state(|| 0);
    let display_mode = use_state(|| RelationExplorerModes::EDGES);

    let toggle_mode = {
        let diplay_mode = display_mode.clone();
        Callback::from(move |int| {
            match int {
                0 => {
                    display_mode.set(RelationExplorerModes::EDGES);
                },
                1 => {
                    display_mode.set(RelationExplorerModes::POINTS);
                },
                _ => {}
            }
        })
    };

    let selected = {
        let selection = props.object_selection.clone();
        let selected_index = selected_index.clone();
        Callback::from(move |(pairing, index)| {
            selected_index.set(index as i32);
            selection.set(ObjectSelection::from_edge(pairing))
        })
    };

    let onkeydown = {
        let selected_index = selected_index.clone();
        let selected = selected.clone();
        let values = values.clone();

        Callback::from(move |e: KeyboardEvent| {
            if values.is_empty() {
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

            let len = values.len() as i32;
            let index = (index + len) % len;

            selected_index.set(index);

            let pairing = values[index as usize].clone();
            selected.emit((pairing, index));
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
            {for props.relation.values.iter().enumerate().map(|(index, (a, b))| {
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
                                selected.emit((pairing.clone(), index as i32));
                            })
                        }
                    >
                        <code>{format!("({}, {})", a, b)}</code>
                    </button>
                }
            })}
        </div>
    }
}