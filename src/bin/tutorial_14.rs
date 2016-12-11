#[macro_use]
extern crate glium;
extern crate ogldev;

use glium::{DisplayBuild, Surface, VertexBuffer, Program};
use glium::glutin::{Event, WindowBuilder};
use glium::index::{IndexBuffer, PrimitiveType};
use glium::backend::glutin_backend::GlutinFacade;

use ogldev::{Camera, Pipeline};

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;

// Represent a 3D vertex
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3]
}

// Let glium implement Vertex for us
implement_vertex!(Vertex, position);

fn create_vertex_buffer(display: &GlutinFacade) -> VertexBuffer<Vertex> {
    let vertices = vec![
        Vertex { position: [-1.0, -1.0, 0.0] },
        Vertex { position: [0.0, -1.0, 1.0] },
        Vertex { position: [1.0, -1.0, 0.0] },
        Vertex { position: [0.0, 1.0, 0.0] }
    ];
    let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
    vertex_buffer
}

fn create_index_buffer(display: &GlutinFacade) -> IndexBuffer<u32> {
    let indcies = vec![
        0, 3, 1,
        1, 3, 2,
        2, 3, 0,
        0, 1, 2
    ];
    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &indcies).unwrap();
    index_buffer
}

fn create_shaders(display: &GlutinFacade) -> Program {
    let vertex_shader_src = r#"
        #version 330

        layout (location = 0) in vec3 position;

        uniform mat4 gWVP;

        out vec4 color;

        void main() {
            gl_Position = gWVP * vec4(position, 1.0);
            color = vec4(clamp(position, 0.0, 1.0), 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330

        in vec4 color;

        out vec4 fragColor;

        void main() {
            fragColor = color;
        }
    "#;

    Program::from_source(display,
        vertex_shader_src, fragment_shader_src, None).unwrap()
}

fn render_scene(display: &GlutinFacade, vertex_buffer: &VertexBuffer<Vertex>,
        index_buffer: &IndexBuffer<u32>, program: &Program, camera: &Camera, scale: f32) {

    // Create a Pipeline
    let mut pipeline = Pipeline::new();
    pipeline.rotate(0.0, scale, 0.0);
    pipeline.world_pos(0.0, 0.0, 3.0);
    pipeline.set_camera(camera.get_pos(), camera.get_target(), camera.get_up());
    pipeline.set_perspective_proj(60.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32, 1.0, 100.0);

    // Set the uniform matrix
    let wvp: [[f32; 4]; 4] = pipeline.get_wvp_trans().into();
    let uniform = uniform!{ gWVP: wvp };

    // Drawing
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    frame.draw(vertex_buffer, index_buffer, program,
        &uniform, &Default::default()).unwrap();
    frame.finish().unwrap();
}

fn main() {
    // Set up and create a window
    let display = WindowBuilder::new()
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_srgb(Some(true))
        .with_title("Tutorial 14")
        .build_glium()
        .unwrap();

    // Create a vertex buffer and indices
    let vertex_buffer = create_vertex_buffer(&display);
    let index_buffer = create_index_buffer(&display);

    // Create a shader program
    let program = create_shaders(&display);

    // Create a camera
    let mut camera = Camera::default();

    // Main loop
    let mut scale: f32 = 0.0;
    loop {
        // Change the scale
        // (I use a smaller factor than the one used in the original source code
        // since the original factor is too large in my case)
        scale += 0.01;

        // Render
        render_scene(&display, &vertex_buffer, &index_buffer, &program, &camera, scale);

        // Handle events
        for event in display.poll_events() {
            match event {
                Event::Closed => return,
                Event::KeyboardInput(_, _, Some(key)) => {
                    camera.on_key_board(key);
                },
                _ => ()
            }
        }
    }
}
