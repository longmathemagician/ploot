use ploot::prelude::*;

fn main() {
    let grid = GridData::from_fn(
        |x, y| x.sin() * y.cos(),
        (-std::f64::consts::PI, std::f64::consts::PI),
        (-std::f64::consts::PI, std::f64::consts::PI),
        40,
        40,
    );

    let layout = Layout2D::new()
        .with_title("Heatmap: sin(x)*cos(y)")
        .with_x_label("x")
        .with_y_label("y")
        .with_plot(HeatmapPlot::new(grid));

    Figure::new()
        .with_size(80, 24)
        .with_layout(layout)
        .show();
}
