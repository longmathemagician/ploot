use ploot::{AutoOption, AxisPair, DashType, Figure, PlotOption};
use ploot::canvas::color::TermColor;

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

    let mut fig = Figure::new();
    fig.set_terminal_size(80, 24);
    {
        let ax = fig.axes2d();
        ax.set_title("Monthly Temperature & Rainfall");
        ax.set_x_label("Month", &[]);
        ax.set_y_label("Temperature (C)", &[]);
        ax.set_y2_label("Rainfall (mm)", &[]);

        ax.set_x_range(AutoOption::Fix(1.0), AutoOption::Fix(12.0));
        ax.set_x_ticks_custom(&[
            (1.0, "Jan"),
            (2.0, "Feb"),
            (3.0, "Mar"),
            (4.0, "Apr"),
            (5.0, "May"),
            (6.0, "Jun"),
            (7.0, "Jul"),
            (8.0, "Aug"),
            (9.0, "Sep"),
            (10.0, "Oct"),
            (11.0, "Nov"),
            (12.0, "Dec"),
        ]);

        // Temperature on primary y-axis
        ax.lines(
            months.iter().copied(),
            temperature.iter().copied(),
            &[
                PlotOption::Caption("Temperature".into()),
                PlotOption::Color(TermColor::Red),
            ],
        );

        // Rainfall on secondary y-axis
        ax.lines(
            months.iter().copied(),
            rainfall.iter().copied(),
            &[
                PlotOption::Caption("Rainfall".into()),
                PlotOption::Color(TermColor::Blue),
                PlotOption::Axes(AxisPair::X1Y2),
                PlotOption::LineStyle(DashType::Dash),
            ],
        );

        ax.set_y_grid(true);
    }

    println!("{}", fig.render());
}
