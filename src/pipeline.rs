
use cgmath::{Vector3, Matrix, Matrix4};
use graphical_math;
use graphical_math::{PersProjInfo, Camera};

fn default_matrix() -> Matrix4<f32> {
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ).transpose()
}

pub struct Pipeline {
    scale: Vector3<f32>,
    world_pos: Vector3<f32>,
    rotate_info: Vector3<f32>,

    pers_proj_info: PersProjInfo,
    camera: Camera,

    w_transformation: Matrix4<f32>,
    v_transformation: Matrix4<f32>,
    p_transformation: Matrix4<f32>,
    wp_transformation: Matrix4<f32>,
    wvp_transformation: Matrix4<f32>
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            scale: Vector3::new(1.0, 1.0, 1.0),
            world_pos: Vector3::new(0.0, 0.0, 0.0),
            rotate_info: Vector3::new(0.0, 0.0, 0.0),
            pers_proj_info: PersProjInfo::default(),
            camera: Camera {
                pos: Vector3::new(0.0, 0.0, 0.0),
                target: Vector3::new(0.0, 0.0, 1.0),
                up: Vector3::new(0.0, 1.0, 0.0)
            },
            w_transformation: default_matrix(),
            v_transformation: default_matrix(),
            p_transformation: default_matrix(),
            wp_transformation: default_matrix(),
            wvp_transformation: default_matrix()
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
        };
    }

    pub fn set_camera(&mut self, pos: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>) {
        self.camera = Camera {
            pos: pos,
            target: target,
            up: up
        };
    }

    pub fn get_world_trans(&mut self) -> Matrix4<f32> {
        let scale_trans = graphical_math::init_scale_transform(self.scale.x, self.scale.y, self.scale.z);
        let rotate_trans = graphical_math::init_rotate_transform(self.rotate_info.x, self.rotate_info.y, self.rotate_info.z);
        let translation_trans = graphical_math::init_translation_transform(self.world_pos.x, self.world_pos.y, self.world_pos.z);

        self.w_transformation = translation_trans * rotate_trans * scale_trans;
        self.w_transformation
    }

    pub fn get_view_trans(&mut self) -> Matrix4<f32> {
        let camera_translation_trans = graphical_math::init_translation_transform(
            -self.camera.pos.x, -self.camera.pos.y, -self.camera.pos.z);
        let camera_rotate_trans = graphical_math::init_camera_transform(
            self.camera.target, self.camera.up);

        self.v_transformation = camera_rotate_trans * camera_translation_trans;
        self.v_transformation
    }

    pub fn get_project_trans(&mut self) -> Matrix4<f32> {
        self.p_transformation = graphical_math::init_pers_proj_transform(self.pers_proj_info);
        self.p_transformation
    }

    pub fn get_wp_trans(&mut self) -> Matrix4<f32> {
        self.get_world_trans();

        let pers_proj_trans = graphical_math::init_pers_proj_transform(self.pers_proj_info);

        self.wp_transformation = pers_proj_trans * self.w_transformation;
        self.wp_transformation
    }

    pub fn get_wvp_trans(&mut self) -> Matrix4<f32> {
        self.get_world_trans();
        self.get_view_trans();
        self.get_project_trans();

        self.wvp_transformation = self.p_transformation * self.v_transformation * self.w_transformation;
        self.wvp_transformation
    }
}
