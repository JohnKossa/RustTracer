use scene_items::color::Color;
#[derive(Debug, Copy, Clone)]
pub struct Material{
    pub reflection: f64,
    pub color: Color
}

impl Material{
    pub fn new(reflection: f64, color: Color) -> Material{
        Material{ reflection: reflection, color: color}
    }
}