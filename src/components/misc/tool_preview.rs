use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ToolData {
    pub name: String,
    pub description: String,
    pub published_date: String,
    pub image_path: String,
    pub url: String
}

#[derive(Properties, PartialEq)]
pub struct ToolPreviewProps {
    #[prop_or_default]
    pub class: String,
    pub tool_data: ToolData
}

#[function_component(ToolPreview)]
pub fn logo(props: &ToolPreviewProps) -> Html {

    html!{
        <div class={classes!("tool-preview", props.class.clone())}>
            <img class="tool-preview__img" src={format!("../../assets/tool_preview_images/{}", props.tool_data.image_path)}/>
            <code class="tool-preview__name">{props.tool_data.name.clone()}</code>
            <code class="tool-preview__description">{props.tool_data.description.clone()}</code>
            <code class="tool-preview__date">{format!("{}", props.tool_data.published_date)}</code>
        </div>
    }
}