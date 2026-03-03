use ploot::prelude::*;
use ploot::AutoOption;

fn main() {
    // Two overlapping distributions using deterministic trig-based pseudo-random data
    let mut dist_a = Vec::new();
    let mut dist_b = Vec::new();

    for i in 0..500 {
        let t = i as f64;
        // Distribution A: centered around 0
        let v = (t * 0.1).sin() + (t * 0.37).cos() * 1.5 + (t * 0.07).sin() * 0.8;
        dist_a.push(v);
        // Distribution B: centered around 2, narrower
        let v = 2.0 + (t * 0.13).cos() * 0.8 + (t * 0.41).sin() * 0.5;
        dist_b.push(v);
    }

    let h1 = HistogramPlot::new(dist_a, 25)
        .with_caption("Sample A")
        .with_color(TermColor::Cyan)
        .with_normalize(true)
        .with_range(-4.0, 5.0);

    let h2 = HistogramPlot::new(dist_b, 25)
        .with_caption("Sample B")
        .with_color(TermColor::Magenta)
        .with_normalize(true)
        .with_range(-4.0, 5.0);

    let mut layout = Layout2D::new()
        .with_title("Overlapping Histograms")
        .with_x_label("value")
        .with_y_label("density")
        .with_plot(h1)
        .with_plot(h2);
    layout.set_y_range(AutoOption::Fix(0.0), AutoOption::Auto);

    Figure::new()
        .with_size(80, 24)
        .with_layout(layout)
        .show();
}
