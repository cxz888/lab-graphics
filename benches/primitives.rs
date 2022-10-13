use lab_graphics::{color, Rasterizer};

use criterion::{criterion_group, criterion_main, Criterion};
use nalgebra_glm::vec2;

pub fn criterion_benchmark(c: &mut Criterion) {
    let width = 800;
    let height = 800;
    let mut rst = Rasterizer::new(width, height);
    let from1 = vec2(0.0, 0.0);
    let to1 = vec2(300.0, 400.0);
    let from2 = vec2(3.0, 4.0);
    let to2 = vec2(50.0, 700.0);
    let from3 = vec2(0.0, 300.0);
    let to3 = vec2(400.0, 0.0);
    let color = color::RED;
    c.bench_function("draw_line DDA", |b| {
        b.iter(|| {
            rst.draw_line(from1, to1, color);
            rst.draw_line(from2, to2, color);
            rst.draw_line(from3, to3, color)
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
