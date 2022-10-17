use crate::{
    object::Object,
    shader::{EmptyShader, Payload, Shader},
    triangle::Triangle,
};
use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};
use rgb::alt::BGRA8;

pub struct Rasterizer<S: Shader = EmptyShader> {
    width: usize,
    height: usize,
    frame_buf: Vec<BGRA8>,
    depth_buf: Vec<f32>,
    view: Mat4,
    projection: Mat4,
    pub shader: S,
}

// 实用函数
impl<S: Shader> Rasterizer<S> {
    pub fn new(width: usize, height: usize, shader: S) -> Self {
        Self {
            width,
            height,
            frame_buf: vec![Default::default(); width * height],
            depth_buf: vec![f32::NEG_INFINITY; width * height],
            view: Default::default(),
            projection: Default::default(),
            shader,
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.frame_buf.fill(Default::default());
        self.depth_buf.fill(f32::NEG_INFINITY);
    }

    /// 获取内部 BGRA 数据
    #[inline]
    pub fn data(&self) -> &[u32] {
        unsafe {
            std::slice::from_raw_parts(
                std::mem::transmute(self.frame_buf.as_ptr()),
                self.frame_buf.len() * 4,
            )
        }
    }

    #[inline]
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        let index = (self.height - 1 - y) * self.width + x;
        return index;
    }
}

// 3D 光栅化
impl<S: Shader> Rasterizer<S> {
    pub fn draw(&mut self, object: &Object) {
        let vp = self.projection * self.view;
        for t_id in 0..object.indices.len() {
            let [i, j, k] = object.indices[t_id];
            let [ni, nj, nk] = object.normal_indices[t_id];
            #[inline]
            fn to_homogeneous(v3: Vec3, w: f32) -> Vec4 {
                Vec4::new(v3.x, v3.y, v3.z, w)
            }
            let model_pos = [
                object.model * to_homogeneous(object.vertices[i], 1.),
                object.model * to_homogeneous(object.vertices[j], 1.),
                object.model * to_homogeneous(object.vertices[k], 1.),
            ];
            let mut t = Triangle {
                v: [vp * model_pos[0], vp * model_pos[1], vp * model_pos[2]],
                color: [
                    object.vertex_color[i],
                    object.vertex_color[j],
                    object.vertex_color[k],
                ],
                normal: [object.normals[ni], object.normals[nj], object.normals[nk]],
                ..Default::default()
            };
            // 齐次除法将 (x,y,z) 限定在 [-1,1]
            // 然后将 x、y 映射到屏幕坐标系上
            // z 的处理比较奇怪，暂且保留，后续也许可以去掉
            // w 没有进行齐次除法，因为按之前的计算这里的 w 保存了 mv 变换之后的真实 z 值
            for p in t.v.iter_mut() {
                p.x = 0.5 * self.width as f32 * (p.x / p.w + 1.);
                p.y = 0.5 * self.height as f32 * (p.y / p.w + 1.);
            }
            self.rasterize_triangle(&t, &model_pos);
        }
    }
    /// 将 3D 三角形光栅化到屏幕上。
    ///
    /// 注意 `t` 的 x y 坐标已经表示为屏幕坐标
    fn rasterize_triangle(&mut self, t: &Triangle, model_pos: &[Vec4; 3]) {
        // println!("{:?}", t.v);
        let bbox = t.bounding_box();
        let (left, top, right, bottom) = (
            bbox.0.clamp(0., (self.width - 1) as f32) as usize,
            bbox.1.clamp(0., (self.height - 1) as f32) as usize,
            bbox.2.clamp(0., (self.width - 1) as f32) as usize,
            bbox.3.clamp(0., (self.height - 1) as f32) as usize,
        );
        for py in bottom..=top {
            for px in left..=right {
                let (alpha, beta, gamma) = t.barycentric_coordinates(px as f32, py as f32);
                if !Triangle::inside_triangle(alpha, beta, gamma) {
                    continue;
                }
                // 透视校正插值
                let z = 1.0 / (alpha / t.v[0].w + beta / t.v[1].w + gamma / t.v[2].w);
                fn to_vec3(color: BGRA8) -> Vec3 {
                    Vec3::new(color.b as f32, color.g as f32, color.r as f32)
                }
                macro_rules! interp {
                    ($p0:expr,$p1:expr,$p2:expr) => {
                        z * (alpha * $p0 / t.v[0].w
                            + beta * $p1 / t.v[1].w
                            + gamma * $p2 / t.v[2].w)
                    };
                }
                let index = self.get_index(px, py);
                if self.depth_buf[index] < z {
                    let interp_color: Vec3 = interp!(
                        to_vec3(t.color[0]),
                        to_vec3(t.color[1]),
                        to_vec3(t.color[2])
                    );
                    let interp_normal: Vec3 =
                        interp!(t.normal[0], t.normal[1], t.normal[2]).normalize();
                    let interp_texture: Vec2 = interp!(t.texture[0], t.texture[1], t.texture[2]);
                    let interp_view_pos: Vec4 = interp!(model_pos[0], model_pos[1], model_pos[2]);
                    let payload = Payload {
                        color: interp_color,
                        normal: interp_normal,
                        point: Vec3::new(interp_view_pos.x, interp_view_pos.y, interp_view_pos.z),
                        tex_coords: interp_texture,
                    };
                    let color = self.shader.shading(&payload);

                    self.depth_buf[index] = z;
                    self.frame_buf[index] = color;
                }
            }
        }
    }
}

