use yew::prelude::*;

use crate::services::digraph_services::relation_storage::{get_stored_relations_from_json};
use crate::services::digraph_services::{types::StoredRelation};

#[derive(PartialEq, Properties)]
pub struct RelationBrowseProps {
    pub onselect: Callback<StoredRelation>
}

#[function_component(RelationBrowse)]
pub fn relation_library(props: &RelationBrowseProps) -> Html {
    // Get the relations from json
    let relations = get_stored_relations_from_json();

    html! {
        <div class="library">
            {match &relations {
                // If we successfully obtained the relations, then render a row for each loaded relation
                Ok(rels) => html! {
                    for rels.iter().map(|rel| html!{
                        <RelationBrowseRow
                            stored_relation={rel.clone()}
                            onselect={props.onselect.clone()}
                            ondelete={|_|{}}
                        />
                    })
                },
                // If loading from memory failed, show the user an error
                Err(e) => html! {
                    <code>{"Failed to load relations: "} {format!("{}", e).to_lowercase() + "."}</code>
                }
            }}
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct RelationBrowseRowProps {
    pub stored_relation: StoredRelation,
    pub onselect: Callback<StoredRelation>,
    pub ondelete: Callback<i32>
}
#[function_component(RelationBrowseRow)]
pub fn relation_library_row(props: &RelationBrowseRowProps) -> Html {
    // Define variables for cloning
    let rel = &props.stored_relation;
    let relation = props.stored_relation.clone();
    let onselect = props.onselect.clone();

    html! {
        <div class="relation-row">
            <div class="relation-row__left">
                <div class="relation-row__title">
                    { &rel.name }
                </div>
                <code class="relation-row__text"> { &rel.raw_text } </code>
            </div>
            <div class="relation-row__right">
                <div class="relation-row__date">
                    { &rel.date_saved }
                </div>
                <button
                    class="relation-row__button"
                    onclick={Callback::from(move |_| {
                        // Call the passed onselect callback with the stored relation data
                        onselect.emit(relation.clone())
                    })}
                >
                    { "Open" }
                </button>
            </div>
        </div>
    }
}