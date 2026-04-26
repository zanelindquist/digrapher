#[derive(Clone, Copy, PartialEq)]
pub struct DotStyle {
    pub radius: f32,
    pub fill: &'static str,
    pub stroke: &'static str,
    pub stroke_width: f32,
}

impl Default for DotStyle {
    fn default() -> Self {
        Self {
            radius: 6.0,
            fill: "var(--primary)",
            stroke: "var(--outline)",
            stroke_width: 2.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct EdgeStyle {
    pub stroke: &'static str,
    pub stroke_width: f32,
}

impl Default for EdgeStyle {
    fn default() -> Self {
        Self {
            stroke: "var(--outline)",
            stroke_width: 2.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct RenderStyles {
    pub dot: DotStyle,
    pub edge: EdgeStyle,
}
impl Default for RenderStyles {
    fn default() -> Self {
        Self {
            dot: DotStyle::default(),
            edge: EdgeStyle::default(),
        }
    }
}