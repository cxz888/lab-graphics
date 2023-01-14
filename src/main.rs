#![cfg_attr(test, allow(unused))]

use lab_graphics::object::Object;
use lab_graphics::rasterizer::Rasterizer;
use lab_graphics::shaders::{BlinnPhongShader, BumpShader, DisplacementShader, TextureShader};
use lab_graphics::{color, transform};

use glam::Vec3;
use minifb::{Key, MouseMode, Window, WindowOptions};

const WIDTH: usize = 700;
const HEIGHT: usize = 700;

fn respond_keyboard(
    window: &Window,
    eye_pos: &mut Vec3,
    angle_alpha: &mut f32,
    angle_beta: &mut f32,
) {
    const VELOCITY: f32 = 1.0;
    const DELTA_Y: f32 = 1.0;
    let a = angle_alpha.to_radians();
    if window.is_key_down(Key::A) {
        eye_pos.x -= VELOCITY * a.cos();
        eye_pos.z += VELOCITY * a.sin();
    }
    if window.is_key_down(Key::D) {
        eye_pos.x += VELOCITY * a.cos();
        eye_pos.z -= VELOCITY * a.sin();
    }
    if window.is_key_down(Key::W) {
        eye_pos.x -= VELOCITY * a.sin();
        eye_pos.z -= VELOCITY * a.cos();
    }
    if window.is_key_down(Key::S) {
        eye_pos.x += VELOCITY * a.sin();
        eye_pos.z += VELOCITY * a.cos();
    }
    if window.is_key_down(Key::Space) {
        eye_pos.y += DELTA_Y;
    }
    if window.is_key_down(Key::LeftShift) {
        eye_pos.y -= DELTA_Y;
    }
    // if window.is_key_down(Key::Left) {
    //     light_source.x -= VELOCITY;
    // }
    // if window.is_key_down(Key::Right) {
    //     light_source.x += VELOCITY;
    // }
    // if window.is_key_down(Key::Up) {
    //     light_source.z += VELOCITY;
    // }
    // if window.is_key_down(Key::Down) {
    //     light_source.z -= VELOCITY;
    // }
    // if window.is_key_down(Key::RightShift) {
    //     light_source.y -= VELOCITY;
    // }
    // if window.is_key_down(Key::Enter) {
    //     light_source.y += VELOCITY;
    // }
    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
        *angle_alpha = 360. * (WIDTH as f32 - x) / (WIDTH as f32) - 180.;
        *angle_beta = 180. * (HEIGHT as f32 - y) / (HEIGHT as f32) - 90.;
    }
    if window.is_key_down(Key::I) {
        println!("视点：{eye_pos}");
        println!("水平角：{angle_alpha}");
        println!("仰角：{angle_beta}");
        println!();
    }
}

fn main() {
    // NOTE: 可能是因为选取的坐标系和法向量变幻的方法不同，normal、bump、displacement 的效果和参考的不太一致

    let z_near = 0.1;
    let z_far = 50.;

    // 相机位置、水平角和仰角。
    let mut eye_pos = Vec3::new(0., 0., 10.);
    let mut angle_alpha = 0.;
    let mut angle_beta = 0.;

    let _phong_shader = BlinnPhongShader::example(eye_pos);
    let _texture_shader = TextureShader::example(eye_pos);
    let _bump_shader = BumpShader::new(eye_pos);
    let displacement_shader = DisplacementShader::example(eye_pos);

    let mut rst = Rasterizer::new(WIDTH, HEIGHT, displacement_shader);
    rst.view(transform::view(eye_pos, angle_alpha, angle_beta))
        .projection(transform::perspective(45., 1., z_near, z_far));

    let spot = Object::load_obj("model/spot_triangulated_good.obj", "model/hmap.jpg")
        .unwrap()
        .model(transform::model(0., 0., 0., 140., 2.5));
    let objects = vec![spot];

    let mut window = Window::new("Graphic Lab", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    // 限制至多为 60fps
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        rst.clear();
        rst.view(transform::view(eye_pos, angle_alpha, angle_beta));
        // rst.shader.eye_pos(eye_pos);
        for obj in &objects {
            rst.draw(obj);
        }
        rst.draw_crosshair(20, color::RED);
        window
            .update_with_buffer(rst.data(), WIDTH, HEIGHT)
            .unwrap();

        respond_keyboard(&window, &mut eye_pos, &mut angle_alpha, &mut angle_beta);
        rst.shader.eye_pos(eye_pos);
    }
}
