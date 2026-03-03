use ploot::prelude::*;

fn main() {
    let xs: Vec<f64> = (-30..=30).map(|i| i as f64 / 10.0).collect();
    let quadratic: Vec<f64> = xs.iter().map(|&x| x * x).collect();
    let sine: Vec<f64> = xs.iter().map(|&x| 8.0 * (x * 1.5).sin()).collect();
    let gaussian: Vec<f64> = xs.iter().map(|&x| 20.0 * (-x * x).exp()).collect();

    let q_plot = LinePlot::new(xs.iter().copied(), quadratic.iter().copied())
        .with_caption("x^2")
        .with_dash(DashType::Solid);

    let s_plot = LinePlot::new(xs.iter().copied(), sine.iter().copied())
        .with_caption("8*sin(1.5x)")
        .with_dash(DashType::Dash);

    let g_plot = ScatterPlot::new(xs.iter().copied(), gaussian.iter().copied())
        .with_caption("20*e^(-x^2)")
        .with_symbol(PointSymbol::Cross);

    let layout = Layout2D::new()
        .with_title("Lines, Points & Dash Styles")
        .with_x_label("x")
        .with_y_label("y")
        .with_y_grid(true)
        .with_plot(q_plot)
        .with_plot(s_plot)
        .with_plot(g_plot);

    Figure::new()
        .with_size(80, 24)
        .with_layout(layout)
        .show();
}
