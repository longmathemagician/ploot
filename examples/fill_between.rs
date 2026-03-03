use ploot::prelude::*;

fn main() {
    // Line with a confidence band
    let xs: Vec<f64> = (0..=80).map(|i| i as f64 * 0.1).collect();
    let mean: Vec<f64> = xs.iter().map(|&x| x.sin() * 2.0 + x * 0.3).collect();
    let upper: Vec<f64> = mean.iter().enumerate().map(|(i, &m)| {
        m + 0.5 + (i as f64 * 0.2).sin().abs() * 0.5
    }).collect();
    let lower: Vec<f64> = mean.iter().enumerate().map(|(i, &m)| {
        m - 0.5 - (i as f64 * 0.2).cos().abs() * 0.5
    }).collect();

    let band = FillBetweenPlot::new(
        xs.iter().copied(),
        lower.iter().copied(),
        upper.iter().copied(),
    )
    .with_caption("95% CI")
    .with_color(TermColor::Cyan);

    let line = LinePlot::new(xs.iter().copied(), mean.iter().copied())
        .with_caption("Mean")
        .with_color(TermColor::Blue);

    let layout = Layout2D::new()
        .with_title("Confidence Band")
        .with_x_label("x")
        .with_y_label("y")
        .with_plot(band)
        .with_plot(line);

    Figure::new()
        .with_size(80, 24)
        .with_layout(layout)
        .show();
}
