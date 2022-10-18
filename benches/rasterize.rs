use lab_graphics::{
    object::Object,
    shader::{EmptyShader, Light, PhongShader},
    transform, Rasterizer,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::Vec3;

pub fn criterion_benchmark(c: &mut Criterion) {
    let width = 800;
    let height = 800;
    let z_near = 0.1;
    let z_far = 50.;
    let tetra = Object::load_obj("model/tetrahedron.obj")
        .unwrap()
        .model(transform::model(0., 0., 0., 1.));
    let spot = Object::load_obj("model/spot_triangulated_good.obj")
        .unwrap()
        .model(transform::model(0., 0., 0., 2.5));
    let cube = Object::load_obj("model/cube.obj")
        .unwrap()
        .model(transform::model(-20., 0., 0., 1.0));
    let objects = vec![tetra, spot, cube];

    let eye_pos = Vec3::new(28., 21., 35.34);
    let angle_alpha = 40.;
    let angle_beta = -10.;

    {
        let mut rst = Rasterizer::new(width, height, EmptyShader);

        rst.view(transform::view(eye_pos, angle_alpha, angle_beta))
            .projection(transform::perspective(45., 1., z_near, z_far));
        c.bench_function("rasterize_empty", |b| {
            b.iter(|| {
                for obj in &objects {
                    rst.draw(obj);
                }
            });
        });
    }

    {
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

        let phong_shader = PhongShader::new(
            eye_pos,
            lights,
            diffuse_coeff,
            amb_coeff,
            amb_intensity,
            spec_coeff,
            spec_exp,
        );

        let mut rst = Rasterizer::new(width, height, phong_shader);

        rst.view(transform::view(eye_pos, angle_alpha, angle_beta))
            .projection(transform::perspective(45., 1., z_near, z_far));

        c.bench_function("rasterize_phong", |b| {
            b.iter(|| {
                for obj in &objects {
                    rst.draw(obj);
                }
            });
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
