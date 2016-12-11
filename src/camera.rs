
use cgmath::{Deg, InnerSpace, Vector2, Vector3, Matrix3};
use glium::glutin::VirtualKeyCode;

const STEP_SCALE: f32 = 0.2;
const MARGIN: i32 = 10;

pub struct Camera {
    // For View Transformation
    pos: Vector3<f32>,
    target: Vector3<f32>,
    up: Vector3<f32>,

    // For control the camera
    window_width: i32,
    window_height: i32,
    angle_h: f32,
    angle_v: f32,
    on_upper_edge: bool,
    on_lower_edge: bool,
    on_left_edge: bool,
    on_right_edge: bool,
    mouse_pos: Vector2<i32>
}

impl Camera {

    pub fn default(window_width: u32, window_height: u32) -> Camera {
        let mut camera = Camera {
            pos: Vector3::new(0.0, 0.0, 0.0),
            target: Vector3::new(0.0, 0.0, 1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            window_width: window_width as i32,
            window_height: window_height as i32,
            angle_h: 0.0,
            angle_v: 0.0,
            on_upper_edge: false,
            on_lower_edge: false,
            on_left_edge: false,
            on_right_edge: false,
            mouse_pos: Vector2::new(0, 0)
        };

        camera.init();
        camera
    }

    pub fn new(window_width: u32, window_height: u32, pos: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>) -> Camera {
        let mut camera = Self::default(window_width, window_height);
        camera.pos = pos;
        camera.target = target.normalize();
        camera.up = up;

        camera.init();
        camera
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

    pub fn on_mouse(&mut self, x: i32, y: i32) {
        let delta_x = x - self.mouse_pos.x;
        let delta_y = y - self.mouse_pos.y;

        self.mouse_pos.x = x;
        self.mouse_pos.y = y;

        self.angle_h += (delta_x as f32) / 20.0;
        self.angle_v += (delta_y as f32) / 20.0;

        // Horizontal edge detection
        if delta_x == 0 {
            if x <= MARGIN {
                self.on_left_edge = true;
            } else if x >= (self.window_width - MARGIN) {
                self.on_right_edge = true;
            }
        } else {
            self.on_left_edge = false;
            self.on_right_edge = false;
        }

        // Vertical edge detection
        if delta_y == 0 {
            if y <= MARGIN {
                self.on_upper_edge = true;
            } else if y >= (self.window_height - MARGIN) {
                self.on_lower_edge = true;
            }
        } else {
            self.on_upper_edge = false;
            self.on_lower_edge = false;
        }

        self.update();
    }

    pub fn on_render(&mut self) {
        let mut should_update = false;

        if self.on_left_edge {
            self.angle_h -= 0.1;
            should_update = true;
        } else if self.on_right_edge {
            self.angle_h += 0.1;
            should_update = true;
        }

        if self.on_upper_edge {
            if self.angle_v > -90.0 {
                self.angle_v -= 0.1;
                should_update = true;
            }
        } else if self.on_lower_edge {
            if self.angle_v < 90.0 {
                self.angle_v += 0.1;
                should_update = true;
            }
        }

        if should_update {
            self.update();
        }
    }

    fn init(&mut self) {
        let h_target = Vector3::new(self.target.x, 0.0, self.target.z).normalize();

        if h_target.z >= 0.0 {
            if h_target.x >= 0.0 {
                self.angle_h = 360.0 - h_target.z.asin().to_degrees();
            } else {
                self.angle_h = 180.0 - h_target.z.asin().to_degrees();
            }
        } else {
            if h_target.x >= 0.0 {
                self.angle_h = (-h_target.z).asin().to_degrees();
            } else {
                self.angle_h = 90.0 + (-h_target.z).asin().to_degrees();
            }
        }

        self.angle_v = - h_target.y.asin().to_degrees();

        // NOTE: The flags for edges have been initialized in default()

        self.mouse_pos.x = self.window_width / 2;
        self.mouse_pos.y = self.window_height / 2;
    }

    fn update(&mut self) {
        let v_axis = Vector3::new(0.0, 1.0, 0.0);

        // Rotate the view vector by the horizontal angle around the vertical axis
        let mut view = Vector3::new(1.0, 0.0, 0.0);
        // There is something different here. Crate cgmath provides another way to rotate a vector
        // using a transfromation matrix. So this looks different from the original tutorial
        let mut rotate_matrix = Matrix3::from_axis_angle(v_axis, Deg(self.angle_h));
        view = rotate_matrix * view;
        view = view.normalize();

        // Rotate the view vector by the vertical angle around the horizontal axis
        let mut h_axis = v_axis.cross(view);
        h_axis = h_axis.normalize();
        rotate_matrix = Matrix3::from_axis_angle(h_axis, Deg(self.angle_v));
        view = rotate_matrix * view;
        view = view.normalize();

        self.target = view;
        self.up = self.target.cross(h_axis).normalize();
    }
}
