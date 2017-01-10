extern crate glutin;
extern crate gfx;
extern crate gfx_window_glutin;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

fn main() {
    let gl_version = glutin::GlRequest::GlThenGles {
        opengl_version: (3, 2),
        opengles_version: (2, 0),
    };

    let builder = glutin::WindowBuilder::new()
        .with_title("fay")
        .with_gl(gl_version);

    let (window, mut device, mut factory, main_color, main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => return,
                _ => println!("event {:?}", event),
            }
        }
    }
}
