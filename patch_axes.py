import re

with open("src/api/axes.rs", "r") as f:
    text = f.read()

text = text.replace("pub(crate) fn new() -> Self", "pub fn new() -> Self")

addition = """
    /// Create a layout automatically based on given plots.
    pub fn auto_from_plots<P: Into<Plot2D> + Clone>(plots: &[P]) -> Self {
        let mut layout = Self::new();
        for plot in plots {
            layout.series.push(plot.clone().into());
        }
        layout
    }

    /// Add a plot to the layout directly.
    pub fn with_plot<P: Into<Plot2D>>(mut self, plot: P) -> Self {
        self.series.push(plot.into());
        self
    }

    /// Consuming builder for title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.set_title(title);
        self
    }

    /// Consuming builder for x label.
    pub fn with_x_label(mut self, label: &str) -> Self {
        self.set_x_label(label, &[]);
        self
    }

    /// Consuming builder for y label.
    pub fn with_y_label(mut self, label: &str) -> Self {
        self.set_y_label(label, &[]);
        self
    }

    /// Consuming builder for y grid.
    pub fn with_y_grid(mut self, enabled: bool) -> Self {
        self.set_y_grid(enabled);
        self
    }

    /// Consuming builder for x grid.
    pub fn with_x_grid(mut self, enabled: bool) -> Self {
        self.set_x_grid(enabled);
        self
    }
"""

text = text.replace("    // ── Series addition methods ─────────────────────────────────────────", addition + "\n    // ── Series addition methods ─────────────────────────────────────────")

with open("src/api/axes.rs", "w") as f:
    f.write(text)
