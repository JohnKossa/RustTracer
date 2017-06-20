use std::ops::{Add, Sub, Mul};

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

    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color{r: r, g: g, b: b}
    }

    pub fn BLACK() -> Color { Color{r: 0.0, g: 0.0, b: 0.0} }

    pub fn WHITE() -> Color { Color{r: 1.0, g: 1.0, b: 1.0} }

    pub fn RED() -> Color { Color{r: 1.0, g: 0.0, b: 0.0} }

    pub fn GREEN() -> Color { Color{r: 0.0, g: 1.0, b: 0.0} }

    pub fn BLUE() -> Color { Color{r: 0.0, g: 0.0, b: 1.0} }

    pub fn MAGENTA() -> Color { Color{r: 1.0, g: 0.0, b: 1.0} }

    pub fn YELLOW() -> Color { Color{r: 1.0, g: 1.0, b: 0.0} }

    pub fn CYAN() -> Color { Color{r: 0.0, g: 1.0, b: 1.0} }

    pub fn normalize(self) -> Color{
        let highest = self.r.max(self.g.max(self.b));
        if highest > 1.0 {
            return self.scale(1.0/highest);
        }
        let lowest = self.r.min(self.g.min(self.b));
        if lowest < 0.0{
            return Color {r: self.r + lowest, g: self.g + lowest, b: self.b + lowest}.normalize();
        }
        self
    }

    pub fn scale(self, factor: f64) -> Color {
        Color{r: self.r * factor, g: self.g * factor, b: self.b * factor}.normalize()
    }
}

impl Add for Color{
    type Output = Color;

    fn add(self, other: Color) -> Color{
        Color{r: self.r + other.r, g: self.g + other.g, b: self.b + other.b}.normalize()
    }
}

impl Sub for Color{
    type Output = Color;

    fn sub(self, other: Color) -> Color{
        Color{r: self.r - other.r, g: self.g - other.g, b: self.b - other.b}.normalize()
    }
}

impl Mul for Color{
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color{r: self.r * other.r, g: self.g * other.g, b: self.b * other.b}.normalize()
    }
}