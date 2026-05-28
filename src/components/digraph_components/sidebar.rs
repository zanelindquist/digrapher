use std::ops::Deref;
use yew::prelude::*;
use gloo_timers::callback::Timeout;

use crate::components::digraph_components::analytics::Analytics;
use crate::components::digraph_components::browse::RelationBrowse;
use crate::components::digraph_components::explorer::Explorer;
use crate::components::digraph_components::library::RelationLibrary;
use crate::components::misc::button::Button;
use crate::components::misc::icon::Icon;
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
    let input_display_mode = use_state(|| 0);
    let display_check = use_state(|| false);
    let failed_delete_flash = use_state(|| false);

    // When the user puts in text
    let oninput: Callback<InputEvent> = {
        let input_value = props.on_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            input_value.emit(input.value());
        })
    };

    // When the button to switch between selection modes is pressed
    let on_toggle_library: Callback<MouseEvent> = {
        let input_display_mode = input_display_mode.clone();
        Callback::from(move |_: MouseEvent| {
            // Circular scroll the display mode
            input_display_mode.set((*input_display_mode + 1) % 3);
        })
    };
    // Whent the suers presses save relation
    let save_relation: Callback<MouseEvent> = {
        let dv = props.digested_values.clone();
        let display_check = display_check.clone();
        let failed_delete_flash = failed_delete_flash.clone();
        Callback::from(move |_: MouseEvent| {
            if let Ok(relation) = dv.deref() {
                // If we successfully saved the relation, flash a checkmark
                if let Ok(_) = store_new_relation(relation) {
                    display_check.set(true);
                    let dc = display_check.clone();
                    let timeout = Timeout::new(1_000, move || {
                        dc.set(false);
                    });
                    timeout.forget();
                }
                // Otherwise, flash an X
                else {
                    failed_delete_flash.set(true);
                    let f = failed_delete_flash.clone();
                    let timeout = Timeout::new(1_000, move || {
                        f.set(false);
                    });
                    timeout.forget();
                }
            }
        })
    };
    // Used for piping a loaded saved relation into the input box to be loaded and parsed
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
                        >
                        // Show the button for the next mode to switch to
                        {match *input_display_mode {
                            2 => {"Create"},
                            0 => {"Library"},
                            1 => {"Browse"},
                            _ => {""}
                        }}
                        </Button>
                    </div>
                    // Render different selection modes
                    {match *input_display_mode {
                        0 => html! {
                            <textarea 
                                id="graph-input"
                                class="sidebar__input"
                                rows="5"
                                placeholder="{(a, b), (b, c), (c, c)}"
                                value={props.value.clone()}
                                oninput={oninput}
                            />
                        },
                        1 => html! {
                            <RelationLibrary
                                onselect={load_saved_relation}
                            />},
                        2 => html! {
                            <RelationBrowse
                                onselect={load_saved_relation}
                            />
                        },
                        _ => html! {}
                    }}
                </div>
                <div class="sidebar__input__save">
                    // Show the save relation button
                    <Button
                        onclick={save_relation}
                        // Disable the button if we are on saved relations
                        // OR if we don't have a valid relation being displayed
                        disabled={
                            *input_display_mode == 1
                            || !props.digested_values.is_ok()
                        }
                    >{"Save relation"}</Button>
                    // Display the icon to flash a success or failure
                    if *display_check || *failed_delete_flash {
                        <Icon
                            // Set its values based on it being a success or failure
                            icon={if *failed_delete_flash {"close"} else {"check"}}
                            color={if *failed_delete_flash {"error"} else {"onSecondaryContainer"}}
                            class={if *failed_delete_flash {"relation-row__right__trash--error"} else {"sidebar__input__save__check"}}
                        />
                    }
                </div>
                <div class="sidebar__analysis">
                    {
                        match &*props.digested_values {
                            Ok(relation) => html!{
                                <Analytics relation={relation.clone()}/>
                            },
                            Err(e)  => html! {
                                <div class="sidebar__preview">
                                    <code>{ format!("Parsing error: {}", e.message)}</code>                
                                </div>
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