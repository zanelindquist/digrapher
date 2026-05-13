use yew::prelude::*;

use crate::components::misc::icon::Icon;

#[derive(Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: String,
    pub set_theme: Callback<String>,
}

// Get the theme as a string from the json file
const THEME_JSON: &str = include_str!("../../assets/themes/organic.json");

fn generate_css_vars(scheme: &str) -> String {
    // Objectify the string from theme json
    let value: serde_json::Value = serde_json::from_str(THEME_JSON).unwrap();

    // If the object is defined and has schemes, then we can process its data
    if let Some(obj) = value["schemes"][scheme].as_object() {
        let mut css = String::from(":root {\n");
        for (key, val) in obj {
            // If the value is a string
            if let Some(s) = val.as_str() {
                // Push it to the root variables
                css.push_str(&format!("  --{}: {};\n", key, s));
            }
        }
        css.push_str("}\n");
        // Return the root variables as a string
        return css;
    }
    // If the object is not defined, return nothing
    else {
        String::new()
    }
}

#[function_component(StyleProvider)]
pub fn style_provider() -> Html {
    // Get the theme context
    let theme_ctx = use_context::<ThemeContext>().expect("ThemeContext not found");
    // Process the selected theme
    let theme_css = generate_css_vars(&theme_ctx.theme);

    html! {
        // Inject the processed CSS variables
        <style>{ theme_css }</style>
    }
}

#[function_component(StyleToggle)]
pub fn style_toggle() -> Html {
    // Load the current theme context
    let theme_ctx = use_context::<ThemeContext>().expect("ThemeContext not found");
    let is_light = theme_ctx.theme == "light";

    // Register a callback for when the button is clicked
    let onclick = {
        let theme_ctx = theme_ctx.clone();
        Callback::from(move |_| {
            let next = if theme_ctx.theme == "light" {
                "dark"
            } else {
                "light"
            };
            // Set the global theme context
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
                    // Render the icon based on the selected mode
                    if is_light {
                        "sun"
                    } else {
                        "moon"
                    }
                }
                color={"secondary"}
                size={35}
            />
        </button>
    }
}