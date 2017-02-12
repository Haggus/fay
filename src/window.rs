use gfx;
use gfx::traits::FactoryExt;
use gfx::{Device, Factory, texture};
use gfx_window_glutin;
use glutin;
use glutin::{Event, ElementState, VirtualKeyCode};
use cgmath::{Matrix4, Point3, Vector3, Transform, ortho};

// TODO: move this to a separate file
pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

static VERTEX_SRC: &'static [u8] = b"
    #version 150 core

    in vec2 a_Pos;
    in vec2 a_Uv;

    uniform b_VsLocals {
        mat4 u_Model;
        mat4 u_View;
        mat4 u_Proj;
    };

    out vec2 v_Uv;

    void main() {
        v_Uv = a_Uv;
        gl_Position = u_Proj * u_View * u_Model * vec4(a_Pos, 0.0, 1.0);
    }
";

static FRAGMENT_SRC: &'static [u8] = b"
    #version 150 core

    uniform sampler2D t_Tex;
    in vec2 v_Uv;

    out vec4 Target0;

    void main() {
        vec3 color = texture(t_Tex, v_Uv).rgb;
        Target0 = vec4(color, 1.0);
    }
";

gfx_defines!{
    vertex Vertex{
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant ProjectionData {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] = "u_Proj",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        projection_cb: gfx::ConstantBuffer<ProjectionData> = "b_VsLocals",
        tex: gfx::TextureSampler<[f32; 4]> = "t_Tex",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

impl Vertex {
    fn new(p: [f32; 2], u: [f32; 2]) -> Vertex {
        Vertex {
            pos: p,
            uv: u,
        }
    }
}

// Larger red dots
const L0_DATA: [[u8; 4]; 16] = [
    [ 0x00, 0x00, 0x00, 0x00 ], [ 0x00, 0x00, 0x00, 0x00 ], [ 0x00, 0x00, 0x00, 0x00 ], [ 0x00, 0x00, 0x00, 0x00 ],
    [ 0x00, 0x00, 0x00, 0x00 ], [ 0xc0, 0x00, 0x00, 0x00 ], [ 0xc0, 0x00, 0x00, 0x00 ], [ 0x00, 0x00, 0x00, 0x00 ],
    [ 0x00, 0x00, 0x00, 0x00 ], [ 0xc0, 0x00, 0x00, 0x00 ], [ 0xc0, 0x00, 0x00, 0x00 ], [ 0x00, 0x00, 0x00, 0x00 ],
    [ 0x00, 0x00, 0x00, 0x00 ], [ 0x00, 0x00, 0x00, 0x00 ], [ 0x00, 0x00, 0x00, 0x00 ], [ 0x00, 0x00, 0x00, 0x00 ],
];

// Uniform green
const L1_DATA: [[u8; 4]; 4] = [
    [ 0x00, 0xc0, 0x00, 0x00 ], [ 0x00, 0xc0, 0x00, 0x00 ],
    [ 0x00, 0xc0, 0x00, 0x00 ], [ 0x00, 0xc0, 0x00, 0x00 ],
];

// Uniform blue
const L2_DATA: [[u8; 4]; 1] = [ [ 0x00, 0x00, 0xc0, 0x00 ] ];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub struct Window {
    width: u32,
    height: u32,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        Window {
            width: width,
            height: height,
        }
    }

    pub fn run(&self) {
        let gl_version = glutin::GlRequest::GlThenGles {
            opengl_version: (3, 2),
            opengles_version: (2, 0),
        };

        let builder = glutin::WindowBuilder::new()
            .with_dimensions(self.width, self.height)
            .with_title("fay")
            .with_gl(gl_version);

        println!("gl_version {:?}", gl_version);

        let (window, mut device, mut factory, main_color, mut main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

        let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        let pso = factory.create_pipeline_simple(
            VERTEX_SRC, FRAGMENT_SRC,
            pipe::new()
        ).expect("Failed to compile shaders");

        let vertex_data = [
            Vertex::new([ 0.0,  0.0], [ 0.0,  0.0]),
            Vertex::new([ 1.0,  0.0], [50.0,  0.0]),
            Vertex::new([ 1.0,  1.0], [50.0, 50.0]),

            Vertex::new([ 1.0,  1.0], [ 0.0,  0.0]),
            Vertex::new([ 0.0,  1.0], [ 0.0, 50.0]),
            Vertex::new([ 0.0,  0.0], [50.0, 50.0]),
        ];
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, ());

        let (_, texture_view) = factory.create_texture_immutable::<ColorFormat>(
                texture::Kind::D2(4, 4, texture::AaMode::Single),
                &[&L0_DATA, &L1_DATA, &L2_DATA]
                ).unwrap();

            let sampler = factory.create_sampler(texture::SamplerInfo::new(
                texture::FilterMethod::Trilinear,
                texture::WrapMode::Tile,
        ));

        let mut data = pipe::Data {
            vbuf: vertex_buffer,
            projection_cb: factory.create_constant_buffer(1),
            tex: (texture_view, sampler),
            out: main_color,
        };

        let view: Matrix4<f32> = Transform::look_at(
            Point3::new(0.0, 0.0, 8.0),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::unit_y(),
        );

        let mut project = ProjectionData {
            model: Matrix4::one().into(),
            view: view.into(),
            proj: ortho(-1.0, 1.0, -1.0, 1.0, -1.0, 8.0).into(),
        };

        'main: loop {
            for event in window.poll_events() {
                match event {
                    Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => break 'main,
                    Event::Closed => break 'main,
                    Event::Resized(_width, _height) => {
                        let ratio = _width as f32 / _height as f32;
                        project.proj = ortho(-1.0 * ratio, 1.0 * ratio, -1.0, 1.0, -1.0, 8.0).into();

                        gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                    },
                    _ => (),
                }
            }

            encoder.update_constant_buffer(&data.projection_cb, &project);

            encoder.clear(&data.out, CLEAR_COLOR);

            encoder.draw(&slice, &pso, &data);
            encoder.flush(&mut device);

            window.swap_buffers().unwrap();
            device.cleanup();
        }
    }
}
