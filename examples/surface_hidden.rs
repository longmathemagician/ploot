use ploot::prelude::*;

fn main() {
    let grid = GridData::from_fn(
        |x, y| x.sin() + y.cos(),
        (-std::f64::consts::PI, std::f64::consts::PI),
        (-std::f64::consts::PI, std::f64::consts::PI),
        20,
        20,
    );

    let layout = Layout3D::new()
        .with_title("3D Hidden-Line: sin(x) + cos(y)")
        .with_view(45.0, 30.0)
        .with_surface(grid, SurfaceStyle::HiddenLine, &[]);

    Figure::new()
        .with_size(80, 24)
        .with_layout3d(layout)
        .show();
}
