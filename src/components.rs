#[derive(Copy, Clone)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct VelocityComponent {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct ColorComponent {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
