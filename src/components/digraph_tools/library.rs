use std::ops::Deref;

use yew::prelude::*;
use crate::services::digraph_services::relation_storage::remove_relation;
use crate::services::digraph_services::{relation_storage::get_stored_relations, types::StoredRelation};
use crate::components::misc::icon::Icon;

#[derive(PartialEq, Properties)]
pub struct RelationLibraryProps {
    pub onselect: Callback<StoredRelation>
}

#[function_component(RelationLibrary)]
pub fn relation_library(props: &RelationLibraryProps) -> Html {
    let relations = use_state(|| get_stored_relations());

    let on_delete: Callback<i32> = {
        let relations = relations.clone();
        Callback::from(move |_| {
            relations.set(get_stored_relations());
        })
    };

    html! {
        <div class="library">
            {match &relations.deref() {
                Ok(rels) => html! {
                    for rels.iter().map(|rel| html!{
                        <RelationLibraryRow
                            stored_relation={rel.clone()}
                            onselect={props.onselect.clone()}
                            ondelete={on_delete.clone()}
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
    pub stored_relation: StoredRelation,
    pub onselect: Callback<StoredRelation>,
    pub ondelete: Callback<i32>
}
#[function_component(RelationLibraryRow)]
pub fn relation_library_row(props: &RelationLibraryRowProps) -> Html {
    let rel = &props.stored_relation;
    let relation = props.stored_relation.clone();
    let id = props.stored_relation.id.clone();
    let onselect = props.onselect.clone();
    let ondelete = props.ondelete.clone();

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
                        onselect.emit(relation.clone())
                    })}
                >
                    { "Open" }
                </button>
                <button
                    onclick={Callback::from(move |_| {
                        remove_relation(id.clone());
                        ondelete.emit(0);
                    })}
                >
                    <Icon
                        icon="trashcan"
                        color="onPrimaryContainer"
                        class="relation-row__right__trash"
                    />
                </button>
            </div>
        </div>
    }
}