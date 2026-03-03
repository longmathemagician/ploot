use ploot::prelude::*;

fn main() {
    let grid = GridData::from_fn(
        |x, y| (-0.5 * (x * x + y * y)).exp(),
        (-3.0, 3.0),
        (-3.0, 3.0),
        40,
        40,
    );

    let layout = Layout2D::new()
        .with_title("Contour: Gaussian")
        .with_plot(ContourPlot::new(grid).with_levels(8));

    Figure::new()
        .with_size(80, 24)
        .with_layout(layout)
        .show();
}
