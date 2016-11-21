#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Surface, VertexBuffer, Program};
use glium::glutin::{Event, WindowBuilder};
use glium::index::{NoIndices, PrimitiveType};
use glium::backend::glutin_backend::GlutinFacade;

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
        Vertex { position: [1.0, -1.0, 0.0] },
        Vertex { position: [0.0, 1.0, 0.0] }
    ];
    let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
    vertex_buffer
}

fn create_shaders(display: &GlutinFacade) -> Program {
    let vertex_shader_src = r#"
        #version 330

        layout (location = 0) in vec3 position;

        uniform float gScale;

        void main() {
            gl_Position = vec4(gScale * position.x, gScale * position.y, position.z, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    Program::from_source(display,
        vertex_shader_src, fragment_shader_src, None).unwrap()
}

fn render_scene(display: &GlutinFacade, vertex_buffer: &VertexBuffer<Vertex>, program: &Program,
        scale: f32) {

    let uniform = uniform!{ gScale: scale };

    let mut frame = display.draw();

    frame.clear_color(0.0, 0.0, 0.0, 0.0);

    frame.draw(vertex_buffer, &NoIndices(PrimitiveType::TrianglesList), program,
        &uniform, &Default::default()).unwrap();

    frame.finish().unwrap();
}

fn main() {
    // Set up and create a window
    let display = WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_srgb(Some(true))
        .with_title("Tutorial 03")
        .build_glium()
        .unwrap();

    // Create a vertex buffer and indices
    let vertex_buffer = create_vertex_buffer(&display);

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
        render_scene(&display, &vertex_buffer, &program, scale.sin());

        // Handle events
        for event in display.poll_events() {
            match event {
                Event::Closed => return,
                _ => ()
            }
        }
    }
}
