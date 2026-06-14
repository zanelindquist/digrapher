use yew::prelude::*;

use crate::{
    components::{
        matrix_components::sidebar::Sidebar,
        navigation::topbar_layout::TopbarLayout
    }, render::matrix_render::canvas::MatrixCanvas, services::{matrix_services::types::{MatrixEquation, ObjectSelection, ParseError, Term}, objects::scalar::Scalar}
};


#[function_component(MatrixPage)]
pub fn matrix() -> Html {
    let object_selection: UseStateHandle<ObjectSelection> = use_state(|| ObjectSelection::default());
    let matrix_equation: UseStateHandle<Result<MatrixEquation, ParseError>> = use_state(|| Err(ParseError::new("No input")));
    let computed_answer: UseStateHandle<Option<Term>> = use_state(|| Some(Term::Scalar(Scalar::from_f64(1.1))));

    {
        let computed_answer = computed_answer.clone();
        let matrix_equation = matrix_equation.clone();
        use_effect_with(matrix_equation, move |_| {

        })
    }

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
                            answer={(*computed_answer).clone()}
                        />
                    }
                }
                Err(e) => {

                    if e.message.to_lowercase() == "no input" {
                        html! {
                            <div class="graph--error">
                                <img class="graph__no-input" src={format!("/assets/digraph_assets/no_input_variant.png")}/>
                                <code class="graph__error--heading">{ "No Input" }</code>     
                                <code class="graph__error">{ "Enter a formula to get started." }</code>                
                            </div>
                        }
                    } else {
                        html!{
                            <div class="graph--error">
                                <code class="graph__error--heading">{ e.message.to_string() }</code>     
                            </div>
                        }
                    }

                }
            }}

        </TopbarLayout>
    }
}