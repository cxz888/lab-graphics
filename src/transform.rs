use nalgebra_glm::Mat4;

/// 绕 z 旋转变换，`r` 为旋转的角度，以弧度制表示
#[rustfmt::skip]
pub fn rotation_z(r: f32) -> Mat4 {
    Mat4::new(
        r.cos(), -r.sin(), 0., 0.,
        r.sin(), r.cos(),0.,  0.,
        0., 0., 1., 0.,
        0., 0., 0., 1.,
    )
}

/// 绕 x 旋转变换，`r` 为旋转的角度，以弧度制表示
#[rustfmt::skip]
pub fn rotation_x(r: f32) -> Mat4 {
    Mat4::new(
        1., 0., 0., 0.,
        0., r.cos(),-r.sin(),  0.,
        0., r.sin(), r.cos(), 0.,
        0., 0., 0., 1.,
    )
}

/// 缩放变换，三个参数分别为三个维度的缩放
#[rustfmt::skip]
pub fn scaling(x_scale: f32,y_scale:f32,z_scale:f32) -> Mat4 {
    Mat4::new(
        x_scale, 0., 0., 0.,
        0., y_scale, 0., 0.,
        0., 0., z_scale, 0.,
        0., 0., 0., 1.,
    )
}

/// 平移变换，三个参数分别为三个维度的偏移值
#[rustfmt::skip]
pub fn tranlation(x_offset: f32,y_offset:f32,z_offset:f32) -> Mat4 {
    Mat4::new(
        1., 0., 0., x_offset,
        0., 1., 0., y_offset,
        0., 0., 1., z_offset,
        0., 0., 0., 1.,
    )
}
