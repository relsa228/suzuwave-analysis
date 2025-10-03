use std::ops::Mul;

use crate::models::graphic_view::{
    canvas_style::GraphicViewStyle, plot::GraphicViewPlot, point::Point,
};

const ZOOM_IN_COEFFICIENT: f64 = 1.2;
const ZOOM_OUT_COEFFICIENT: f64 = 0.8;

pub struct GraphicViewState {
    plots: Vec<GraphicViewPlot>,
    current_plot_id: usize,
    canvas_style: GraphicViewStyle,
}

impl GraphicViewState {
    pub fn new() -> Self {
        Self {
            plots: vec![],
            current_plot_id: 0,
            canvas_style: GraphicViewStyle::new(),
        }
    }

    pub fn current_dataset(&self) -> GraphicViewPlot {
        self.plots[self.current_plot_id].clone()
    }

    pub fn canvas_style(&self) -> &GraphicViewStyle {
        &self.canvas_style
    }

    pub fn x_min(&self) -> f64 {
        self.plots[self.current_plot_id].x_min
    }

    pub fn x_max(&self) -> f64 {
        self.plots[self.current_plot_id].x_max
    }

    pub fn y_min(&self) -> f64 {
        self.plots[self.current_plot_id].y_min
    }

    pub fn y_max(&self) -> f64 {
        self.plots[self.current_plot_id].y_max
    }

    pub fn plot_scale(&mut self, zoom_in: bool, zoom_multiplier: f64) {
        let center = (self.x_min() + self.x_max()) / 2.0;
        let half = (self.x_max() - self.x_min()) / 2.0;
        let half = if zoom_in {
            half.mul(ZOOM_IN_COEFFICIENT * zoom_multiplier)
        } else {
            half.mul(ZOOM_OUT_COEFFICIENT * zoom_multiplier)
        };
        self.plots[self.current_plot_id].x_min = center - half;
        self.plots[self.current_plot_id].x_max = center + half;
    }

    pub fn plot_move(&mut self, left: bool, points: f64) {
        if left {
            self.plots[self.current_plot_id].x_min -= points;
            self.plots[self.current_plot_id].x_max -= points;
        } else {
            self.plots[self.current_plot_id].x_min += points;
            self.plots[self.current_plot_id].x_max += points;
        }
    }

    pub fn add_plot(&mut self, data: Vec<Point>) {
        let current_plot = GraphicViewPlot::new(data);
        self.plots.push(current_plot);
        self.current_plot_id = self.plots.len() - 1;
    }

    pub fn delete_current_plot(&mut self) {
        self.plots.remove(self.current_plot_id);
        if self.current_plot_id > 0 {
            self.current_plot_id -= 1;
        }
    }

    pub fn change_current_plot(&mut self, id: u32) {
        self.current_plot_id = (id % self.plots.len() as u32) as usize;
    }
}
