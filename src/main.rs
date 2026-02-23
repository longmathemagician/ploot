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
    use ploot::{
        AutoOption, ColorMapType, DashType, Figure, GridData, PlotOption, PointSymbol, SurfaceStyle,
    };

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

    println!();

    // Demo 4: Heatmap — sin(x)*cos(y)
    let grid = GridData::from_fn(
        |x, y| x.sin() * y.cos(),
        (-std::f64::consts::PI, std::f64::consts::PI),
        (-std::f64::consts::PI, std::f64::consts::PI),
        40,
        40,
    );
    let output = ploot::quick_heatmap(
        grid,
        Some("Heatmap: sin(x)*cos(y)"),
        Some("x"),
        Some("y"),
        80,
        24,
    );
    println!("{output}");

    println!();

    // Demo 5: Contour — Gaussian
    let grid = GridData::from_fn(
        |x, y| (-0.5 * (x * x + y * y)).exp(),
        (-3.0, 3.0),
        (-3.0, 3.0),
        40,
        40,
    );
    let output = ploot::quick_contour(grid, 8, Some("Contour: Gaussian"), 80, 24);
    println!("{output}");

    println!();

    // Demo 6: Heatmap + Contour overlay
    let grid = GridData::from_fn(
        |x, y| x.sin() * y.cos(),
        (-std::f64::consts::PI, std::f64::consts::PI),
        (-std::f64::consts::PI, std::f64::consts::PI),
        40,
        40,
    );
    let mut fig = Figure::new();
    fig.set_terminal_size(80, 24);
    {
        let ax = fig.axes2d();
        ax.set_title("Heatmap + Contour: sin(x)*cos(y)");
        ax.set_x_label("x", &[]);
        ax.set_y_label("y", &[]);
        ax.heatmap_contour(grid, None, &[PlotOption::ContourLevels(10)]);
    }
    fig.show();

    println!();

    // Demo 7: 3D Wireframe — sin(sqrt(x^2+y^2))
    let grid = GridData::from_fn(
        |x, y| (x * x + y * y).sqrt().sin(),
        (-4.0, 4.0),
        (-4.0, 4.0),
        25,
        25,
    );
    let output = ploot::quick_surface(
        grid,
        Some("3D Wireframe: sin(sqrt(x^2+y^2))"),
        80,
        24,
        35.0,
        25.0,
    );
    println!("{output}");

    println!();

    // Demo 8: 3D Hidden-line surface
    let grid = GridData::from_fn(
        |x, y| x.sin() + y.cos(),
        (-std::f64::consts::PI, std::f64::consts::PI),
        (-std::f64::consts::PI, std::f64::consts::PI),
        20,
        20,
    );
    let mut fig = Figure::new();
    fig.set_terminal_size(80, 24);
    {
        let ax = fig.axes3d();
        ax.set_title("3D Hidden-Line: sin(x) + cos(y)");
        ax.set_view(45.0, 30.0);
        ax.surface(grid, SurfaceStyle::HiddenLine, &[]);
    }
    fig.show();

    println!();

    // Demo 9: 3D Filled surface
    let grid = GridData::from_fn(
        |x, y| (x * x + y * y).sqrt().sin(),
        (-4.0, 4.0),
        (-4.0, 4.0),
        20,
        20,
    );
    let mut fig = Figure::new();
    fig.set_terminal_size(80, 24);
    {
        let ax = fig.axes3d();
        ax.set_title("3D Filled: sin(sqrt(x^2+y^2))");
        ax.set_view(30.0, 30.0);
        ax.set_colormap(ColorMapType::Rainbow);
        ax.surface(grid, SurfaceStyle::Filled, &[]);
    }
    fig.show();
}
