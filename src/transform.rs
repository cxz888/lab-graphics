use glam::{Mat4, Vec3, Vec4};

/// 绕 z 旋转变换，`r` 为旋转的角度，以弧度制表示
pub fn rotation_z(r: f32) -> Mat4 {
    Mat4::from_rotation_z(r)
}

/// 绕 x 旋转变换，`r` 为旋转的角度，以弧度制表示
pub fn rotation_x(r: f32) -> Mat4 {
    Mat4::from_rotation_x(r)
}

/// 绕 x 旋转变换，`r` 为旋转的角度，以弧度制表示
pub fn rotation_y(r: f32) -> Mat4 {
    Mat4::from_rotation_y(r)
}

/// 缩放变换，三个参数分别为三个维度的缩放
pub fn scaling(x_scale: f32, y_scale: f32, z_scale: f32) -> Mat4 {
    Mat4::from_scale(Vec3::new(x_scale, y_scale, z_scale))
}

/// 平移变换，三个参数分别为三个维度的偏移值
pub fn tranlation(x_offset: f32, y_offset: f32, z_offset: f32) -> Mat4 {
    Mat4::from_translation(Vec3::new(x_offset, y_offset, z_offset))
}

/// 模型变换，将原始物体摆放到希望的位置上
///
pub fn model(offset_x: f32, offset_y: f32, offset_z: f32, angle: f32, scale: f32) -> Mat4 {
    tranlation(offset_x, offset_y, offset_z)
        * rotation_y(angle.to_radians())
        * scaling(scale, scale, scale)
}

/// 视图变换，将视点移至原点，朝向 -z 方向，上方向为 y 方向。
pub fn view(eye_pos: Vec3, angle_alpha: f32, angle_beta: f32) -> Mat4 {
    let t_view = tranlation(-eye_pos[0], -eye_pos[1], -eye_pos[2]);
    let a = angle_alpha.to_radians();
    let b = angle_beta.to_radians();
    let g = Vec3::new(-b.cos() * a.sin(), b.sin(), -b.cos() * a.cos());
    let t = Vec3::new(a.sin() * b.sin(), b.cos(), a.cos() * b.sin());
    let g_t = g.cross(t);
    #[rustfmt::skip]
    let r_view = Mat4::from_cols(
        Vec4::new(g_t.x, t.x, -g.x,0.),
        Vec4::new(g_t.y, t.y, -g.y,0.),
        Vec4::new(g_t.z, t.z, -g.z,0.),
        Vec4::new(0., 0., 0.,1.),
    );
    r_view * t_view
}

/// 投影变换，投影结果位于 \[-1,1\]^3 的标准立方体之间。这里是透视投影。
///
/// `fovy` y 轴视域角度，以角度输入，`aspect` 长宽比，`z_near`、`z_far` 分别是近远平面的**距离**
///
/// 注意在该种实现中，投影后的齐次坐标的 `w` 项是投影前的 `z` 坐标
pub fn perspective(fovy: f32, aspect: f32, z_near: f32, z_far: f32) -> Mat4 {
    let fovy = fovy.to_radians();

    let (zn, zf) = (-z_near, -z_far);
    let t = -(fovy / 2.).tan() * zn;
    let r = aspect * t;

    let ortho = orthogonal(-r, r, -t, t, zf, zn);
    let persp2ortho = Mat4::from_cols(
        Vec4::new(zn, 0., 0., 0.),
        Vec4::new(0., zn, 0., 0.),
        Vec4::new(0., 0., zn + zf, 1.),
        Vec4::new(0., 0., -zn * zf, 0.),
    );
    ortho * persp2ortho
}

/// 投影变换，投影结果位于 \[-1,1\]^3 的标准立方体之间。这里是正交投影。
///
/// l r b t n f
///
/// 注意在该种实现中，投影后的齐次坐标的 `w` 项是投影前的 `z` 坐标
pub fn orthogonal(l: f32, r: f32, b: f32, t: f32, f: f32, n: f32) -> Mat4 {
    Mat4::from_cols(
        Vec4::new(2. / (r - l), 0., 0., 0.),
        Vec4::new(0., 2. / (t - b), 0., 0.),
        Vec4::new(0., 0., 2. / (n - f), 0.),
        Vec4::new(
            -(r + l) / (r - l),
            -(t + b) / (t - b),
            -(n + f) / (n - f),
            1.,
        ),
    )
}
