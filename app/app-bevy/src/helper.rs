use glam::{quat, Mat4, Vec2, Vec3};
pub use simulation::helper::wrap;

pub struct Bounds {
    width: f32,
    height: f32,
}

impl Bounds {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

pub fn transform_viewport_from_postion(bounds: Vec2, position: Vec3) -> Mat4 {
    Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        quat(0., 0., 0., 0.),
        position * bounds.extend(1.0),
    )
}
