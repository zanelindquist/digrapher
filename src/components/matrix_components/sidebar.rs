use yew::prelude::*;

use crate::{components::{matrix_components::formula_editor::FormulaEditor, misc::icon::Icon}, services::matrix_services::types::{MatrixEquationResult, ObjectSelection}};

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub matrix_equation: UseStateHandle<MatrixEquationResult>,
    pub object_selection: UseStateHandle<ObjectSelection>
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let is_hidden = use_state(|| false);

    // Functions

    let on_toggle_hidden: Callback<MouseEvent> = {
        let is_hidden = is_hidden.clone();
        Callback::from(move |_: MouseEvent| {
            is_hidden.set(!*is_hidden);
        })
    };

    html! {
        <>
            <aside class={ if *is_hidden { "sidebar sidebar--hidden" } else { "sidebar" } }>
                <div class="sidebar__header">
                    <div class="sidebar__header__right">
                        <h2 class="sidebar__title">{"Matrix"}</h2>
                        <p class="sidebar__subtitle">{"Linear algebra calculator"}</p>
                    </div>
                    <button
                        class="sidebar__hide-toggle"
                        onclick={on_toggle_hidden.clone()}
                    >
                        <Icon
                            icon={if *is_hidden {"arrow-collapse-right"} else {"arrow-collapse-left"}}
                            size={24}
                            color="outline"
                        />
                    </button>
                </div>

                <div class="sidebar__content">
                    <div class="sidebar__input-group">
                        <div class="sidebar__input__container">
                            <label class="sidebar__label" for="graph-input">{"Enter formula"}</label>
                        </div>
                        <FormulaEditor
                            matrix_equation={props.matrix_equation.clone()}
                            object_selection={props.object_selection.clone()}
                        />
                    </div>
                </div>
            </aside>

            <div class={ if *is_hidden { "sidebar__hidden visible" } else { "sidebar__hidden" } }>
                <button
                    class="sidebar__hide-toggle"
                    onclick={on_toggle_hidden}
                >
                    <Icon
                        icon={if *is_hidden {"arrow-collapse-right"} else {"arrow-collapse-left"}}
                        size={24}
                        color="outline"
                    />
                </button>
            </div>
        </>
    }
}