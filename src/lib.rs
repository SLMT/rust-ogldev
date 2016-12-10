extern crate cgmath;

// Re-export
pub use pipeline::*;

// Modules
mod pipeline;

#[derive(Default)]
struct PersProjInfo {
    fov: f32,
    width: f32,
    height: f32,
    z_near: f32,
    z_far: f32
}
