use std::ops::Deref;

use yew::prelude::*;

use crate::components::digraph_tools::analytics::Analytics;
use crate::components::digraph_tools::explorer::Explorer;
use crate::components::digraph_tools::library::RelationLibrary;
use crate::components::misc::button::Button;
use crate::services::digraph_services::types::{DigestedValuesResult, ObjectSelection, StoredRelation};
use crate::services::digraph_services::relation_storage::{store_new_relation};

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub value: String,
    pub on_input: Callback<String>,
    pub digested_values: UseStateHandle<DigestedValuesResult>,
    pub object_selection: UseStateHandle<ObjectSelection>
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let display_library = use_state(|| false);

    let oninput: Callback<InputEvent> = {
        let input_value = props.on_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            input_value.emit(input.value());
        })
    };

    let on_toggle_library: Callback<MouseEvent> = {
        let display_library = display_library.clone();
        Callback::from(move |_: MouseEvent| {
            display_library.set(!*display_library);
        })
    };
    let save_relation: Callback<MouseEvent> = {
        let dv = props.digested_values.clone();
        Callback::from(move |_: MouseEvent| {
            if let Ok(relation) = dv.deref() {
                let _ = store_new_relation(relation);
            }
        })
    };
    let load_saved_relation: Callback<StoredRelation> = {
        let input_value = props.on_input.clone();
        Callback::from(move |relation: StoredRelation| {
            input_value.emit(relation.raw_text);
        })
    };
    
    html! {
        <aside class="sidebar">
            <div class="sidebar__header">
                <h2 class="sidebar__title">{"Digraph"}</h2>
                <p class="sidebar__subtitle">{"Graph visualization"}</p>
            </div>
            
            <div class="sidebar__content">
                <div class="sidebar__input-group">
                    <div class="sidebar__input__container">
                        <label class="sidebar__label" for="graph-input">{"Enter graph"}</label>
                        <Button
                            class="sidebar__input__toggle"
                            onclick={on_toggle_library}
                        >{if *display_library {"Library"} else {"Create"}}</Button>
                    </div>
                    
                    if *display_library {
                        <RelationLibrary
                            onselect={load_saved_relation}
                        />
                    } else {
                        <textarea 
                            id="graph-input"
                            class="sidebar__input"
                            rows="5"
                            placeholder="{(a, b), (b, c), (c, c)}"
                            value={props.value.clone()}
                            oninput={oninput}
                        />
                    }
                </div>
                <div class="sidebar__preview">
                    <span class="sidebar__preview-label">{"Preview"}</span>
                    <code class="sidebar__preview-code">{ if props.value.is_empty() {String::from("No relations yet")} else {props.value.clone()} }</code>
                </div>
                <Button
                    onclick={save_relation}
                >{"Save relation"}</Button>
                <div class="sidebar__analysis">
                    {
                        match &*props.digested_values {
                            Ok(relation) => html!{
                                <Analytics relation={relation.clone()}/>
                            },
                            Err(_)  => html! {
                                <p class="sidebar__subtitle">{"Waiting for valid relation"}</p>
                            }
                        }

                    }
                </div>
                {
                    match &*props.digested_values {
                        Ok(relation) => html!{
                            <div class="sidebar__explorer">
                                <Explorer
                                    relation={relation.clone()}
                                    object_selection={props.object_selection.clone()}
                                />
                            </div>
                        },
                        Err(_)  => html! {}
                    }

                }
            </div>
        </aside>
    }
}