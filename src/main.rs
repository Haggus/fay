extern crate glutin;
#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

static VERTEX_SRC: &'static [u8] = b"
    #version 150 core

    in vec2 a_Pos;
    in vec3 a_Color;
    out vec4 v_Color;

    void main() {
        v_Color = vec4(a_Color, 1.0);
        gl_Position = vec4(a_Pos, 0.0, 1.0);
    }
";

static FRAGMENT_SRC: &'static [u8] = b"
    #version 150 core

    in vec4 v_Color;
    out vec4 Target0;

    void main() {
        Target0 = v_Color;
    }
";

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] }
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

fn main() {
    let gl_version = glutin::GlRequest::GlThenGles {
        opengl_version: (3, 2),
        opengles_version: (2, 0),
    };

    let builder = glutin::WindowBuilder::new()
        .with_title("fay")
        .with_gl(gl_version);

    println!("gl_version {:?}", gl_version);

    let (window, mut device, mut factory, main_color, mut main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        VERTEX_SRC, FRAGMENT_SRC,
        pipe::new()
    ).expect("Failed to compile shaders");

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };

    loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => return,
                glutin::Event::Resized(_width, _height) => {
                    gfx_window_glutin::update_views(&window,&mut data.out, &mut main_depth);
                },
                _ => println!("event {:?}", event),
            }
        }

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
