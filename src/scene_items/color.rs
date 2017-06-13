#[derive(Debug, Copy, Clone)]
pub struct Color{
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Color{
    fn origin() -> Color {
        Color {r:0.0, g:0.0, b:0.0}
    }

    fn new(r: f64, g: f64, b: f64) -> Color {
        Color{r: r, g: g, b: b}
    }

    fn add_color(self, other: Color) -> Color {
        Color{r: self.r+other.r, g: self.g+other.g, b: self.b+other.b}
    }

    fn times_color(self, other: Color) -> Color {
        Color{r: self.r * other.r, g: self.g * other.g, b: self.b * other.b}
    }

    fn scale(self, factor: f64) -> Color {
        Color{r: self.r * factor, g: self.g * factor, b: self.b * factor}
    }
}