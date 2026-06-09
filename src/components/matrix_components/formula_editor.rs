use yew::prelude::*;

use crate::{components::misc::icon::Icon, services::matrix_services::types::{MatrixEquation, MatrixEquationResult, ObjectSelection, ParseError}};

#[derive(Properties, PartialEq)]
pub struct FormulaEditorProps {
    pub matrix_equation: UseStateHandle<MatrixEquationResult>,
    pub object_selection: UseStateHandle<ObjectSelection>
}

#[function_component(FormulaEditor)]
pub fn formula_editor(props: &FormulaEditorProps) -> Html {


    let oninput = {
        let matrix_equation = props.matrix_equation.clone();
        Callback::from(move |e: InputEvent | {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            if input.value().is_empty() {
                matrix_equation.set(Err(ParseError::new("No input")))
            } else {
                matrix_equation.set(Ok(MatrixEquation::from_text(input.value())));
            }
        })
    };

    html! {
        {
            match &*props.matrix_equation {
                Ok(equation) => html! {
                    <textarea 
                        id="graph-input"
                        class="sidebar__input"
                        rows="5"
                        placeholder="{}"
                        value={equation.raw_text.clone()}
                        oninput={oninput}
                    />
                },
                Err(_) => html!{
                    <textarea 
                        id="graph-input"
                        class="sidebar__input"
                        rows="5"
                        placeholder="No input yet"
                        oninput={oninput}
                    />
                }
            }
        }
    }
}