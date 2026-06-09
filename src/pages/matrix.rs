use yew::prelude::*;

use crate::{
    components::{
        matrix_components::sidebar::Sidebar,
        navigation::topbar_layout::TopbarLayout
    },
    services::matrix_services::types::{MatrixEquation, ObjectSelection, ParseError},
    render::matrix_render::canvas::MatrixCanvas
};


#[function_component(MatrixPage)]
pub fn matrix() -> Html {
    let object_selection: UseStateHandle<ObjectSelection> = use_state(|| ObjectSelection::default());
    let matrix_equation: UseStateHandle<Result<MatrixEquation, ParseError>> = use_state(|| Err(ParseError::new("No input")));

    html! {
        <TopbarLayout class="app">
            <Sidebar
                object_selection={object_selection.clone()}
                matrix_equation={matrix_equation.clone()}
            />
            {match  &*matrix_equation {
                Ok(equation) => {
                    html! {
                        <MatrixCanvas
                            object_selection={object_selection}
                            matrix_equation={equation.clone()}
                        />
                    }
                }
                Err(e) => {
                    html! {
                        <div class="graph--error">
                            <img class="graph__no-input" src={format!("/assets/digraph_assets/no_input_variant.png")}/>
                            <code class="graph__error--heading">{ "No Input" }</code>     
                            <code class="graph__error">{ "Enter a formula to get started." }</code>                
                        </div>
                    }
                }
            }}

        </TopbarLayout>
    }
}