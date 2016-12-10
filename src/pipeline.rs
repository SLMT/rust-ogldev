
use cgmath::{Vector3, Matrix, Matrix4};
use PersProjInfo;

pub struct Pipeline {
    scale: Vector3<f32>,
    world_pos: Vector3<f32>,
    rotate_info: Vector3<f32>,

    pers_proj_info: PersProjInfo,

    w_transformation: Matrix4<f32>,
    wp_transformation: Matrix4<f32>
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            scale: Vector3::new(1.0, 1.0, 1.0),
            world_pos: Vector3::new(0.0, 0.0, 0.0),
            rotate_info: Vector3::new(0.0, 0.0, 0.0),
            pers_proj_info: PersProjInfo::default(),
            w_transformation: Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ).transpose(),
            wp_transformation: Matrix4::new(
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

    pub fn set_perspective_proj(&mut self, fov: f32, width: f32, height: f32, z_near: f32, z_far: f32) {
        self.pers_proj_info = PersProjInfo {
            fov: fov,
            width: width,
            height: height,
            z_near: z_near,
            z_far: z_far
        }
    }

    pub fn get_world_trans(&mut self) -> Matrix4<f32> {
        let scale_trans = self.init_scale_transform();
        let rotate_trans = self.init_rotate_transform();
        let translation_trans = self.init_translation_transform();

        self.w_transformation = translation_trans * rotate_trans * scale_trans;
        self.w_transformation
    }

    pub fn get_wp_trans(&mut self) -> Matrix4<f32> {
        self.get_world_trans();
        let pers_proj_trans = self.init_pers_proj_transform();

        self.wp_transformation = pers_proj_trans * self.w_transformation;
        self.wp_transformation
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

    fn init_pers_proj_transform(&self) -> Matrix4<f32> {
        let p = &self.pers_proj_info;
        let ar = p.width / p.height;
        let z_range = p.z_near - p.z_far;
        let tan_half_fov = (p.fov / 2.0).to_radians().tan();

        Matrix4::new(
            1.0 / (tan_half_fov * ar), 0.0, 0.0, 0.0,
            0.0, 1.0 / tan_half_fov, 0.0, 0.0,
            0.0, 0.0, (-p.z_near - p.z_far) / z_range, 2.0 * p.z_far * p.z_near / z_range,
            0.0, 0.0, 1.0, 0.0,
        ).transpose()
    }
}
