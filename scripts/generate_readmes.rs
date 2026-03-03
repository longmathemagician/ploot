use ploot::prelude::*;
use ploot::AutoOption;

fn main() {
    let _ = std::fs::create_dir_all("screenshots");

    // 1. Line plot
    {
        let xs: Vec<f64> = (0..=60).map(|i| i as f64 / 10.0).collect();
        let y1: Vec<f64> = xs.iter().map(|&x| x.sin()).collect();
        let y2: Vec<f64> = xs.iter().map(|&x| x.cos()).collect();
        let y3: Vec<f64> = xs.iter().map(|&x| (x * 2.0).sin() * 0.5).collect();

        let mut fig = Figure::new();
        fig.set_terminal_size(80, 24);
        let ax = fig.axes2d();
        ax.set_title("Line Types");
        ax.set_y_grid(true);

        ax.lines(
            xs.iter().copied(),
            y1.iter().copied(),
            &[PlotOption::Caption("Solid".into()), PlotOption::LineStyle(DashType::Solid)],
        );
        ax.lines(
            xs.iter().copied(),
            y2.iter().copied(),
            &[PlotOption::Caption("Dash".into()), PlotOption::LineStyle(DashType::Dash)],
        );
        ax.lines(
            xs.iter().copied(),
            y3.iter().copied(),
            &[PlotOption::Caption("Dot".into()), PlotOption::LineStyle(DashType::Dot)],
        );

        fig.save_svg("screenshots/line_plot.svg", true).unwrap();
    }

    // 2. Bar chart
    {
        let categories = [1.0, 2.0, 3.0, 4.0, 5.0];
        let values = [12.0, 25.0, 18.0, 30.0, 22.0];

        let mut layout = Layout2D::new()
            .with_title("Bar Chart")
            .with_x_label("category")
            .with_y_label("value")
            .with_plot(BarPlot::new(categories.iter().copied(), values.iter().copied())
                .with_caption("sales"));
                
        layout.set_y_range(AutoOption::Fix(0.0), AutoOption::Auto);

        Figure::new()
            .with_size(60, 18)
            .with_layout(layout)
            .save_svg("screenshots/bar_chart.svg", true).unwrap();
    }

    // 3. Heatmap
    {
        let grid = GridData::from_fn(
            |x, y| x.sin() * y.cos(),
            (-std::f64::consts::PI, std::f64::consts::PI),
            (-std::f64::consts::PI, std::f64::consts::PI),
            60,
            30,
        );

        let layout = Layout2D::new()
            .with_title("Heatmap")
            .with_plot(HeatmapPlot::new(grid).with_colormap(ColorMapType::Heat));

        Figure::new()
            .with_size(80, 24)
            .with_layout(layout)
            .save_svg("screenshots/heatmap.svg", true).unwrap();
    }

    // 4. Contour
    {
        let grid = GridData::from_fn(
            |x, y| (-0.5 * (x * x + y * y)).exp(),
            (-3.0, 3.0),
            (-3.0, 3.0),
            40,
            40,
        );

        let layout = Layout2D::new()
            .with_title("Contour")
            .with_plot(ContourPlot::new(grid).with_levels(8));

        Figure::new()
            .with_size(80, 24)
            .with_layout(layout)
            .save_svg("screenshots/contour.svg", true).unwrap();
    }

    // 5. Heatmap + Contour
    {
        let grid = GridData::from_fn(
            |x, y| x.sin() * y.cos(),
            (-std::f64::consts::PI, std::f64::consts::PI),
            (-std::f64::consts::PI, std::f64::consts::PI),
            60,
            30,
        );

        let layout = Layout2D::new()
            .with_title("Heatmap + Contour")
            .with_plot(HeatmapContourPlot::new(grid).with_colormap(ColorMapType::BlueRed));

        Figure::new()
            .with_size(80, 24)
            .with_layout(layout)
            .save_svg("screenshots/heatmap_contour.svg", true).unwrap();
    }

    // 6. Dual Axis
    {
        let months: Vec<f64> = (1..=12).map(|i| i as f64).collect();
        let temperature = vec![
            -2.0, 0.5, 5.0, 11.0, 16.0, 20.0, 22.0, 21.0, 17.0, 11.0, 5.0, 0.0,
        ];
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
            .save_svg("screenshots/dual_axis.svg", true).unwrap();
    }

    // 7. Multi Series
    {
        let xs: Vec<f64> = (-30..=30).map(|i| i as f64 / 10.0).collect();
        let y1: Vec<f64> = xs.iter().map(|&x| x * x).collect();
        let y2: Vec<f64> = xs.iter().map(|&x| 8.0 * (x * 1.5).sin()).collect();
        let y3: Vec<f64> = xs.iter().map(|&x| -x * x + 5.0).collect();
        let y4: Vec<f64> = xs.iter().map(|&x| x * 2.0).collect();

        let mut fig = Figure::new();
        fig.set_terminal_size(80, 24);
        let ax = fig.axes2d();
        ax.set_title("Multi-series Auto Colors");
        ax.set_y_grid(true);

        ax.lines(xs.iter().copied(), y1.iter().copied(), &[PlotOption::Caption("x^2".into())]);
        ax.lines(xs.iter().copied(), y2.iter().copied(), &[PlotOption::Caption("8*sin(1.5x)".into())]);
        ax.lines(xs.iter().copied(), y3.iter().copied(), &[PlotOption::Caption("-x^2 + 5".into())]);
        ax.lines(xs.iter().copied(), y4.iter().copied(), &[PlotOption::Caption("2x".into())]);

        fig.save_svg("screenshots/multi_series.svg", true).unwrap();
    }

    // 8. Surface Wireframe
    {
        let grid = GridData::from_fn(
            |x, y| (x * x + y * y).sqrt().sin(),
            (-5.0, 5.0),
            (-5.0, 5.0),
            30,
            30,
        );

        let layout = Layout3D::new()
            .with_title("Surface Wireframe")
            .with_surface(grid, SurfaceStyle::Wireframe, &[]);

        Figure::new()
            .with_size(80, 24)
            .with_layout3d(layout)
            .save_svg("screenshots/surface_wireframe.svg", true).unwrap();
    }

    // 9. Surface Hidden
    {
        let grid = GridData::from_fn(
            |x, y| x.sin() + y.cos(),
            (-std::f64::consts::PI, std::f64::consts::PI),
            (-std::f64::consts::PI, std::f64::consts::PI),
            40,
            40,
        );

        let layout = Layout3D::new()
            .with_title("Surface Hidden")
            .with_surface(grid, SurfaceStyle::HiddenLine, &[]);

        Figure::new()
            .with_size(80, 24)
            .with_layout3d(layout)
            .save_svg("screenshots/surface_hidden.svg", true).unwrap();
    }

    // 10. Surface Filled
    {
        let grid = GridData::from_fn(
            |x, y| (x * x + y * y).sqrt().sin(),
            (-5.0, 5.0),
            (-5.0, 5.0),
            40,
            40,
        );

        let layout = Layout3D::new()
            .with_title("Surface Filled")
            .with_colormap(ColorMapType::Rainbow)
            .with_surface(grid, SurfaceStyle::Filled, &[]);

        Figure::new()
            .with_size(80, 24)
            .with_layout3d(layout)
            .save_svg("screenshots/surface_filled.svg", true).unwrap();
    }

    // 11. Scatter Plot
    {
        let n = 40;
        let (mut x1, mut y1) = (Vec::new(), Vec::new());
        let (mut x2, mut y2) = (Vec::new(), Vec::new());
        let (mut x3, mut y3) = (Vec::new(), Vec::new());

        for i in 0..n {
            let t = i as f64 * 0.15;
            x1.push(2.0 + t.sin() * 1.5 + (t * 3.7).sin() * 0.3);
            y1.push(3.0 + t.cos() * 1.5 + (t * 2.3).cos() * 0.3);
            x2.push(7.0 + (t * 1.3).cos() * 1.0 + (t * 4.1).sin() * 0.2);
            y2.push(8.0 + (t * 0.9).sin() * 1.0 + (t * 3.0).cos() * 0.2);
            x3.push(5.0 + (t * 0.7).sin() * 2.0);
            y3.push(5.5 + (t * 1.1).cos() * 2.0);
        }

        let layout = Layout2D::new()
            .with_title("Scatter Plot")
            .with_x_label("x")
            .with_y_label("y")
            .with_plot(ScatterPlot::new(x1, y1).with_caption("Group A").with_color(TermColor::Red).with_symbol(PointSymbol::Circle))
            .with_plot(ScatterPlot::new(x2, y2).with_caption("Group B").with_color(TermColor::Blue).with_symbol(PointSymbol::Cross))
            .with_plot(ScatterPlot::new(x3, y3).with_caption("Group C").with_color(TermColor::Green).with_symbol(PointSymbol::Diamond));

        Figure::new()
            .with_size(80, 24)
            .with_layout(layout)
            .save_svg("screenshots/scatter.svg", true).unwrap();
    }

    // 12. Histogram
    {
        let mut dist_a = Vec::new();
        let mut dist_b = Vec::new();
        for i in 0..500 {
            let t = i as f64;
            dist_a.push((t * 0.1).sin() + (t * 0.37).cos() * 1.5 + (t * 0.07).sin() * 0.8);
            dist_b.push(2.0 + (t * 0.13).cos() * 0.8 + (t * 0.41).sin() * 0.5);
        }

        let mut layout = Layout2D::new()
            .with_title("Overlapping Histograms")
            .with_x_label("value")
            .with_y_label("density")
            .with_plot(HistogramPlot::new(dist_a, 25).with_caption("Sample A").with_color(TermColor::Cyan).with_normalize(true).with_range(-4.0, 5.0))
            .with_plot(HistogramPlot::new(dist_b, 25).with_caption("Sample B").with_color(TermColor::Magenta).with_normalize(true).with_range(-4.0, 5.0));
        layout.set_y_range(AutoOption::Fix(0.0), AutoOption::Auto);

        Figure::new()
            .with_size(80, 24)
            .with_layout(layout)
            .save_svg("screenshots/histogram.svg", true).unwrap();
    }

    // 13. Box & Whisker
    {
        let groups  = [1.0, 2.0, 3.0, 4.0, 5.0];
        let mins    = [12.0, 18.0, 5.0, 22.0, 15.0];
        let q1s     = [20.0, 25.0, 15.0, 30.0, 22.0];
        let medians = [28.0, 32.0, 22.0, 38.0, 30.0];
        let q3s     = [35.0, 40.0, 30.0, 45.0, 38.0];
        let maxs    = [42.0, 48.0, 38.0, 52.0, 44.0];

        let mut layout = Layout2D::new()
            .with_title("Box & Whisker")
            .with_x_label("group")
            .with_y_label("score")
            .with_plot(BoxAndWhiskerPlot::new(
                groups.iter().copied(), mins.iter().copied(), q1s.iter().copied(),
                medians.iter().copied(), q3s.iter().copied(), maxs.iter().copied(),
            ).with_caption("Scores").with_color(TermColor::Cyan));
        layout.set_x_ticks_custom(&[
            (1.0, "A"), (2.0, "B"), (3.0, "C"), (4.0, "D"), (5.0, "E"),
        ]);

        Figure::new()
            .with_size(60, 20)
            .with_layout(layout)
            .save_svg("screenshots/box_whisker.svg", true).unwrap();
    }

    // 14. Confidence Band (Fill Between)
    {
        let xs: Vec<f64> = (0..=80).map(|i| i as f64 * 0.1).collect();
        let mean: Vec<f64> = xs.iter().map(|&x| x.sin() * 2.0 + x * 0.3).collect();
        let upper: Vec<f64> = mean.iter().enumerate().map(|(i, &m)| {
            m + 0.5 + (i as f64 * 0.2).sin().abs() * 0.5
        }).collect();
        let lower: Vec<f64> = mean.iter().enumerate().map(|(i, &m)| {
            m - 0.5 - (i as f64 * 0.2).cos().abs() * 0.5
        }).collect();

        let layout = Layout2D::new()
            .with_title("Confidence Band")
            .with_x_label("x")
            .with_y_label("y")
            .with_plot(FillBetweenPlot::new(xs.iter().copied(), lower.iter().copied(), upper.iter().copied())
                .with_caption("95% CI").with_color(TermColor::Cyan))
            .with_plot(LinePlot::new(xs.iter().copied(), mean.iter().copied())
                .with_caption("Mean").with_color(TermColor::Blue));

        Figure::new()
            .with_size(80, 24)
            .with_layout(layout)
            .save_svg("screenshots/fill_between.svg", true).unwrap();
    }

    // 15. Candlestick Chart
    {
        let days: Vec<f64> = (1..=20).map(|i| i as f64).collect();
        let mut open  = Vec::new();
        let mut high  = Vec::new();
        let mut low   = Vec::new();
        let mut close = Vec::new();
        let mut price = 100.0;
        for i in 0..20 {
            let t = i as f64;
            let o = price;
            let change = (t * 1.3).sin() * 3.0 + (t * 0.7).cos() * 2.0;
            let c = o + change;
            let h = o.max(c) + (t * 2.1).sin().abs() * 2.0 + 0.5;
            let l = o.min(c) - (t * 1.7).cos().abs() * 2.0 - 0.5;
            open.push(o);
            high.push(h);
            low.push(l);
            close.push(c);
            price = c;
        }

        let layout = Layout2D::new()
            .with_title("Candlestick Chart")
            .with_x_label("day")
            .with_y_label("price ($)")
            .with_plot(CandlestickPlot::new(
                days.iter().copied(), open.iter().copied(), high.iter().copied(),
                low.iter().copied(), close.iter().copied(),
            ).with_caption("ACME Corp"));

        Figure::new()
            .with_size(80, 24)
            .with_layout(layout)
            .save_svg("screenshots/candlestick.svg", true).unwrap();
    }

    // 16. Pie Chart
    {
        let layout = Layout2D::new()
            .with_title("Language Popularity")
            .with_plot(PiePlot::new([35.0, 25.0, 20.0, 12.0, 8.0].iter().copied())
                .with_labels(["Rust", "Python", "Go", "TypeScript", "Other"].iter()));

        Figure::new()
            .with_size(60, 24)
            .with_layout(layout)
            .save_svg("screenshots/pie_chart.svg", true).unwrap();
    }

    println!("Successfully regenerated all SVG screenshots.");
}
