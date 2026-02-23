fn main() {
    let xs: Vec<f64> = (-30..=30).map(|i| i as f64 / 10.0).collect();
    let quadratic: Vec<f64> = xs.iter().map(|&x| x * x).collect();
    let cubic: Vec<f64> = xs.iter().map(|&x| x * x * x).collect();
    let sine: Vec<f64> = xs.iter().map(|&x| 8.0 * (x * 1.5).sin()).collect();
    let gaussian: Vec<f64> = xs.iter().map(|&x| 20.0 * (-x * x).exp()).collect();

    let plot = ploot::quick_plot_multi(
        &[
            (&xs, &quadratic),
            (&xs, &cubic),
            (&xs, &sine),
            (&xs, &gaussian),
        ],
        Some("x^2  vs  x^3  vs  8*sin(1.5x)  vs  20*e^(-x^2)"),
        Some("x"),
        Some("y"),
        80,
        24,
    );
    println!("{plot}");
}
