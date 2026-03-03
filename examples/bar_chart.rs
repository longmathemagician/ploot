use ploot::prelude::*;
use ploot::AutoOption;

fn main() {
    let categories = [1.0, 2.0, 3.0, 4.0, 5.0];
    let values = [12.0, 25.0, 18.0, 30.0, 22.0];

    let plot = BarPlot::new(categories.iter().copied(), values.iter().copied())
        .with_caption("sales");

    let mut layout = Layout2D::new()
        .with_title("Bar Chart")
        .with_x_label("category")
        .with_y_label("value")
        .with_plot(plot);
    layout.set_y_range(AutoOption::Fix(0.0), AutoOption::Auto);

    Figure::new()
        .with_size(60, 18)
        .with_layout(layout)
        .show();
}
