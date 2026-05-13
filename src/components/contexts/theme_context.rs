use yew::prelude::*;

use crate::components::misc::icon::Icon;

#[derive(PartialEq)]
pub enum ThemeTypes {LIGHT, DARK}

#[derive(Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: String,
    pub set_theme: Callback<String>,
}

const THEME_JSON: &str = include_str!("../../assets/themes/organic.json");

fn generate_css_vars(scheme: &str) -> String {
    let value: serde_json::Value = serde_json::from_str(THEME_JSON).unwrap();

    if let Some(obj) = value["schemes"][scheme].as_object() {
        let mut css = String::from(":root {\n");
        for (key, val) in obj {
            if let Some(s) = val.as_str() {
                css.push_str(&format!("  --{}: {};\n", key, s));
            }
        }
        css.push_str("}\n");
        return css;
    }

    String::new()
}

#[function_component(StyleProvider)]
pub fn style_provider() -> Html {
    let theme_ctx = use_context::<ThemeContext>().expect("ThemeContext not found");
    let theme_css = generate_css_vars(&theme_ctx.theme);

    html! {
        <style>{ theme_css }</style>
    }
}

#[function_component(StyleToggle)]
pub fn style_toggle() -> Html {
    let theme_ctx = use_context::<ThemeContext>().expect("ThemeContext not found");
    let is_light = theme_ctx.theme == "light";

    let onclick = {
        let theme_ctx = theme_ctx.clone();
        Callback::from(move |_| {
            let next = if theme_ctx.theme == "light" {
                "dark"
            } else {
                "light"
            };
            theme_ctx.set_theme.emit(next.to_string());
        })
    };

    html! {
        <button
            class="style-toggle"
            {onclick}
        >
            <Icon
                icon={
                    if is_light {
                        "sun"
                    } else {
                        "moon"
                    }
                }
                color={"secondary"}
                size={40}
            />
        </button>
    }
}