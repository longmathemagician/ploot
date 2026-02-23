use ploot::GridData;

fn main() {
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
}
