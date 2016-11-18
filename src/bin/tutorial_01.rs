extern crate glium;

use glium::{DisplayBuild, Surface};
use glium::glutin::Event;

fn main() {
    // Set up and create a window
    let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
            .with_srgb(Some(true))
            .with_title("Tutorial 01")
            .build_glium()
            .unwrap();

    loop {
        // Draw the background
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        frame.finish().unwrap();

        // Handle events
        for event in display.poll_events() {
            match event {
                Event::Closed => return,
                _ => ()
            }
        }
    }
}
