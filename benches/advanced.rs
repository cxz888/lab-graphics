use lab_graphics::{color, shader::EmptyShader, Rasterizer};

use criterion::{criterion_group, criterion_main, Criterion};
use glam::vec2;

pub fn criterion_benchmark(c: &mut Criterion) {
    let width = 800;
    let height = 800;
    let mut rst = Rasterizer::new(width, height, EmptyShader);
    let vertices1 = [
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
    let mut vertices2 = Vec::with_capacity(100);
    vertices2.push(vec2(600., 300.));
    let mut x = 550.;
    let mut y = 600.;
    while x > 50. {
        vertices2.push(vec2(x, y));
        x -= 20.;
        y = 350.;
        vertices2.push(vec2(x, y));
        x -= 20.;
        y = 600.;
    }
    vertices2.push(vec2(50., 300.));
    y = 250.0;
    while x < 550. {
        vertices2.push(vec2(x, y));
        x += 20.;
        y = 50.;
        vertices2.push(vec2(x, y));
        x += 20.;
        y = 250.;
    }

    c.bench_function("draw polygon", |b| {
        b.iter(|| {
            rst.draw_polygon(&vertices1, color::RED);
            rst.draw_polygon(&vertices2, color::RED);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
