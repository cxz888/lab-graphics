#![cfg_attr(test, allow(unused))]

use lab_graphics::object::Object;
use lab_graphics::rasterizer::Rasterizer;
use lab_graphics::shader::{light, PhongShader};
use lab_graphics::{color, transform};

use glam::{vec3, Vec3};
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
    let z_near = 0.1;
    let z_far = 50.;
    let lights = vec![
        light(vec3(20., 20., 20.), vec3(500., 500., 500.)),
        light(vec3(-20., 20., 0.), vec3(500., 500., 500.)),
    ];
    // 环境
    let amb_coeff = vec3(0.005, 0.005, 0.005);
    let amb_intensity = vec3(10., 10., 10.);
    let spec_coeff = vec3(0.7937, 0.7937, 0.7937);
    let spec_exp = 150;

    // 相机位置、水平角和仰角。
    let mut eye_pos = Vec3::new(0., 0., 10.);
    let mut angle_alpha = 0.;
    let mut angle_beta = 0.;

    let phong_shader = PhongShader::new(
        eye_pos,
        lights,
        amb_coeff,
        amb_intensity,
        spec_coeff,
        spec_exp,
    );

    let mut rst = Rasterizer::new(WIDTH, HEIGHT, phong_shader);
    rst.view(transform::view(eye_pos, angle_alpha, angle_beta))
        .projection(transform::perspective(45., 1., z_near, z_far));

    let spot = Object::load_obj("model/spot_triangulated_good.obj")
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
        // rst.shader.eye_pos(eye_pos);
    }
}

#[cfg(test)]
#[test]
fn test_mvp() {
    use glam::{Mat4, Vec4};
    use lab_graphics::shader::EmptyShader;

    let z_near = 0.1;
    let z_far = 50.;

    // 相机位置、水平角和仰角。
    let mut eye_pos = Vec3::new(0., 0., 0.);
    let mut angle_alpha = 0.;
    let mut angle_beta = 0.;

    let mut rst = Rasterizer::new(WIDTH, HEIGHT, EmptyShader);
    rst.view(transform::view(eye_pos, angle_alpha, angle_beta))
        .projection(transform::perspective(45., 1., z_near, z_far));

    let mut window = Window::new("Graphic Lab", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let p0 = Vec3::new(-1., 1., 5.);
    let p1 = Vec3::new(1., 1., 5.);
    let p2 = Vec3::new(0., -1., 15.);

    let obj = Object {
        vertices: vec![p0, p1, p2],
        vertex_color: vec![color::WHITE, color::WHITE, color::WHITE],
        normals: vec![
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
        ],
        texcoords: Vec::new(),
        indices: vec![[0, 1, 2]],
        normal_indices: vec![[0, 1, 2]],
        texcoord_indices: Vec::new(),
        model: Mat4::IDENTITY,
    };

    fn test(mut p: Vec4) {
        let project = transform::perspective(45., 1., 0.1, 50.);
        p = project * p;
        println!("裁剪坐标：{}", p);
        p /= p.w;
        println!("齐次除法：{}\n", p);
    }

    test(p0.extend(1.));
    test(p1.extend(1.));
    test(p2.extend(1.));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        rst.clear();
        rst.draw(&obj);

        window
            .update_with_buffer(rst.data(), WIDTH, HEIGHT)
            .unwrap();
    }
}
