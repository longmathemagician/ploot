use ploot::prelude::*;
use ploot::AutoOption;

fn main() {
    let months: Vec<f64> = (1..=12).map(|i| i as f64).collect();
    // Temperature in °C (primary y-axis, small range)
    let temperature = vec![
        -2.0, 0.5, 5.0, 11.0, 16.0, 20.0, 22.0, 21.0, 17.0, 11.0, 5.0, 0.0,
    ];
    // Rainfall in mm (secondary y-axis, large range)
    let rainfall = vec![
        45.0, 40.0, 55.0, 60.0, 70.0, 80.0, 75.0, 65.0, 70.0, 75.0, 60.0, 50.0,
    ];

    let t_plot = LinePlot::new(months.iter().copied(), temperature.iter().copied())
        .with_caption("Temperature")
        .with_color(TermColor::Red);

    let r_plot = LinePlot::new(months.iter().copied(), rainfall.iter().copied())
        .with_caption("Rainfall")
        .with_color(TermColor::Blue)
        .with_axes(AxisPair::X1Y2)
        .with_dash(DashType::Dash);

    let mut layout = Layout2D::new()
        .with_title("Monthly Temperature & Rainfall")
        .with_x_label("Month")
        .with_y_label("Temperature (C)")
        .with_plot(t_plot)
        .with_plot(r_plot);
        
    layout.set_y2_label("Rainfall (mm)", &[]);
    layout.set_x_range(AutoOption::Fix(1.0), AutoOption::Fix(12.0));
    layout.set_x_ticks_custom(&[
        (1.0, "Jan"), (2.0, "Feb"), (3.0, "Mar"), (4.0, "Apr"),
        (5.0, "May"), (6.0, "Jun"), (7.0, "Jul"), (8.0, "Aug"),
        (9.0, "Sep"), (10.0, "Oct"), (11.0, "Nov"), (12.0, "Dec"),
    ]);
    layout.set_y_grid(true);

    Figure::new()
        .with_size(80, 24)
        .with_layout(layout)
        .show();
}
