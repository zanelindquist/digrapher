#[derive(Clone, Copy, PartialEq)]
pub struct PointStyle {
    pub radius: f32,
    pub fill: &'static str,
    pub stroke: &'static str,
    pub stroke_width: f32,
    pub highlighted_stroke: &'static str,
    pub hovered_stroke: &'static str,
    pub point_connection_origin_highlight: &'static str,
    pub label_displacement: f32
}
impl Default for PointStyle {
    fn default() -> Self {
        Self {
            radius: 6.0,
            fill: "var(--primary)",
            stroke: "var(--outline)",
            stroke_width: 2.0,
            highlighted_stroke: "var(--inversePrimary)",
            hovered_stroke: "var(--error)",
            point_connection_origin_highlight: "#FFAE42",
            label_displacement: 8.0
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct EdgeStyle {
    pub stroke: &'static str,
    pub stroke_width: f32,
    pub highlighted_stroke: &'static str
}

impl Default for EdgeStyle {
    fn default() -> Self {
        Self {
            stroke: "var(--outline)",
            stroke_width: 2.0,
            highlighted_stroke: "var(--inversePrimary)"
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct FontStyle {
    pub size: f32,
    pub fill: &'static str,
    pub family: &'static str,
    pub stroke: &'static str,
    pub stroke_width: f32
}
impl Default for FontStyle {
    fn default() -> Self {
        Self {
            size: 20.0,
            fill: "var(--primary)",
            family: "",
            stroke: "var(--background)",
            stroke_width: 0.0
        }        
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct MatrixStyle {
    pub font: FontStyle,
    pub legend_font: FontStyle,
    pub cell_size: i32,
    pub stroke: &'static str,
    pub stroke_width: i32,
    pub selected_text_color: &'static str,
    pub selected_outline_color: &'static str,
    pub selected_stroke_width: i32,
}
impl Default for MatrixStyle {
    fn default() -> Self {
        let lf = FontStyle { size: 16.0, fill: "var(--outlineVariant)", family: "", stroke: "", stroke_width: 0.0 };
        Self {
            font: FontStyle::default(),
            legend_font: lf,
            cell_size: 40,
            stroke: "var(--outlineVariant)",
            stroke_width: 2,
            selected_text_color: "var(--onBackground)",
            selected_outline_color: "var(--onPrimaryContainer)",
            selected_stroke_width: 1
        }        
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct RenderStyles {
    pub point: PointStyle,
    pub edge: EdgeStyle,
    pub font: FontStyle,
    pub matrix: MatrixStyle
}
impl Default for RenderStyles {
    fn default() -> Self {
        Self {
            point: PointStyle::default(),
            edge: EdgeStyle::default(),
            font: FontStyle::default(),
            matrix: MatrixStyle::default()
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct GraphTheoryLayoutSettings {
    pub tree_settings: GraphTheoryTreeLayoutSettings,
    pub chain_settings: GraphTheoryChainLayoutSettings,
    pub layered_settings: GraphTheoryLayeredLayoutSettings,
    pub network_settings: GraphTheoryNetworkLayoutSettings
}
impl Default for GraphTheoryLayoutSettings {
    fn default() -> Self {
        Self {
            tree_settings: GraphTheoryTreeLayoutSettings::default(),
            chain_settings: GraphTheoryChainLayoutSettings::default(),
            layered_settings: GraphTheoryLayeredLayoutSettings::default(),
            network_settings: GraphTheoryNetworkLayoutSettings::default()
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct GraphTheoryTreeLayoutSettings {
    pub layer_height_l: f32,
}
impl Default for GraphTheoryTreeLayoutSettings {
    fn default() -> Self {
        Self {
            layer_height_l: 0.5
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
pub struct GraphTheoryChainLayoutSettings {
    pub point_seperation_l: f32,
}
impl Default for GraphTheoryChainLayoutSettings {
    fn default() -> Self {
        Self {
            point_seperation_l: 0.5
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
pub struct GraphTheoryLayeredLayoutSettings {
    pub bookend_taper_scale_l: f32,
}
impl Default for GraphTheoryLayeredLayoutSettings {
    fn default() -> Self {
        Self {
            bookend_taper_scale_l: 0.65
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
pub struct GraphTheoryNetworkLayoutSettings {
    pub num_outer_nodes: i32,
    pub max_width_l: f32,
    pub max_height_l: f32,
    pub max_fr_iterations: i32,
    pub repulsion_multiplier: f32,
    pub attraction_multiplier: f32,
}
impl Default for GraphTheoryNetworkLayoutSettings {
    fn default() -> Self {
        Self {
            num_outer_nodes: 4,
            max_width_l: 2.0,
            max_height_l: 2.0,
            max_fr_iterations: 60,
            repulsion_multiplier: 1.0,
            attraction_multiplier: 1.0
        }
    }
}