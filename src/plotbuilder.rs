use conrod;
use std::marker::{Send, Sync};

pub type PlotFn = &'static (Fn(f64) -> f64 + Sync);
pub type AnimFn = &'static (Fn(f64, f64) -> f64 + Sync);

#[derive(Clone)]
pub enum PlotVals2D {
    Xy(Vec<(f64, f64)>),
    XyColor(conrod::color::Color, Vec<f64>, Vec<f64>),
    Fun(PlotFn),
    FunColor(conrod::color::Color, PlotFn),
    AnimFun(AnimFn),
    AnimFunColor(conrod::color::Color, AnimFn),
    Bars(Vec<f64>),
    BarsColor(conrod::color::Color, Vec<f64>),
}

#[derive(Clone)]
pub struct PlotBuilder2D {
    pub pvs: Vec<PlotVals2D>,
    pub min_x: Option<f64>,
    pub max_x: Option<f64>,
    pub min_y: Option<f64>,
    pub max_y: Option<f64>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub title: Option<String>,
    pub y_axis: bool,
    pub y_gridlines: bool,
    pub x_axis: bool,
    pub x_gridlines: bool,
}

impl PlotBuilder2D {
    pub fn simple_xy(xy: Vec<(f64, f64)>) -> PlotBuilder2D {
        PlotBuilder2D {
            pvs: vec![PlotVals2D::Xy(xy)],
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
            x_label: None,
            y_label: None,
            title: None,
            y_axis: true,
            y_gridlines: true,
            x_axis: true,
            x_gridlines: true,
        }
    }

    pub fn simple_fun(plotfn: PlotFn) -> PlotBuilder2D {
        PlotBuilder2D {
            pvs: vec![PlotVals2D::Fun(plotfn)],
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
            x_label: None,
            y_label: None,
            title: None,
            y_axis: true,
            y_gridlines: true,
            x_axis: true,
            x_gridlines: true,
        }
    }
}