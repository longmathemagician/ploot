use ploot::{ColorMapType, Figure, GridData, SurfaceStyle};

fn main() {
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
