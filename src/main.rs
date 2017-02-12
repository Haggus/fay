extern crate glutin;
#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate cgmath;

mod window;

use window::Window;

fn main() {
    let window = Window::new(800, 600);

    window.run();
}
