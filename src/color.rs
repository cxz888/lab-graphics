use glam::{vec3, Vec3};
use rgb::alt::BGRA8;

// 注，颜色的顺序按照 bgr 排列
pub const WHITE: Vec3 = vec3(1., 1., 1.);
pub const GREEN: Vec3 = vec3(0., 1., 0.);
pub const RED: Vec3 = vec3(0., 0., 1.);

pub fn to_bgra(color: Vec3) -> BGRA8 {
    BGRA8 {
        b: (color.x * 255.) as u8,
        g: (color.y * 255.) as u8,
        r: (color.z * 255.) as u8,
        a: 0,
    }
}

pub fn to_vec3(color: BGRA8) -> Vec3 {
    vec3(
        color.b as f32 * 255.,
        color.g as f32 * 255.,
        color.r as f32 * 255.,
    )
}
