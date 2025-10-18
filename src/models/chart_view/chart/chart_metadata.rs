use ratatui::widgets::GraphType;

use crate::shared::constants::chart::CHART_METADATA_DEFAULT_DESCRIPTION;

#[derive(Debug, Clone, Default)]
pub struct ChartMetadata {
    pub title: String,
    pub description: String,
    pub chart_display_type: GraphType,
}

impl ChartMetadata {
    pub fn new(title: &str, description: Option<&str>, chart_display_type: GraphType) -> Self {
        Self {
            title: String::from(title),
            description: String::from(description.unwrap_or(CHART_METADATA_DEFAULT_DESCRIPTION)),
            chart_display_type,
        }
    }
}
