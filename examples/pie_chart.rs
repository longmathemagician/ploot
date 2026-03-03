use ploot::prelude::*;

fn main() {
    let values = [35.0, 25.0, 20.0, 12.0, 8.0];
    let labels = ["Rust", "Python", "Go", "TypeScript", "Other"];

    let pie = PiePlot::new(values.iter().copied())
        .with_labels(labels.iter());

    let layout = Layout2D::new()
        .with_title("Language Popularity")
        .with_plot(pie);

    Figure::new()
        .with_size(60, 24)
        .with_layout(layout)
        .show();
}
