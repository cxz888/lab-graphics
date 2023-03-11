use lab_graphics::{rasterizer::Rasterizer, shaders::EmptyShader};

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 700;
const HEIGHT: usize = 700;

fn main() {
    let mut rst = Rasterizer::new(WIDTH, HEIGHT, EmptyShader);

    let mut window = Window::new("Graphic Lab", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    // 限制至多为 60fps
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        rst.clear();

        window
            .update_with_buffer(rst.data(), WIDTH, HEIGHT)
            .unwrap();
    }
}
