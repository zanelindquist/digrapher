use yew::prelude::*;

use crate::logic::digraph_logic::types::Relation;

#[derive(Properties, PartialEq)]
pub struct AnalyticsProps {
    pub relation: Relation
}

#[function_component(Analytics)]
pub fn analytics(props: &AnalyticsProps) -> Html {
    let properties = props.relation.properties.clone();

    html! {
        <div class="analytics">
            <label class="sidebar__label" for="graph-preview">{"Relation properties"}</label>
            <div class="sidebar__preview">
                <code class="sidebar__preview-code">
                    {format!("Antisymmetric: {}", properties.antisymmetric.to_string())} <br/>
                    {format!("Symmetric: {}", properties.symmetric.to_string())} <br/>
                    {format!("Relfexive: {}", properties.reflexive.to_string())} <br/>
                    {format!("Transitive: {}", properties.transitive.to_string())} <br/> <br/>
                    {format!("Partial Ordering: {}", (properties.reflexive && properties.antisymmetric && properties.transitive).to_string())} <br/>
                    {format!("Equivalence Relation: {}", (properties.reflexive && properties.symmetric && properties.transitive).to_string())} <br/>
                </code>
                
            </div>
        </div>
    }
}