use ploot::prelude::*;

fn main() {
    let xs: Vec<f64> = (-30..=30).map(|i| i as f64 / 10.0).collect();
    let quadratic: Vec<f64> = xs.iter().map(|&x| x * x).collect();
    let cubic: Vec<f64> = xs.iter().map(|&x| x * x * x).collect();
    let sine: Vec<f64> = xs.iter().map(|&x| 8.0 * (x * 1.5).sin()).collect();
    let gaussian: Vec<f64> = xs.iter().map(|&x| 20.0 * (-x * x).exp()).collect();

    let layout = Layout2D::new()
        .with_title("x^2  vs  x^3  vs  8*sin(1.5x)  vs  20*e^(-x^2)")
        .with_x_label("x")
        .with_y_label("y")
        .with_plot(LinePlot::new(xs.iter().copied(), quadratic.iter().copied()).with_caption("x^2"))
        .with_plot(LinePlot::new(xs.iter().copied(), cubic.iter().copied()).with_caption("x^3"))
        .with_plot(LinePlot::new(xs.iter().copied(), sine.iter().copied()).with_caption("8*sin(1.5x)"))
        .with_plot(LinePlot::new(xs.iter().copied(), gaussian.iter().copied()).with_caption("20*e^(-x^2)"));

    Figure::new()
        .with_size(80, 24)
        .with_layout(layout)
        .show();
}
