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
    pub fn new(data: Vec<Point>) -> Self {
        let current_plot = GraphicViewPlot::new(data);
        Self {
            plots: vec![current_plot],
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

    pub fn plot_scale(&mut self, zoom_in: bool) {
        let center = (self.x_min() + self.x_max()) / 2.0;
        let half = (self.x_max() - self.x_min()) / 2.0;
        let half = if zoom_in {
            half.mul(ZOOM_IN_COEFFICIENT)
        } else {
            half.mul(ZOOM_OUT_COEFFICIENT)
        };
        self.plots[self.current_plot_id].x_min = center - half;
        self.plots[self.current_plot_id].x_max = center + half;
    }

    pub fn plot_move(&mut self, left: bool) {
        if left {
            self.plots[self.current_plot_id].x_min -= 1.0;
            self.plots[self.current_plot_id].x_max -= 1.0;
        } else {
            self.plots[self.current_plot_id].x_min += 1.0;
            self.plots[self.current_plot_id].x_max += 1.0;
        }
    }
}
