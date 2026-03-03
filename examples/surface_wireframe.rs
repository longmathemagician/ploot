use ploot::prelude::*;

fn main() {
    let grid = GridData::from_fn(
        |x, y| (x * x + y * y).sqrt().sin(),
        (-4.0, 4.0),
        (-4.0, 4.0),
        25,
        25,
    );

    let layout = Layout3D::new()
        .with_title("3D Wireframe: sin(sqrt(x^2+y^2))")
        .with_view(35.0, 25.0)
        .with_surface(grid, SurfaceStyle::Wireframe, &[]);

    Figure::new()
        .with_size(80, 24)
        .with_layout3d(layout)
        .show();
}
