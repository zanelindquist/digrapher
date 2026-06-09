use yew::prelude::*;

use crate::{components::{matrix_components::sidebar::Sidebar, navigation::topbar_layout::TopbarLayout}, services::matrix_services::types::{MatrixEquation, MatrixEquationResult, ObjectSelection, ParseError}};


#[function_component(MatrixPage)]
pub fn matrix() -> Html {
    let object_selection: UseStateHandle<ObjectSelection> = use_state(|| ObjectSelection::default());
    let matrix_equation: UseStateHandle<Result<MatrixEquation, ParseError>> = use_state(|| Err(ParseError::new("No input")));

    let update_equation = {
        let matrix_equation = matrix_equation.clone();
        Callback::from(move |raw: String| {
            matrix_equation.set(Ok(MatrixEquation::from_text(raw)));
        })
    };

    html! {
        <TopbarLayout class="app">
            <Sidebar
                object_selection={object_selection}
                matrix_equation={matrix_equation}
            />
        </TopbarLayout>
    }
}