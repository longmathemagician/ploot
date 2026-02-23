use ploot::GridData;

fn main() {
    let grid = GridData::from_fn(
        |x, y| x.sin() * y.cos(),
        (-std::f64::consts::PI, std::f64::consts::PI),
        (-std::f64::consts::PI, std::f64::consts::PI),
        40,
        40,
    );
    let output = ploot::quick_heatmap(
        grid,
        Some("Heatmap: sin(x)*cos(y)"),
        Some("x"),
        Some("y"),
        80,
        24,
    );
    println!("{output}");
}
