
use cgmath::{InnerSpace, Vector3, Matrix, Matrix4};

#[derive(Default, Clone, Copy)]
pub struct PersProjInfo {
    pub fov: f32,
    pub width: f32,
    pub height: f32,
    pub z_near: f32,
    pub z_far: f32
}

pub fn init_scale_transform(scale_x: f32, scale_y: f32, scale_z: f32) -> Matrix4<f32> {
    Matrix4::new(
        scale_x, 0.0, 0.0, 0.0,
        0.0, scale_y, 0.0, 0.0,
        0.0, 0.0, scale_z, 0.0,
        0.0, 0.0, 0.0, 1.0
    ).transpose()
}

pub fn init_rotate_transform(rotate_x: f32, rotate_y: f32, rotate_z: f32) -> Matrix4<f32> {
    let x = rotate_x.to_radians();
    let y = rotate_y.to_radians();
    let z = rotate_z.to_radians();

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

pub fn init_translation_transform(x: f32, y: f32, z: f32) -> Matrix4<f32> {
    Matrix4::new(
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0
    ).transpose()
}

pub fn init_pers_proj_transform(p: PersProjInfo) -> Matrix4<f32> {
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

pub fn init_camera_transform(target: Vector3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let mut n: Vector3<f32> = target;
    n = n.normalize();
    let mut u: Vector3<f32> = up;
    u = u.normalize();
    u = u.cross(n);
    let v: Vector3<f32> = n.cross(u);

    Matrix4::new(
        u.x, u.y, u.z, 0.0,
        v.x, v.y, v.z, 0.0,
        n.x, n.y, n.z, 0.0,
        0.0, 0.0, 0.0, 1.0
    ).transpose()
}

// == Matrix Tamplate ==
// Matrix4::new(
//     1.0, 0.0, 0.0, 0.0,
//     0.0, 1.0, 0.0, 0.0,
//     0.0, 0.0, 1.0, 0.0,
//     0.0, 0.0, 0.0, 1.0
// ).transpose()
