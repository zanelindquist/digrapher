use std::ops::Deref;
use yew::prelude::*;
use gloo_timers::callback::Timeout;

use crate::services::digraph_services::relation_storage::remove_relation;
use crate::services::digraph_services::{relation_storage::get_stored_relations, types::StoredRelation};
use crate::components::misc::icon::Icon;

#[derive(PartialEq, Properties)]
pub struct RelationEditProps {
    pub onselect: Callback<StoredRelation>
}

#[function_component(RelationEdit)]
pub fn relation_edit(props: &RelationEditProps) -> Html {
    html! {
        <div class="edit">
            
        </div>
    }
}