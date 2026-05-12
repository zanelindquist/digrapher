use gloo_console::log;
use yew::prelude::*;

use crate::components::navigation::topbar_layout::TopbarLayout;
use crate::components::misc::{
    logo::Logo,
    tool_preview::{ToolPreview, ToolData}
};

const TOOLS_JSON: &str = include_str!("../assets/tools.json");

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let tools: Vec<ToolData> = serde_json::from_str(TOOLS_JSON).unwrap();

    html! {
        <TopbarLayout>
            <div class="home">
                <Logo class="home__logo" hide_logo={true} size={60}/>
                <code class="home__description">
                    {"Visualize complex math concepts in an ultra-responsive, dynamic web environment. Explore tools to bring college-level math to life."}
                </code>
                <div class="home__tools">
                    {for tools.iter().map(|tool| html! {
                        <ToolPreview class="home__tool" tool_data={tool.clone()}/>
                    })}
                </div>
            </div>
        </TopbarLayout>
    }
}