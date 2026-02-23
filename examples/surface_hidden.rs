use ploot::{Figure, GridData, SurfaceStyle};

fn main() {
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
}