// builder 相关
impl<S: Shader> Rasterizer<S> {
    pub fn view(&mut self, view: Mat4) -> &mut Self {
        self.view = view;
        self
    }
    pub fn projection(&mut self, projection: Mat4) -> &mut Self {
        self.projection = projection;
        self
    }
}

// 基本原语，包括像素、直线
impl<S: Shader> Rasterizer<S> {
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: BGRA8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (self.height - 1 - y) * self.width + x;
        self.frame_buf[index] = color;
    }

    /// 绘制直线（线段），根据起点和中点
    ///
    /// 内置了 dda、中点 bresenham 和 改进 bresenham 算法
    ///
    /// 根据 benchmark 目前采取中点 bresenham 算法
    #[inline]
    pub fn draw_line(&mut self, from: Vec2, to: Vec2, color: BGRA8) {
        /// DDA，数值微分算法
        fn _dda<S: Shader>(rst: &mut Rasterizer<S>, from: Vec2, to: Vec2, color: BGRA8) {
            let (mut x, mut y) = (from.x, from.y);
            let (dx, dy) = (to.x - x, to.y - y);
            let eps = 1.0 / dx.abs().max(dy.abs());
            for _ in 0..(dx.abs().max(dy.abs())) as usize {
                // 这一步有进行浮点数转化为整数，比较耗时
                rst.set_pixel((x + 0.5) as usize, (y + 0.5) as usize, color);
                x += eps * dx;
                y += eps * dy;
            }
        }
        /// 中点 Bresenham 算法
        fn bresenham_center<S: Shader>(
            rst: &mut Rasterizer<S>,
            from: Vec2,
            to: Vec2,
            color: BGRA8,
        ) {
            let (x0, y0) = (from.x as i32, from.y as i32);
            let (x1, y1) = (to.x as i32, to.y as i32);

            let dx = x1 - x0;
            let dy = y1 - y0;
            let dx_abs = dx.abs();
            let dy_abs = dy.abs();

            // 以 y 为自变量
            if dx_abs < dy_abs {
                let (mut x0, _, mut y0, y1) = if dy < 0 {
                    (x1, x0, y1, y0)
                } else {
                    (x0, x1, y0, y1)
                };
                let mut d = 2 * dx_abs - dy_abs;

                let mut index = (rst.height - 1 - y0 as usize) * rst.width + x0 as usize;

                while y0 < y1 {
                    rst.frame_buf[index] = color;
                    y0 += 1;
                    index = index.wrapping_sub(rst.width);

                    if d <= 0 {
                        d += 2 * dx_abs;
                    } else {
                        if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                            x0 += 1;
                            index += 1;
                        } else {
                            x0 -= 1;
                            index -= 1;
                        }
                        d = d + 2 * (dx_abs - dy_abs);
                    }
                }
            } else {
                let (mut x0, x1, mut y0, _) = if dx < 0 {
                    (x1, x0, y1, y0)
                } else {
                    (x0, x1, y0, y1)
                };
                let mut d = 2 * dy_abs - dx_abs;
                let mut index = (rst.height - 1 - y0 as usize) * rst.width + x0 as usize;

                while x0 < x1 {
                    rst.frame_buf[index] = color;
                    x0 += 1;
                    index += 1;
                    if d < 0 {
                        d = d + 2 * dy_abs;
                    } else {
                        if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                            y0 += 1;
                            index -= rst.width;
                        } else {
                            y0 -= 1;
                            index += rst.width;
                        }
                        d = d + 2 * (dy_abs - dx_abs);
                    }
                }
            }
        }
        /// 改进 Bresenham 算法。当前的实现慢于上面的中点 Bresenham 算法
        fn _bresenham<S: Shader>(rst: &mut Rasterizer<S>, from: Vec2, to: Vec2, color: BGRA8) {
            let (x0, y0) = (from.x as i32, from.y as i32);
            let (x1, y1) = (to.x as i32, to.y as i32);

            // y 自变
            if (x1 - x0).abs() < (y1 - y0).abs() {
                let (mut x0, x1, mut y0, y1) = if y0 > y1 {
                    (x1, x0, y1, y0)
                } else {
                    (x0, x1, y0, y1)
                };
                let dx = x1 - x0;
                let dy = y1 - y0;
                let mut index = (rst.height - 1 - y0 as usize) * rst.width + x0 as usize;
                let mut e = if dx < 0 { dy } else { -dy };
                while y0 < y1 {
                    rst.frame_buf[index] = color;
                    y0 += 1;
                    index -= rst.width;
                    e += 2 * dx;
                    if e > 0 && dx >= 0 {
                        x0 += 1;
                        index += 1;
                        e -= 2 * dy;
                    } else if e < 0 && dx < 0 {
                        x0 -= 1;
                        index -= 1;
                        e += 2 * dy;
                    }
                }
            }
            // x 自变
            else {
                let (mut x0, x1, mut y0, y1) = if x0 > x1 {
                    (x1, x0, y1, y0)
                } else {
                    (x0, x1, y0, y1)
                };
                let dx = x1 - x0;
                let dy = y1 - y0;
                let mut index = (rst.height - 1 - y0 as usize) * rst.width + x0 as usize;
                let mut e = if dy < 0 { dx } else { -dx };
                while x0 < x1 {
                    rst.frame_buf[index] = color;
                    x0 += 1;
                    index += 1;
                    e += 2 * dy;
                    if e > 0 && dy >= 0 {
                        y0 += 1;
                        index -= rst.width;
                        e -= 2 * dx;
                    } else if e < 0 && dy < 0 {
                        y0 -= 1;
                        index += rst.width;
                        e += 2 * dx;
                    }
                }
            }
        }

        // dda(self, from, to, color);
        bresenham_center(self, from, to, color);
        // bresenham(self, from, to, color);
    }

    pub fn draw_crosshair(&mut self, size: usize, color: BGRA8) {
        let cx = self.width / 2;
        let cy = self.height / 2;
        let lx = cx - size / 2;
        let dy = cy - size / 2;
        for x in lx..(lx + size) {
            self.set_pixel(x, cy - 1, color);
            self.set_pixel(x, cy, color);
            self.set_pixel(x, cy + 1, color);
        }
        for y in dy..(dy + size) {
            self.set_pixel(cx - 1, y, color);
            self.set_pixel(cx, y, color);
            self.set_pixel(cx + 1, y, color);
        }
    }
}

