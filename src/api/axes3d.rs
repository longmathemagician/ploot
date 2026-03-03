use crate::canvas::ColorMapType;
use crate::canvas::PALETTE;
use crate::canvas::color::TermColor;

use super::grid::GridData;
use super::options::PlotOption;

/// Rendering style for 3D surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceStyle {
    /// Wireframe mesh (no hidden-line removal).
    Wireframe,
    /// Wireframe with hidden-line removal via z-buffer.
    HiddenLine,
    /// Filled surface with color-mapped density shading.
    Filled,
}

/// A single surface dataset with its rendering style.
#[derive(Debug, Clone)]
pub struct SurfaceData {
    pub(crate) grid: GridData,
    pub(crate) style: SurfaceStyle,
    pub(crate) options: Vec<PlotOption>,
}

/// 3D axes that own surface data and view configuration.
pub type Layout3D = Axes3D;

/// 3D axes that own surface data and view configuration.
pub struct Axes3D {
    pub(crate) surfaces: Vec<SurfaceData>,
    pub(crate) title: Option<String>,
    pub(crate) x_label: Option<String>,
    pub(crate) y_label: Option<String>,
    pub(crate) z_label: Option<String>,
    pub(crate) azimuth: f64,
    pub(crate) elevation: f64,
    pub(crate) colormap: ColorMapType,
    next_color: usize,
}

impl Axes3D {
    /// Create a new empty 3D axes with default view angles.
    pub fn new() -> Self {
        Self {
            surfaces: Vec::new(),
            title: None,
            x_label: None,
            y_label: None,
            z_label: None,
            azimuth: 30.0,
            elevation: 30.0,
            colormap: ColorMapType::Heat,
            next_color: 0,
        }
    }

    /// Add a surface dataset (consuming builder).
    pub fn with_surface(mut self, grid: GridData, style: SurfaceStyle, options: &[PlotOption]) -> Self {
        self.surface(grid, style, options);
        self
    }

    /// Set the viewing angle (consuming builder).
    pub fn with_view(mut self, azimuth: f64, elevation: f64) -> Self {
        self.set_view(azimuth, elevation);
        self
    }

    /// Set the colormap (consuming builder).
    pub fn with_colormap(mut self, cmap: ColorMapType) -> Self {
        self.set_colormap(cmap);
        self
    }

    /// Set the plot title (consuming builder).
    pub fn with_title(mut self, title: &str) -> Self {
        self.set_title(title);
        self
    }

    /// Set the x-axis label (consuming builder).
    pub fn with_x_label(mut self, label: &str) -> Self {
        self.set_x_label(label);
        self
    }

    /// Set the y-axis label (consuming builder).
    pub fn with_y_label(mut self, label: &str) -> Self {
        self.set_y_label(label);
        self
    }

    /// Set the z-axis label (consuming builder).
    pub fn with_z_label(mut self, label: &str) -> Self {
        self.set_z_label(label);
        self
    }

    /// Set the viewing angle (azimuth and elevation in degrees).
    pub fn set_view(&mut self, azimuth: f64, elevation: f64) -> &mut Self {
        self.azimuth = azimuth;
        self.elevation = elevation;
        self
    }

    /// Set the colormap for filled surfaces.
    pub fn set_colormap(&mut self, cmap: ColorMapType) -> &mut Self {
        self.colormap = cmap;
        self
    }

    /// Add a surface to the plot.
    pub fn surface(
        &mut self,
        grid: GridData,
        style: SurfaceStyle,
        options: &[PlotOption],
    ) -> &mut Self {
        self.next_color += 1;
        self.surfaces.push(SurfaceData {
            grid,
            style,
            options: options.to_vec(),
        });
        self
    }

    /// Set the plot title.
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set the x-axis label.
    pub fn set_x_label(&mut self, label: &str) -> &mut Self {
        self.x_label = Some(label.to_string());
        self
    }

    /// Set the y-axis label.
    pub fn set_y_label(&mut self, label: &str) -> &mut Self {
        self.y_label = Some(label.to_string());
        self
    }

    /// Set the z-axis label.
    pub fn set_z_label(&mut self, label: &str) -> &mut Self {
        self.z_label = Some(label.to_string());
        self
    }

    /// Resolve color for a surface from options or palette.
    pub(crate) fn resolve_color(&self, opts: &[PlotOption], idx: usize) -> TermColor {
        opts.iter()
            .find_map(|o| {
                if let PlotOption::Color(c) = o {
                    Some(*c)
                } else {
                    None
                }
            })
            .unwrap_or(PALETTE[idx % PALETTE.len()])
    }
}
