use ploot::{Figure, GridData, PlotOption};

fn main() {
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
}
