#[macro_use]
extern crate glium;
extern crate cgmath;

use glium::{DisplayBuild, Surface, VertexBuffer, Program};
use glium::glutin::{Event, WindowBuilder};
use glium::index::{IndexBuffer, PrimitiveType};
use glium::backend::glutin_backend::GlutinFacade;
use cgmath::{Matrix, Matrix4};

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

        uniform mat4 gWorld;

        out vec4 color;

        void main() {
            gl_Position = gWorld * vec4(position, 1.0);
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
        index_buffer: &IndexBuffer<u32>, program: &Program, scale: f32) {

    // Build the transform matrix
    // Note that the matrix is in column-major order
    // so you need to transpose it for OpenGL
    let sin_scale = scale.sin();
    let cos_scale = scale.cos();
    let world: [[f32; 4]; 4] = Matrix4::new(
        cos_scale, 0.0, -sin_scale, 0.0,
        0.0, 1.0, 0.0, 0.0,
        sin_scale, 0.0, cos_scale, 0.0,
        0.0, 0.0, 0.0, 1.0
    ).transpose().into();

    let uniform = uniform!{ gWorld: world };

    let mut frame = display.draw();

    frame.clear_color(0.0, 0.0, 0.0, 0.0);

    frame.draw(vertex_buffer, index_buffer, program,
        &uniform, &Default::default()).unwrap();

    frame.finish().unwrap();
}

fn main() {
    // Set up and create a window
    let display = WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_srgb(Some(true))
        .with_title("Tutorial 10")
        .build_glium()
        .unwrap();

    // Create a vertex buffer and indices
    let vertex_buffer = create_vertex_buffer(&display);
    let index_buffer = create_index_buffer(&display);

    // Create a shader program
    let program = create_shaders(&display);

    // Main loop
    let mut scale: f32 = 0.0;
    loop {
        // Change the scale
        // (I use a smaller factor than the one used in the original source code
        // since the original factor is too large in my case)
        scale += 0.0001;

        // Render
        render_scene(&display, &vertex_buffer, &index_buffer, &program, scale);

        // Handle events
        for event in display.poll_events() {
            match event {
                Event::Closed => return,
                _ => ()
            }
        }
    }
}
