use yew::prelude::*;

use crate::components::analytics::Analytics;
use crate::components::explorer::Explorer;
use crate::logic::types::{DigestedValuesResult, ObjectSelection};

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub value: String,
    pub on_input: Callback<String>,
    pub digested_values: UseStateHandle<DigestedValuesResult>,
    pub object_selection: UseStateHandle<ObjectSelection>
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let oninput: Callback<InputEvent> = {
        let input_value = props.on_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            input_value.emit(input.value());
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
                    <label class="sidebar__label" for="graph-input">{"Enter graph"}</label>
                    <textarea 
                        id="graph-input"
                        class="sidebar__input"
                        rows="5"
                        placeholder="{(a, b), (b, c), (c, c)}"
                        value={props.value.clone()}
                        oninput={oninput}
                    />
                </div>
                <div class="sidebar__preview">
                    <span class="sidebar__preview-label">{"Preview"}</span>
                    <code class="sidebar__preview-code">{ if props.value.is_empty() {String::from("No relations yet")} else {props.value.clone()} }</code>
                </div>
                <div class="sidebar__analysis">
                    <label class="sidebar__label" for="graph-preview">{"Relation properties"}</label>
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
                                <label class="sidebar__label" for="graph-explorer">{"Relation explorer"}</label>
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