// 复杂图形，包括双曲线和多边形
impl<S: Shader> Rasterizer<S> {
    /// 绘制以原点为中心点，焦点在 y 轴上的双曲线
    ///
    /// 需满足 a<b 以保证渐近线斜率小于 1
    pub fn draw_hyperbola(&mut self, a: i32, b: i32, center: Vec2, color: BGRA8) {
        // 原点的屏幕坐标
        let (x, y) = (center.x as i32, center.y as i32);
        let (mut x0, mut y0) = (x + a, y); // 从右顶点 (a,0) 开始画
        let mut d = 4 * a * a - a * b * b - b * b;
        let mut index1 = (self.height - y0 as usize) * self.width + x0 as usize;
        let mut index2 = (self.height - y0 as usize) * self.width + (2 * x - x0) as usize;
        let mut index3 = (self.height - (2 * y - y0) as usize) * self.width + (2 * x - x0) as usize;
        let mut index4 = (self.height - (2 * y - y0) as usize) * self.width + x0 as usize;
        let range = 0..self.width * self.height;
        loop {
            println!("{x0} {y0} {d}");
            if !range.contains(&index1)
                || !range.contains(&index2)
                || !range.contains(&index3)
                || !range.contains(&index4)
            {
                break;
            }
            self.frame_buf[index1] = color;
            self.frame_buf[index2] = color;
            self.frame_buf[index3] = color;
            self.frame_buf[index4] = color;
            if d > 0 {
                d += 4 * a * a * (2 * (y0 - y) + 3) - 8 * b * b * (x0 - x + 1);
                if index2 % self.width == 0 || index3 % self.width == 0 {
                    break;
                }
                if (index1 + 1) % self.width == 0 || (index4 + 1) % self.width == 0 {
                    break;
                }
                x0 += 1;
                index1 += 1;
                index2 -= 1;
                index3 -= 1;
                index4 += 1;
            } else {
                d += 4 * a * a * (2 * (y0 - y) + 3);
            }
            y0 += 1;
            index1 = index1.wrapping_sub(self.width);
            index2 = index2.wrapping_sub(self.width);
            index3 += self.width;
            index4 += self.width;
        }
    }

