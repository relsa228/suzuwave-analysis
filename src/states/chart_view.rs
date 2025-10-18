use crate::{
    models::chart_view::{canvas_style::ChartViewStyle, chart::chart_model::ChartModel},
    shared::{
        constants::chart_view::{ZOOM_IN_COEFFICIENT, ZOOM_OUT_COEFFICIENT},
        errors::chart_view::ChartViewError,
    },
};
use anyhow::Result;

pub struct ChartViewState {
    charts: Vec<ChartModel>,
    current_plot_id: usize,
    canvas_style: ChartViewStyle,
}

impl ChartViewState {
    pub fn new() -> Self {
        Self {
            charts: Vec::new(),
            current_plot_id: 0,
            canvas_style: ChartViewStyle::new(),
        }
    }

    pub fn current_dataset(&self) -> ChartModel {
        self.charts
            .get(self.current_plot_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn canvas_style(&self) -> &ChartViewStyle {
        &self.canvas_style
    }

    pub fn x_min(&self) -> f64 {
        self.charts
            .get(self.current_plot_id)
            .cloned()
            .unwrap_or_default()
            .x_min
    }

    pub fn x_max(&self) -> f64 {
        self.charts
            .get(self.current_plot_id)
            .cloned()
            .unwrap_or_default()
            .x_max
    }

    pub fn y_min(&self) -> f64 {
        self.charts
            .get(self.current_plot_id)
            .cloned()
            .unwrap_or_default()
            .y_min
    }

    pub fn y_max(&self) -> f64 {
        self.charts
            .get(self.current_plot_id)
            .cloned()
            .unwrap_or_default()
            .y_max
    }

    pub fn plot_scale(&mut self, zoom_in: bool, zoom_multiplier: f64) -> Result<()> {
        let plot = self
            .charts
            .get_mut(self.current_plot_id)
            .ok_or(ChartViewError::NoCurrentPlot)?;
        let x_center = (plot.x_min + plot.x_max) / 2.0;
        let x_half = (plot.x_max - plot.x_min) / 2.0;
        let x_half = if zoom_in {
            x_half * ZOOM_IN_COEFFICIENT * zoom_multiplier
        } else {
            x_half * ZOOM_OUT_COEFFICIENT / zoom_multiplier
        };
        plot.x_min = x_center - x_half;
        plot.x_max = x_center + x_half;

        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for point in &plot.data {
            if point.x() >= plot.x_min && point.x() <= plot.x_max {
                if point.y() < y_min {
                    y_min = point.y();
                }
                if point.y() > y_max {
                    y_max = point.y();
                }
            }
        }
        if y_min.is_finite() && y_max.is_finite() {
            let padding = (y_max - y_min) * 0.05;
            plot.y_min = y_min - padding;
            plot.y_max = y_max + padding;
        }
        Ok(())
    }

    pub fn plot_move(&mut self, left: bool, points: f64) {
        if let Some(plot) = self.charts.get_mut(self.current_plot_id) {
            if left {
                plot.x_min -=
                    (plot.x_max - plot.x_min) / (self.canvas_style.canvas_steps()) as f64 / 10.0
                        * points;
                plot.x_max -=
                    (plot.x_max - plot.x_min) / (self.canvas_style.canvas_steps()) as f64 / 10.0
                        * points;
            } else {
                plot.x_min +=
                    (plot.x_max - plot.x_min) / (self.canvas_style.canvas_steps()) as f64 / 10.0
                        * points;
                plot.x_max +=
                    (plot.x_max - plot.x_min) / (self.canvas_style.canvas_steps()) as f64 / 10.0
                        * points;
            }
        }
    }

    pub fn add_plot(&mut self, data: ChartModel) {
        self.charts.push(data);
        self.current_plot_id = self.charts.len() - 1;
    }

    pub fn delete_current_plot(&mut self) {
        self.charts.remove(self.current_plot_id);
        if self.current_plot_id > 0 {
            self.current_plot_id -= 1;
        }
    }

    pub fn change_current_plot(&mut self, id: u32) {
        self.current_plot_id = (id % self.charts.len() as u32) as usize;
    }
}
