use yew::prelude::*;

use crate::logic::types::{DrawObjectSelection, EdgePair, ObjectSelection, Relation};

#[derive(Properties, PartialEq)]
pub struct ExplorerProps {
    pub relation: Relation,
    pub object_selection: UseStateHandle<ObjectSelection>
}

#[function_component(Explorer)]
pub fn analytics(props: &ExplorerProps) -> Html {
    let properties = props.relation.properties.clone();

    let selected = {
        let selection = props.object_selection.clone();
        Callback::from(move |pairing| {
            selection.set(ObjectSelection::from_edge(pairing))
        })
    };

    html! {
        <div class="explorer">
            {for props.relation.values.iter().map(|(a, b)| {
                let selected = selected.clone();
                let pairing = (a.clone(), b.clone());
                
                html!{
                    <button
                        // class="explorer__row"
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
                            )}
                        onclick={Callback::from(move |_| {
                            selected.emit(pairing.clone());
                        })}
                    >
                        <code>{format!("({}, {})", a, b)}</code>
                    </button>
                }
            })}
        </div>
    }
}