
use cgmath::{Vector3, Matrix, Matrix4};

pub struct Pipeline {
    scale: Vector3<f32>,
    world_pos: Vector3<f32>,
    rotate_info: Vector3<f32>,
    transformation: Matrix4<f32>
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            scale: Vector3::new(1.0, 1.0, 1.0),
            world_pos: Vector3::new(0.0, 0.0, 0.0),
            rotate_info: Vector3::new(0.0, 0.0, 0.0),
            transformation: Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ).transpose()
        }
    }

    pub fn scale(&mut self, scale_x: f32, scale_y: f32, scale_z: f32) {
        self.scale.x = scale_x;
        self.scale.y = scale_y;
        self.scale.z = scale_z;
    }

    pub fn world_pos(&mut self, x: f32, y: f32, z: f32) {
        self.world_pos.x = x;
        self.world_pos.y = y;
        self.world_pos.z = z;
    }

    pub fn rotate(&mut self, rotate_x: f32, rotate_y: f32, rotate_z: f32) {
        self.rotate_info.x = rotate_x;
        self.rotate_info.y = rotate_y;
        self.rotate_info.z = rotate_z;
    }

    pub fn get_trans(&mut self) -> Matrix4<f32> {
        let scale_trans = self.init_scale_transform();
        let rotate_trans = self.init_rotate_transform();
        let translation_trans = self.init_translation_transform();

        self.transformation = translation_trans * rotate_trans * scale_trans;
        self.transformation
    }

    fn init_scale_transform(&self) -> Matrix4<f32> {
        Matrix4::new(
            self.scale.x, 0.0, 0.0, 0.0,
            0.0, self.scale.y, 0.0, 0.0,
            0.0, 0.0, self.scale.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        ).transpose()
    }

    fn init_rotate_transform(&self) -> Matrix4<f32> {
        let x = self.rotate_info.x.to_radians();
        let y = self.rotate_info.y.to_radians();
        let z = self.rotate_info.z.to_radians();

        let rx = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, x.cos(), -x.sin(), 0.0,
            0.0, x.sin(), x.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        ).transpose();

        let ry = Matrix4::new(
            y.cos(), 0.0, -y.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            y.sin(), 0.0, y.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        ).transpose();

        let rz = Matrix4::new(
            z.cos(), -z.sin(), 0.0, 0.0,
            z.sin(), z.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ).transpose();

        rz * ry * rx
    }

    fn init_translation_transform(&self) -> Matrix4<f32> {
        Matrix4::new(
            1.0, 0.0, 0.0, self.world_pos.x,
            0.0, 1.0, 0.0, self.world_pos.y,
            0.0, 0.0, 1.0, self.world_pos.z,
            0.0, 0.0, 0.0, 1.0
        ).transpose()
    }
}
