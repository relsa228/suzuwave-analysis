use std::{cell::RefCell, rc::Rc};

use crate::{
    models::chart_view::{canvas_style::ChartViewStyle, chart::chart_model::ChartModel},
    shared::constants::chart_view::{ZOOM_IN_COEFFICIENT, ZOOM_OUT_COEFFICIENT},
};

pub struct ChartViewState {
    current_chart: Option<Rc<RefCell<ChartModel>>>,
    canvas_style: ChartViewStyle,
}

impl ChartViewState {
    pub fn new() -> Self {
        Self {
            current_chart: None,
            canvas_style: ChartViewStyle::new(),
        }
    }

    pub fn current_chart(&self) -> Option<Rc<RefCell<ChartModel>>> {
        self.current_chart.clone()
    }

    pub fn canvas_style(&self) -> &ChartViewStyle {
        &self.canvas_style
    }

    pub fn x_min(&self) -> f64 {
        if let Some(chart) = &self.current_chart {
            chart.borrow().x_min
        } else {
            0.0
        }
    }

    pub fn x_max(&self) -> f64 {
        if let Some(chart) = &self.current_chart {
            chart.borrow().x_max
        } else {
            0.0
        }
    }

    pub fn y_min(&self) -> f64 {
        if let Some(chart) = &self.current_chart {
            chart.borrow().y_min
        } else {
            0.0
        }
    }

    pub fn y_max(&self) -> f64 {
        if let Some(chart) = &self.current_chart {
            chart.borrow().y_max
        } else {
            0.0
        }
    }

    pub fn chart_scale(&mut self, zoom_in: bool, zoom_multiplier: f64) {
        let current_chart = if let Some(chart) = &self.current_chart {
            chart
        } else {
            return;
        };

        let x_center = (current_chart.borrow().x_min + current_chart.borrow().x_max) / 2.0;
        let x_half = (current_chart.borrow().x_max - current_chart.borrow().x_min) / 2.0;
        let x_half = if zoom_in {
            x_half * ZOOM_IN_COEFFICIENT * zoom_multiplier
        } else {
            x_half * ZOOM_OUT_COEFFICIENT / zoom_multiplier
        };
        current_chart.borrow_mut().x_min = x_center - x_half;
        current_chart.borrow_mut().x_max = x_center + x_half;

        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for point in &current_chart.borrow().data {
            if point.x() >= current_chart.borrow().x_min
                && point.x() <= current_chart.borrow().x_max
            {
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
            current_chart.borrow_mut().y_min = y_min - padding;
            current_chart.borrow_mut().y_max = y_max + padding;
        }
    }

    pub fn chart_move(&mut self, left: bool, points: f64) {
        let current_chart = if let Some(chart) = &self.current_chart {
            chart
        } else {
            return;
        };

        let x_delta = current_chart.borrow().x_max - current_chart.borrow().x_min;
        if left {
            current_chart.borrow_mut().x_min -=
                x_delta / (self.canvas_style.canvas_steps()) as f64 / 10.0 * points;
            current_chart.borrow_mut().x_max -=
                x_delta / (self.canvas_style.canvas_steps()) as f64 / 10.0 * points;
        } else {
            current_chart.borrow_mut().x_min +=
                x_delta / (self.canvas_style.canvas_steps()) as f64 / 10.0 * points;
            current_chart.borrow_mut().x_max +=
                x_delta / (self.canvas_style.canvas_steps()) as f64 / 10.0 * points;
        }
    }

    pub fn set_current_chart(&mut self, chart: Option<Rc<RefCell<ChartModel>>>) {
        self.current_chart = chart;
    }
}
