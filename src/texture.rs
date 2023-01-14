use glam::{vec3, Vec3};
use image::{DynamicImage, GenericImageView};

#[derive(Debug)]
pub struct Texture {
    img: DynamicImage,
}

impl Texture {
    pub fn new(img: DynamicImage) -> Self {
        Self { img }
    }
    pub fn width(&self) -> f32 {
        self.img.width() as f32
    }
    pub fn height(&self) -> f32 {
        self.img.height() as f32
    }
    #[inline]
    pub fn pixel(&self, x: f32, y: f32) -> Vec3 {
        // 传入的坐标是以左下为原点的，而 image 库以左上为原点
        // 另外要记得限制 x、y 范围在 [0,1)
        let x = (self.img.height() as f32 * x.clamp(0., 0.9999)) as u32;
        let y = (self.img.width() as f32 * (1. - y).clamp(0., 0.9999)) as u32;
        let p = self.img.get_pixel(x, y).0;
        vec3(
            (p[2] as f32) / 255.,
            (p[1] as f32) / 255.,
            (p[0] as f32) / 255.,
        )
    }
}
