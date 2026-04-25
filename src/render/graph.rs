use yew::prelude::*;
use crate::logic::digest_values::{digest_values, Relation};

#[derive(Properties, PartialEq)]
pub struct GraphProps {
    pub input: String,
}

#[function_component(Graph)]
pub fn graph(props: &GraphProps) -> Html{

    let digested_values: Relation = digest_values(format!("{}", props.input));

    html!{
        <div class="graph">
            <p>{format!("{:?}", digested_values.values)}</p>
        </div>
    }
}