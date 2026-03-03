import re

with open("src/api/axes3d.rs", "r") as f:
    text = f.read()

text = text.replace("pub(crate) fn new() -> Self", "pub fn new() -> Self")
text = text.replace("pub struct Axes3D {", "pub type Layout3D = Axes3D;\n\n/// 3D axes that own surface data and view configuration.\npub struct Axes3D {")

addition = """
    pub fn with_surface(mut self, grid: GridData, style: SurfaceStyle, options: &[PlotOption]) -> Self {
        self.surface(grid, style, options);
        self
    }

    pub fn with_view(mut self, azimuth: f64, elevation: f64) -> Self {
        self.set_view(azimuth, elevation);
        self
    }

    pub fn with_colormap(mut self, cmap: ColorMapType) -> Self {
        self.set_colormap(cmap);
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.set_title(title);
        self
    }

    pub fn with_x_label(mut self, label: &str) -> Self {
        self.set_x_label(label);
        self
    }

    pub fn with_y_label(mut self, label: &str) -> Self {
        self.set_y_label(label);
        self
    }

    pub fn with_z_label(mut self, label: &str) -> Self {
        self.set_z_label(label);
        self
    }
"""

text = text.replace("    // ── Series addition methods ─────────────────────────────────────────", addition)
if addition not in text:
    text = text.replace("    pub fn set_view(", addition + "\n    pub fn set_view(")

with open("src/api/axes3d.rs", "w") as f:
    f.write(text)
