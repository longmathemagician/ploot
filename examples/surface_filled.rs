use ploot::prelude::*;

fn main() {
    let grid = GridData::from_fn(
        |x, y| (x * x + y * y).sqrt().sin(),
        (-4.0, 4.0),
        (-4.0, 4.0),
        20,
        20,
    );

    let layout = Layout3D::new()
        .with_title("3D Filled: sin(sqrt(x^2+y^2))")
        .with_view(30.0, 30.0)
        .with_colormap(ColorMapType::Rainbow)
        .with_surface(grid, SurfaceStyle::Filled, &[]);

    Figure::new()
        .with_size(80, 24)
        .with_layout3d(layout)
        .show();
}