    /// 绘制多边形，用顶点序列输入，假定为逆时针输入。
    ///
    /// 顶点坐标必须是整数。
    pub fn draw_polygon(&mut self, vertices: &[Vec2], color: BGRA8) {
        // 注意，以下算法都假定输入至少三个顶点，也就是至少要构成多边形
        assert!(vertices.len() >= 3);
        /// x 扫描线算法
        fn _x_scan<S: Shader>(rst: &mut Rasterizer<S>, vert: &[Vec2], color: BGRA8) {
            // 限制边界为 0~height
            let mut y_min = rst.height as f32;
            let mut y_max = 0.0f32;

            for v in vert {
                y_min = y_min.min(v.y);
                y_max = y_max.max(v.y);
            }

            let y_min = y_min as usize;
            let y_max = y_max as usize;

            // 交点集合
            let mut intersection = Vec::new();

            // 扫描线自下而上扫描
            for y in y_min..y_max {
                intersection.clear();
                for i in 0..vert.len() {
                    let p1 = vert[i];
                    let p2 = vert[(i + 1) % vert.len()];
                    if p1.y == p2.y {
                        continue;
                    }
                    let alpha = (y as f32 - p2.y) / (p1.y - p2.y);
                    if 0. <= alpha && alpha <= 1. {
                        let x = alpha * p1.x + (1. - alpha) * p2.x;
                        // 元组第 1 维保存交点 x 坐标
                        //    第 2 维记录交点是在边的上半段还是下半段
                        // 第 2 维记录用于交点是共用顶点的特殊情况，用于判断共用顶点的边是否同侧
                        intersection.push((x, (y as f32) < (p1.y + p2.y) / 2.));
                    }
                }
                // 按 x 坐标排序
                intersection.sort_by_key(|p| p.0 as usize);
                let mut i = 0;
                // 每次取两个交点，然后根据共用顶点等情况决定是跳过还是绘制
                while i + 1 < intersection.len() {
                    let x1 = intersection[i].0.ceil() as usize;
                    let x2 = intersection[i + 1].0.floor() as usize;
                    if x1 >= x2 {
                        // 不同侧，视为 1 个交点，下次绘制
                        // 同侧，视为 2 个交点，直接跳过
                        i += if intersection[i].1 != intersection[i + 1].1 {
                            1
                        } else {
                            2
                        };
                        continue;
                    }
                    for x in x1..x2 {
                        rst.set_pixel(x, y, color);
                    }
                    i += 2;
                }
            }
        }
        /// 有效边表 (Active Edge Table, AET) 算法
        fn aet<S: Shader>(rst: &mut Rasterizer<S>, vert: &[Vec2], color: BGRA8) {
            struct Edge {
                from: Vec2,
                to_y: f32,
                k_r: f32,
            }
            let mut edges = Vec::with_capacity(vert.len());
            for i in 0..vert.len() {
                let from = vert[i];
                let to = vert[(i + 1) % vert.len()];
                // 忽略严格相等的情况，因为此时斜率为 0，斜率的倒数无法处理。
                // 其实也可以简单不管，因为根据约定，这样的边会自动从 AET 中删除
                if from.y < to.y {
                    edges.push(Edge {
                        from,
                        to_y: to.y,
                        k_r: (to.x - from.x) / (to.y - from.y),
                    })
                } else if from.y > to.y {
                    edges.push(Edge {
                        from: to,
                        to_y: from.y,
                        k_r: (to.x - from.x) / (to.y - from.y),
                    });
                }
            }
            // 所有的边按照 ymin 降序排序，如果 ymin 相同则按 x 降序排序
            // 降序是因为下面用的栈，从栈顶开始判定是否算入 AET
            // 按 x 降序是因为这样插入 AET 时先在前插入然后在后插入
            // 否则先在后插入，后在前插入会导致后插入的元素再移动
            // 这里的 AET 用的是动态数组而非链表，实际上是偷了点小懒
            // 频繁的元素移动可能会导致性能下降，但由于缓存等原因也有可能不下降，或许需要 benchmark 对比
            edges.sort_by(|lhs, rhs| {
                rhs.from
                    .y
                    .total_cmp(&lhs.from.y)
                    .then(rhs.from.x.total_cmp(&lhs.from.x))
            });
            struct ActiveEdge {
                x: f32,
                ymax: f32,
                /// 斜率的倒数 (reciprocal)
                k_r: f32,
            }
            let y_min = edges[edges.len() - 1].from.y as usize;
            let y_max = vert.into_iter().max_by_key(|e| e.y as usize).unwrap().y as usize;
            let mut aet: Vec<ActiveEdge> = Vec::with_capacity(2);
            for y in y_min..y_max {
                // 把所有恰好进入扫描线范围的边加入 AET
                while let Some(edge) = edges.last() {
                    if y >= edge.from.y as usize {
                        let pos = aet.partition_point(|ae| {
                            (ae.x < edge.from.x) || (ae.x == edge.from.x && ae.k_r < edge.k_r)
                        });
                        aet.insert(
                            pos,
                            ActiveEdge {
                                x: edge.from.x,
                                ymax: edge.to_y,
                                k_r: edge.k_r,
                            },
                        );
                        edges.pop();
                        continue;
                    }
                    break;
                }
                // 去除那些已经不再有效的边
                aet.retain(|ae| y < ae.ymax as usize);
                // 绘制，并且修改 x
                let mut i = 0;
                while i + 1 < aet.len() {
                    let x1 = aet[i].x.ceil() as usize;
                    let x2 = aet[i + 1].x.floor() as usize;
                    for x in x1..x2 {
                        rst.set_pixel(x, y, color);
                    }
                    aet[i].x += aet[i].k_r;
                    aet[i + 1].x += aet[i + 1].k_r;
                    i += 2;
                }
            }
        }
        // x_scan(self, vertices, color);
        aet(self, vertices, color);
    }
}

