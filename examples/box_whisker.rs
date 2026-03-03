use ploot::prelude::*;

fn main() {
    // Five groups with pre-computed five-number summaries
    let groups  = [1.0, 2.0, 3.0, 4.0, 5.0];
    let mins    = [12.0, 18.0, 5.0, 22.0, 15.0];
    let q1s     = [20.0, 25.0, 15.0, 30.0, 22.0];
    let medians = [28.0, 32.0, 22.0, 38.0, 30.0];
    let q3s     = [35.0, 40.0, 30.0, 45.0, 38.0];
    let maxs    = [42.0, 48.0, 38.0, 52.0, 44.0];

    let bw = BoxAndWhiskerPlot::new(
        groups.iter().copied(),
        mins.iter().copied(),
        q1s.iter().copied(),
        medians.iter().copied(),
        q3s.iter().copied(),
        maxs.iter().copied(),
    )
    .with_caption("Scores")
    .with_color(TermColor::Cyan);

    let mut layout = Layout2D::new()
        .with_title("Box & Whisker")
        .with_x_label("group")
        .with_y_label("score")
        .with_plot(bw);
    layout.set_x_ticks_custom(&[
        (1.0, "A"), (2.0, "B"), (3.0, "C"), (4.0, "D"), (5.0, "E"),
    ]);

    Figure::new()
        .with_size(60, 20)
        .with_layout(layout)
        .show();
}
