
use cgmath::{InnerSpace, Vector3};
use glium::glutin::VirtualKeyCode;

const STEP_SCALE: f32 = 0.2;

pub struct Camera {
    pos: Vector3<f32>,
    target: Vector3<f32>,
    up: Vector3<f32>
}

impl Camera {

    pub fn default() -> Camera {
        Camera {
            pos: Vector3::new(0.0, 0.0, 0.0),
            target: Vector3::new(0.0, 0.0, 1.0),
            up: Vector3::new(0.0, 1.0, 0.0)
        }
    }

    pub fn new(pos: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>) -> Camera {
        Camera {
            pos: pos,
            target: target,
            up: up
        }
    }

    pub fn get_pos(&self) -> Vector3<f32> {
        self.pos
    }

    pub fn get_target(&self) -> Vector3<f32> {
        self.target
    }

    pub fn get_up(&self) -> Vector3<f32> {
        self.up
    }

    pub fn on_key_board(&mut self, key: VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::Up => {
                self.pos += self.target * STEP_SCALE;
                true
            },
            VirtualKeyCode::Down => {
                self.pos -= self.target * STEP_SCALE;
                true
            },
            VirtualKeyCode::Left => {
                let mut left = self.target.cross(self.up);
                left = left.normalize();
                left *= STEP_SCALE;
                self.pos += left;
                true
            },
            VirtualKeyCode::Right => {
                let mut right = self.up.cross(self.target);
                right = right.normalize();
                right *= STEP_SCALE;
                self.pos += right;
                true
            },
            _ => false
        }
    }
}
