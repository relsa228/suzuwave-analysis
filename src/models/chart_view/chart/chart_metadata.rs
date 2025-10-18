use crate::models::chart_view::chart::chart_transform::ChartTransform;
use ratatui::widgets::GraphType;

#[derive(Debug, Clone, Default)]
pub struct ChartMetadata {
    pub title: String,
    pub transform: ChartTransform,
    pub chart_display_type: GraphType,
}

impl ChartMetadata {
    pub fn new(
        title: &str,
        transform: Option<ChartTransform>,
        chart_display_type: GraphType,
    ) -> Self {
        Self {
            title: String::from(title),
            transform: transform.unwrap_or_default(),
            chart_display_type,
        }
    }

    pub fn description(&self) -> String {
        self.transform.to_string()
    }
}
