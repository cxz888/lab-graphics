use glam::{Vec2, Vec3, Vec4};
use rgb::alt::BGRA8;

#[derive(Default, Debug)]
pub struct Triangle {
    /// 三个顶点的齐次坐标，不保证顺序
    pub v: [Vec4; 3],
    /// 三个顶点的颜色
    pub color: [BGRA8; 3],
    /// 三个顶点对应的纹理坐标
    pub texture: [Vec2; 3],
    /// 三个顶点的法线
    pub normal: [Vec3; 3],
}

impl Triangle {
    /// 返回三角形平面投影的最小包围盒
    ///
    /// 按 left, top, right bottom 顺序返回
    #[inline]
    pub fn bounding_box(&self) -> (f32, f32, f32, f32) {
        let left = self.v[0].x.min(self.v[1].x).min(self.v[2].x);
        let right = self.v[0].x.max(self.v[1].x).max(self.v[2].x);
        let top = self.v[0].y.max(self.v[1].y).max(self.v[2].y);
        let bottom = self.v[0].y.min(self.v[1].y).min(self.v[2].y);
        return (left, top, right, bottom);
    }
    /// 计算 (x,y) 在三角形平面投影上的重心坐标表示
    pub fn barycentric_coordinates(&self, x: f32, y: f32) -> (f32, f32, f32) {
        let v = &self.v;
        let alpha = (-(x - v[1].x) * (v[2].y - v[1].y) + (y - v[1].y) * (v[2].x - v[1].x))
            / (-(v[0].x - v[1].x) * (v[2].y - v[1].y) + (v[0].y - v[1].y) * (v[2].x - v[1].x));
        let beta = (-(x - v[2].x) * (v[0].y - v[2].y) + (y - v[2].y) * (v[0].x - v[2].x))
            / (-(v[1].x - v[2].x) * (v[0].y - v[2].y) + (v[1].y - v[2].y) * (v[0].x - v[2].x));
        (alpha, beta, 1.0 - alpha - beta)
    }
    /// 根据重心坐标判断一个点是否在三角形内部
    #[inline]
    pub fn inside_triangle(alpha: f32, beta: f32, gamma: f32) -> bool {
        // 重心坐标任意一个为负即说明在外部
        return alpha >= 0. && beta >= 0. && gamma >= 0.;
    }
}
