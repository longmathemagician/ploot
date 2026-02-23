use ploot::{AutoOption, Figure, PlotOption};

fn main() {
    let mut fig = Figure::new();
    fig.set_terminal_size(60, 18);
    {
        let ax = fig.axes2d();
        ax.set_title("Bar Chart");
        ax.set_x_label("category", &[]);
        ax.set_y_label("value", &[]);
        ax.set_y_range(AutoOption::Fix(0.0), AutoOption::Auto);
        let categories = [1.0, 2.0, 3.0, 4.0, 5.0];
        let values = [12.0, 25.0, 18.0, 30.0, 22.0];
        ax.boxes(
            categories.iter().copied(),
            values.iter().copied(),
            &[PlotOption::Caption("sales".into())],
        );
    }
    fig.show();
}