#[cfg(test)]
mod test {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;
    use super::*;
    use crate::{color, transform};

    fn show(buffer: &[u32]) {
        use minifb::{Key, Window, WindowOptions};
        let mut window = Window::new("test", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
        }
    }

    #[test]
    fn test_draw_line() {
        use nalgebra_glm::vec2;

        const WIDTH: usize = 800;
        const HEIGHT: usize = 800;

        let mut rst = Rasterizer::new(WIDTH, HEIGHT, EmptyShader);

        let p0 = vec2(400., 400.);

        rst.draw_line(p0, vec2(700., 600.), color::GREEN);
        rst.draw_line(p0, vec2(600., 700.), color::GREEN);
        rst.draw_line(p0, vec2(400., 750.), color::GREEN);
        rst.draw_line(p0, vec2(200., 700.), color::GREEN);
        rst.draw_line(p0, vec2(100., 600.), color::GREEN);
        rst.draw_line(p0, vec2(50., 400.), color::GREEN);
        rst.draw_line(p0, vec2(100., 200.), color::GREEN);
        rst.draw_line(p0, vec2(200., 100.), color::GREEN);
        rst.draw_line(p0, vec2(400., 50.), color::GREEN);
        rst.draw_line(p0, vec2(600., 100.), color::GREEN);
        rst.draw_line(p0, vec2(700., 200.), color::GREEN);
        rst.draw_line(p0, vec2(750., 400.), color::GREEN);
        show(rst.data());
    }

