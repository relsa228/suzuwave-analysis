use crate::{
    models::chart_view::{canvas_style::ChartViewStyle, chart::chart_model::ChartModel},
    shared::constants::chart_view::{ZOOM_IN_COEFFICIENT, ZOOM_OUT_COEFFICIENT},
};

pub struct ChartViewState {
    current_chart: ChartModel,
    canvas_style: ChartViewStyle,
}

impl ChartViewState {
    pub fn new() -> Self {
        Self {
            current_chart: ChartModel::default(),
            canvas_style: ChartViewStyle::new(),
        }
    }

    pub fn current_chart(&self) -> ChartModel {
        self.current_chart.clone()
    }

    pub fn canvas_style(&self) -> &ChartViewStyle {
        &self.canvas_style
    }

    pub fn x_min(&self) -> f64 {
        self.current_chart.x_min
    }

    pub fn x_max(&self) -> f64 {
        self.current_chart.x_max
    }

    pub fn y_min(&self) -> f64 {
        self.current_chart.y_min
    }

    pub fn y_max(&self) -> f64 {
        self.current_chart.y_max
    }

    pub fn chart_scale(&mut self, zoom_in: bool, zoom_multiplier: f64) {
        let x_center = (self.current_chart.x_min + self.current_chart.x_max) / 2.0;
        let x_half = (self.current_chart.x_max - self.current_chart.x_min) / 2.0;
        let x_half = if zoom_in {
            x_half * ZOOM_IN_COEFFICIENT * zoom_multiplier
        } else {
            x_half * ZOOM_OUT_COEFFICIENT / zoom_multiplier
        };
        self.current_chart.x_min = x_center - x_half;
        self.current_chart.x_max = x_center + x_half;

        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for point in &self.current_chart.data {
            if point.x() >= self.current_chart.x_min && point.x() <= self.current_chart.x_max {
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
            self.current_chart.y_min = y_min - padding;
            self.current_chart.y_max = y_max + padding;
        }
    }

    pub fn chart_move(&mut self, left: bool, points: f64) {
        if left {
            self.current_chart.x_min -= (self.current_chart.x_max - self.current_chart.x_min)
                / (self.canvas_style.canvas_steps()) as f64
                / 10.0
                * points;
            self.current_chart.x_max -= (self.current_chart.x_max - self.current_chart.x_min)
                / (self.canvas_style.canvas_steps()) as f64
                / 10.0
                * points;
        } else {
            self.current_chart.x_min += (self.current_chart.x_max - self.current_chart.x_min)
                / (self.canvas_style.canvas_steps()) as f64
                / 10.0
                * points;
            self.current_chart.x_max += (self.current_chart.x_max - self.current_chart.x_min)
                / (self.canvas_style.canvas_steps()) as f64
                / 10.0
                * points;
        }
    }

    pub fn set_current_chart(&mut self, chart: ChartModel) {
        self.current_chart = chart;
    }
}
