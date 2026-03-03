use ploot::prelude::*;

fn main() {
    // Three clusters of points with different markers
    let n = 40;
    let (mut x1, mut y1) = (Vec::new(), Vec::new());
    let (mut x2, mut y2) = (Vec::new(), Vec::new());
    let (mut x3, mut y3) = (Vec::new(), Vec::new());

    for i in 0..n {
        let t = i as f64 * 0.15;
        // Cluster A: lower-left, circular-ish
        x1.push(2.0 + t.sin() * 1.5 + (t * 3.7).sin() * 0.3);
        y1.push(3.0 + t.cos() * 1.5 + (t * 2.3).cos() * 0.3);
        // Cluster B: upper-right, tighter
        x2.push(7.0 + (t * 1.3).cos() * 1.0 + (t * 4.1).sin() * 0.2);
        y2.push(8.0 + (t * 0.9).sin() * 1.0 + (t * 3.0).cos() * 0.2);
        // Cluster C: middle, spread
        x3.push(5.0 + (t * 0.7).sin() * 2.0);
        y3.push(5.5 + (t * 1.1).cos() * 2.0);
    }

    let s1 = ScatterPlot::new(x1, y1)
        .with_caption("Group A")
        .with_color(TermColor::Red)
        .with_symbol(PointSymbol::Circle);

    let s2 = ScatterPlot::new(x2, y2)
        .with_caption("Group B")
        .with_color(TermColor::Blue)
        .with_symbol(PointSymbol::Cross);

    let s3 = ScatterPlot::new(x3, y3)
        .with_caption("Group C")
        .with_color(TermColor::Green)
        .with_symbol(PointSymbol::Diamond);

    let layout = Layout2D::new()
        .with_title("Scatter Plot")
        .with_x_label("x")
        .with_y_label("y")
        .with_plot(s1)
        .with_plot(s2)
        .with_plot(s3);

    Figure::new()
        .with_size(80, 24)
        .with_layout(layout)
        .show();
}
