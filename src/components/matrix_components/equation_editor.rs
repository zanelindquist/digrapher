use yew::prelude::*;

use crate::services::matrix_services::types::MatrixEquasionResult;

#[derive(Properties, PartialEq)]
pub struct EquationEditorProps {
    pub matrix_equation: MatrixEquasionResult
}

#[function_component(EquationEditor)]
pub fn equation_editor(props: &EquationEditorProps) -> Html {
    html! {

    }
}