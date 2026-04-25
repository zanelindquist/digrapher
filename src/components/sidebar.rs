use yew::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub value: String,
    pub on_input: Callback<String>
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let oninput: Callback<InputEvent> = {
        let input_value = props.on_input.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let value = target.dyn_into::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();
            input_value.emit(value);
        })
    };
    
    html! {
        <aside class="sidebar">
            <div class="sidebar__header">
                <h2 class="sidebar__title">{"Digraph"}</h2>
                <p class="sidebar__subtitle">{"Graph visualization"}</p>
            </div>
            
            <div class="sidebar__content">
                <div class="sidebar__input-group">
                    <label class="sidebar__label" for="graph-input">{"Enter graph"}</label>
                    <input 
                        id="graph-input"
                        class="sidebar__input"
                        type="text" 
                        placeholder="{(a, b), (b, c), (c, c)}"
                        value={props.value.clone()}
                        oninput={oninput}
                    />
                </div>
                
                if !props.value.is_empty() {
                    <div class="sidebar__preview">
                        <span class="sidebar__preview-label">{"Preview"}</span>
                        <code class="sidebar__preview-code">{props.value.clone()}</code>
                    </div>
                }
            </div>
            
            <div class="sidebar__footer">
                <button class="sidebar__button">{"Generate"}</button>
            </div>
        </aside>
    }
}