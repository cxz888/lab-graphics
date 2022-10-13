#![cfg_attr(test, allow(unused))]

pub mod color;
pub mod object;
pub mod rasterizer;
pub mod transform;
pub mod triangle;

use object::Object;
use rasterizer::Rasterizer;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Mat4, Vec3};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

/// 模型变换，将原始物体摆放到希望的位置上
///
/// 这里仅将其按 z 轴旋转。体现在最终结果上应该是顺时针、逆时针旋转
///
/// `angle` 以角度输入
fn get_model(offset_x: f32, offset_y: f32, offset_z: f32) -> Mat4 {
    transform::tranlation(offset_x, offset_y, offset_z)
}

/// 视图变换，将视点移至原点，朝向 -z 方向，上方向为 y 方向。
fn get_view(eye_pos: Vec3, angle_alpha: f32, angle_beta: f32) -> Mat4 {
    let t_view = transform::tranlation(-eye_pos[0], -eye_pos[1], -eye_pos[2]);
    let a = angle_alpha.to_radians();
    let b = angle_beta.to_radians();
    let g = Vec3::new(-b.cos() * a.sin(), b.sin(), -b.cos() * a.cos());
    let t = Vec3::new(a.sin() * b.sin(), b.cos(), a.cos() * b.sin());
    let g_t = g.cross(&t);
    #[rustfmt::skip]
    let r_view = Mat4::new(
        g_t.x, g_t.y, g_t.z, 0.,
        t.x, t.y, t.z, 0.,
        -(g.x), -(g.y), -(g.z), 0.,
        0., 0., 0., 1.,
    );
    r_view * t_view
}

/// 投影变换，投影结果位于 \[-1,1\]^3 的标准立方体之间。这里是透视投影。
///
/// `fovy` y 轴视域角度，以角度输入，`aspect` 长宽比，z_near、z_far 分别是近远平面的**距离**
///
/// 注意在该种实现中，投影后的齐次坐标的 `w` 项是投影前的 `z` 坐标
fn get_projection(fovy: f32, aspect: f32, z_near: f32, z_far: f32) -> Mat4 {
    let fovy = fovy.to_radians();

    let (zn, zf) = (-z_near, -z_far);
    let t = -(fovy / 2.).tan() * zn;
    let r = aspect * t;

    #[rustfmt::skip]
    let ortho: Mat4 = Mat4::new(
        1./r, 0., 0., 0.,
        0., 1./t, 0., 0.,
        0., 0., 2./(zn-zf), -(zn+zf)/(zn-zf),
        0., 0., 0., 1.
    );
    #[rustfmt::skip]
    let persp2ortho: Mat4 = Mat4::new(
        zn, 0., 0., 0.,
        0., zn, 0., 0.,
        0., 0., zn+zf, -zn*zf,
        0., 0., 1., 0.
    );
    ortho * persp2ortho
}

fn response_keyboard(
    window: &Window,
    eye_pos: &mut Vec3,
    angle_alpha: &mut f32,
    angle_beta: &mut f32,
) -> bool {
    const DELTA_ANGLE_ALPHA: f32 = 1.0;
    const DELTA_ANGLE_BETA: f32 = 0.5;
    const VELOCITY: f32 = 1.0;
    const DELTA_Y: f32 = 1.0;
    let mut should_print = false;
    let a = angle_alpha.to_radians();
    if window.is_key_down(Key::A) {
        eye_pos.x -= VELOCITY * a.cos();
        eye_pos.z += VELOCITY * a.sin();
        should_print = true;
    }
    if window.is_key_down(Key::D) {
        eye_pos.x += VELOCITY * a.cos();
        eye_pos.z -= VELOCITY * a.sin();
        should_print = true;
    }
    if window.is_key_down(Key::W) {
        eye_pos.x -= VELOCITY * a.sin();
        eye_pos.z -= VELOCITY * a.cos();
        should_print = true;
    }
    if window.is_key_down(Key::S) {
        eye_pos.x += VELOCITY * a.sin();
        eye_pos.z += VELOCITY * a.cos();
        should_print = true;
    }
    if window.is_key_down(Key::Space) {
        eye_pos.y += DELTA_Y;
        should_print = true;
    }
    if window.is_key_down(Key::LeftShift) {
        eye_pos.y -= DELTA_Y;
        should_print = true;
    }
    // 仰角限定在 [-π/2, π/2]
    if window.is_key_down(Key::Up) {
        *angle_beta = (*angle_beta + DELTA_ANGLE_BETA).min(90.);
        should_print = true;
    }
    if window.is_key_down(Key::Down) {
        *angle_beta = (*angle_beta - DELTA_ANGLE_BETA).max(-90.);
        should_print = true;
    }
    if window.is_key_down(Key::Left) {
        *angle_alpha += DELTA_ANGLE_ALPHA;
        should_print = true;
    }
    if window.is_key_down(Key::Right) {
        *angle_alpha -= DELTA_ANGLE_ALPHA;
        should_print = true;
    }
    should_print
}

fn main() {
    let z_near = 1.;
    let z_far = 50.;

    // 相机位置、水平角和仰角。
    let mut eye_pos = Vec3::new(0., 0., 20.);
    let mut angle_alpha = 0.;
    let mut angle_beta = 0.;
    let mut rst = Rasterizer::new(WIDTH, HEIGHT);
    rst.view(get_view(eye_pos, angle_alpha, angle_beta))
        .projection(get_projection(45., 1., z_near, z_far));

    let tetra = Object::load_obj("model/tetrahedron.obj")
        .unwrap()
        .model(get_model(0., 0., 0.));
    let ground = Object::load_obj("model/ground.obj")
        .unwrap()
        .model(get_model(0., 0., 0.));
    let objects = vec![tetra, ground];

    let mut window = Window::new("Graphic Lab", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    // 限制至多为 60fps
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        rst.clear();
        rst.view(get_view(eye_pos, angle_alpha, angle_beta));
        for obj in &objects {
            rst.draw(obj, z_near, z_far);
        }
        rst.draw_crosshair(20, color::RED);
        window
            .update_with_buffer(rst.data(), WIDTH, HEIGHT)
            .unwrap();
        if response_keyboard(&window, &mut eye_pos, &mut angle_alpha, &mut angle_beta) {
            println!("视点: {}", eye_pos);
            println!("水平角: {}", angle_alpha);
            println!("仰角: {}", angle_beta);
            println!();
        }
    }
}
