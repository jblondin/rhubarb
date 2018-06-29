
#[derive(Serialize, Clone, Debug)]
pub struct Script {
    pub url: String,
}

pub fn default_scripts() -> Vec<Script> {
    vec![
        "https://cdn.plot.ly/plotly-1.38.1.min.js",
        "app_bundle/bundle.js"
    ].iter().map(|s| Script { url: s.to_string() }).collect()
}

#[derive(Serialize, Clone, Debug)]
pub struct Style {
    pub url: String,
}

pub fn default_styles() -> Vec<Style> {
    // vec![
    //     "app_bundle/bundle.css"
    // ].iter().map(|s| Style { url: s.to_string() }).collect()
    vec![]
}
