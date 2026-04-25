use serde_json::Value;
use std::{collections::HashMap};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, console};

pub type ThemeValues = HashMap<String, String>;

fn flatten(prefix: String, value: Value, out: &mut ThemeValues) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let new_key = if prefix.is_empty() {
                    k
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten(new_key, v, out);
            }
        }
        Value::String(s) => {
            out.insert(prefix, s);
        }
        _=> {}
    }
}

pub fn parse_theme(value: Value) -> ThemeValues {
    let mut map: ThemeValues = HashMap::new();
    
    // Navigate to schemes.light (or make configurable)
    if let Value::Object(root) = value {
        if let Some(Value::Object(schemes)) = root.get("schemes") {
            if let Some(Value::Object(light)) = schemes.get("light") {
                flatten(String::from("color"), Value::Object(light.clone()), &mut map);
            }
        }
    }

    map
}

pub fn attach_theme(json_str: &str) {
    let value: Value = serde_json::from_str(json_str).unwrap();
    let theme = parse_theme(value);

    console::log_1(&format!("Parsed {} theme values", theme.len()).into());

    let document = window().unwrap().document().unwrap();

    let root = document
        .document_element()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    for (key, value) in theme {
        let css_var = format!("--{}", key.replace(".", "-"));
        console::log_1(&format!("Setting {}: {}", css_var, value).into());
        root.style().set_property(&css_var, &value).unwrap();
    }
}