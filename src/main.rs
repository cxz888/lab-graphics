#![cfg_attr(test, allow(unused))]

use lab_graphics::object::Object;
use lab_graphics::rasterizer::Rasterizer;
use lab_graphics::shader::{Light, PhongShader};
use lab_graphics::{color, transform};

use glam::Vec3;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn response_keyboard(
    window: &Window,
    eye_pos: &mut Vec3,
    angle_alpha: &mut f32,
    angle_beta: &mut f32,
) -> bool {
    const DELTA_ANGLE_ALPHA: f32 = 2.0;
    const DELTA_ANGLE_BETA: f32 = 1.0;
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
    let z_near = 0.1;
    let z_far = 50.;

    let lights = vec![
        Light {
            source: Vec3::new(-20., 20., -20.),
            intensity: Vec3::new(500., 500., 500.),
        },
        Light {
            source: Vec3::new(20., 20., -20.),
            intensity: Vec3::new(500., 500., 500.),
        },
        Light {
            source: Vec3::new(0., 30., -50.0),
            intensity: Vec3::new(400., 400., 400.),
        },
    ];
    let diffuse_coeff = Vec3::new(0.005, 0.005, 0.005);
    let amb_coeff = Vec3::new(0.005, 0.005, 0.005);
    let amb_intensity = Vec3::new(10., 10., 10.);
    let spec_coeff = Vec3::new(0.9, 0.9, 0.9);
    let spec_exp = 150;

    // 相机位置、水平角和仰角。
    let mut eye_pos = Vec3::new(28., 21., 35.34);
    let mut angle_alpha = 40.;
    let mut angle_beta = -10.;

    let phong_shader = PhongShader::new(
        eye_pos,
        lights,
        diffuse_coeff,
        amb_coeff,
        amb_intensity,
        spec_coeff,
        spec_exp,
    );

    let mut rst = Rasterizer::new(WIDTH, HEIGHT, phong_shader);
    rst.view(transform::view(eye_pos, angle_alpha, angle_beta))
        .projection(transform::perspective(45., 1., z_near, z_far));

    let tetra = Object::load_obj("model/tetrahedron.obj")
        .unwrap()
        .model(transform::model(0., 0., 0., 1.));
    let spot = Object::load_obj("model/spot_triangulated_good.obj")
        .unwrap()
        .model(transform::model(0., 0., 0., 2.5));
    let cube = Object::load_obj("model/cube.obj")
        .unwrap()
        .model(transform::model(-20., 0., 0., 1.0));
    let objects = vec![spot];

    let mut window = Window::new("Graphic Lab", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    // 限制至多为 60fps
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        rst.clear();
        rst.view(transform::view(eye_pos, angle_alpha, angle_beta));
        rst.shader.eye_pos(eye_pos);
        for obj in &objects {
            rst.draw(obj);
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

#[test]
fn test_as() {
    let a: f32 = 1e30;
    let b = a as u8;
    let c = a as usize;
    println!("{b} {c}");
}
