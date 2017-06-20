use geometry::point::Point;
use scene_items::color::Color;
pub struct Light{
    pub position: Point,
    pub color: Color
}

impl Light{
    pub fn new(position: Point, color: Color) -> Light {
        Light{position: position, color: color}
    }
}
