use ploot::GridData;

fn main() {
    let grid = GridData::from_fn(
        |x, y| (-0.5 * (x * x + y * y)).exp(),
        (-3.0, 3.0),
        (-3.0, 3.0),
        40,
        40,
    );
    let output = ploot::quick_contour(grid, 8, Some("Contour: Gaussian"), 80, 24);
    println!("{output}");
}
