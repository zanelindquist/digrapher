use std::ops::Deref;

use yew::prelude::*;
use crate::services::digraph_services::{relation_storage::get_stored_relations, types::StoredRelation};

#[derive(PartialEq, Properties)]
pub struct RelationLibraryProps {
    #[prop_or_default]
    onselect: Callback<StoredRelation>
}

#[function_component(RelationLibrary)]
pub fn relation_library() -> Html {
    let relations = get_stored_relations();

    html! {
        <div class="library">
            {match &relations {
                Ok(rels) => html! {
                    for rels.iter().map(|rel| html!{
                        <RelationLibraryRow
                            stored_relation={rel.clone()}
                        />
                    })
                },
                Err(e) => html! {
                    <code>{"Failed to load relations: "} {format!("{}", e).to_lowercase() + "."}</code>
                }
            }}
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct RelationLibraryRowProps {
    stored_relation: StoredRelation
}
#[function_component(RelationLibraryRow)]
pub fn relation_library_row(props: &RelationLibraryRowProps) -> Html {
    html! {

    }
}