    #[test]
    fn test_draw_polygon1() {
        use crate::color;
        use nalgebra_glm::vec2;

        let mut rst = Rasterizer::new(WIDTH, HEIGHT, EmptyShader);

        let vertices = [
            vec2(10., 10.),
            vec2(200., 100.),
            vec2(150., 400.),
            vec2(100., 300.),
        ];
        rst.draw_polygon(&vertices, color::WHITE);
        let vertices = [
            vec2(500., 500.),
            vec2(450., 470.),
            vec2(400., 400.),
            vec2(300., 360.),
            vec2(340., 100.),
            vec2(420., 240.),
            vec2(460., 120.),
            vec2(530., 180.),
            vec2(520., 50.),
            vec2(670., 280.),
            vec2(480., 240.),
            vec2(630., 400.),
            vec2(480., 430.),
        ];
        rst.draw_polygon(&vertices, color::RED);
        show(rst.data());
    }

    #[test]
    fn test_draw_polygon2() {
        use crate::color;
        use nalgebra_glm::vec2;

        const WIDTH: usize = 800;
        const HEIGHT: usize = 800;

        let mut rst = Rasterizer::new(WIDTH, HEIGHT, EmptyShader);

        let mut vertices = Vec::with_capacity(100);
        vertices.push(vec2(600., 300.));
        let mut x = 550.;
        let mut y = 600.;
        while x > 50. {
            vertices.push(vec2(x, y));
            x -= 20.;
            y = 350.;
            vertices.push(vec2(x, y));
            x -= 20.;
            y = 600.;
        }
        vertices.push(vec2(50., 300.));
        y = 250.0;
        while x < 550. {
            vertices.push(vec2(x, y));
            x += 20.;
            y = 50.;
            vertices.push(vec2(x, y));
            x += 20.;
            y = 250.;
        }
        rst.draw_polygon(&vertices, color::RED);
        show(rst.data());
    }

    #[test]
    fn test_interplote() {
        let width = 800;
        let height = 800;
        let z_near = 0.1;
        let z_far = 50.;

        let mvp = transform::perspective(45., 1., z_near, z_far)
            * transform::view(Vec3::new(0., 0., 5.), 0., 0.)
            * transform::model(0., 0., 0., 2.5);

        // mvp 变换
        let mut points = [
            mvp * Vec4::new(-2., 2., -2., 1.),
            mvp * Vec4::new(2., 2., -2., 1.),
            mvp * Vec4::new(0., 0., -5., 1.),
        ];
        println!("mvp 变换后：");
        points.iter().for_each(|p| {
            println!("{:?}", p.as_slice());
        });
        println!();

        // 齐次除法
        for p in points.iter_mut() {
            p.x /= p.w;
            p.y /= p.w;
            p.z /= p.w;
        }
        println!("齐次除法后：");
        points.iter().for_each(|p| {
            println!("{:?}", p.as_slice());
        });
        println!();

        // viewport 变换
        let f1 = (z_far - z_near) / 2.;
        let f2 = -(z_far + z_near) / 2.;
        for p in points.iter_mut() {
            p.x = 0.5 * (p.x + 1.) * width as f32;
            p.y = 0.5 * (p.y + 1.) * height as f32;
            p.z = p.w * f1 + f2;
        }
        println!("viewport 变换：");
        points.iter().for_each(|p| {
            println!("{:?}", p.as_slice());
        });
        println!();
    }
}
