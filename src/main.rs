fn main() {
    // Demo 1: Legacy quick_plot_multi API (unchanged)
    let xs: Vec<f64> = (-30..=30).map(|i| i as f64 / 10.0).collect();
    let quadratic: Vec<f64> = xs.iter().map(|&x| x * x).collect();
    let cubic: Vec<f64> = xs.iter().map(|&x| x * x * x).collect();
    let sine: Vec<f64> = xs.iter().map(|&x| 8.0 * (x * 1.5).sin()).collect();
    let gaussian: Vec<f64> = xs.iter().map(|&x| 20.0 * (-x * x).exp()).collect();

    let plot = ploot::quick_plot_multi(
        &[
            (&xs, &quadratic),
            (&xs, &cubic),
            (&xs, &sine),
            (&xs, &gaussian),
        ],
        Some("x^2  vs  x^3  vs  8*sin(1.5x)  vs  20*e^(-x^2)"),
        Some("x"),
        Some("y"),
        80,
        24,
    );
    println!("{plot}");

    println!();

    // Demo 2: New Figure/Axes2D builder API
    use ploot::{AutoOption, DashType, Figure, PlotOption, PointSymbol};

    let mut fig = Figure::new();
    fig.set_terminal_size(80, 24);
    {
        let ax = fig.axes2d();
        ax.set_title("Builder API Demo");
        ax.set_x_label("x", &[]);
        ax.set_y_label("y", &[]);
        ax.set_y_grid(true);

        ax.lines(
            xs.iter().copied(),
            quadratic.iter().copied(),
            &[
                PlotOption::Caption("x^2".into()),
                PlotOption::LineStyle(DashType::Solid),
            ],
        );
        ax.lines(
            xs.iter().copied(),
            sine.iter().copied(),
            &[
                PlotOption::Caption("8*sin(1.5x)".into()),
                PlotOption::LineStyle(DashType::Dash),
            ],
        );
        ax.points(
            xs.iter().copied(),
            gaussian.iter().copied(),
            &[
                PlotOption::Caption("20*e^(-x^2)".into()),
                PlotOption::PointSymbol(PointSymbol::Cross),
            ],
        );
    }
    fig.show();

    println!();

    // Demo 3: Bar chart
    let mut fig = Figure::new();
    fig.set_terminal_size(60, 18);
    {
        let ax = fig.axes2d();
        ax.set_title("Bar Chart");
        ax.set_x_label("category", &[]);
        ax.set_y_label("value", &[]);
        ax.set_y_range(AutoOption::Fix(0.0), AutoOption::Auto);
        let categories = [1.0, 2.0, 3.0, 4.0, 5.0];
        let values = [12.0, 25.0, 18.0, 30.0, 22.0];
        ax.boxes(
            categories.iter().copied(),
            values.iter().copied(),
            &[PlotOption::Caption("sales".into())],
        );
    }
    fig.show();
}
