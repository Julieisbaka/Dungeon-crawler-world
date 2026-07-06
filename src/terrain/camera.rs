#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera3d {
    pub eye: [f32; 3],
    pub target: [f32; 3],
    pub up: [f32; 3],
    pub fovy_radians: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera3d {
    pub fn follow_player(player_position: [f32; 3], cell_size: [f32; 3]) -> Self {
        let height = cell_size[2].max(1.0);
        Self {
            eye: [
                player_position[0] - cell_size[0] * 1.15,
                player_position[1] - cell_size[1] * 1.85,
                player_position[2] + height * 1.35,
            ],
            target: [
                player_position[0],
                player_position[1] + cell_size[1] * 0.65,
                player_position[2] + height * 0.45,
            ],
            up: [0.0, 0.0, 1.0],
            fovy_radians: 65.0_f32.to_radians(),
            z_near: 0.1,
            z_far: 900.0,
        }
    }

    pub fn view_projection_matrix(&self, aspect_ratio: f32) -> [[f32; 4]; 4] {
        let view = look_at_rh(self.eye, self.target, self.up);
        let projection = perspective_rh_zo(
            self.fovy_radians,
            aspect_ratio.max(0.1),
            self.z_near,
            self.z_far,
        );
        multiply_mat4(projection, view)
    }
}

fn perspective_rh_zo(fovy_radians: f32, aspect: f32, z_near: f32, z_far: f32) -> [[f32; 4]; 4] {
    let focal_length = 1.0 / (fovy_radians * 0.5).tan();
    [
        [focal_length / aspect, 0.0, 0.0, 0.0],
        [0.0, focal_length, 0.0, 0.0],
        [0.0, 0.0, z_far / (z_near - z_far), -1.0],
        [0.0, 0.0, (z_near * z_far) / (z_near - z_far), 0.0],
    ]
}

fn look_at_rh(eye: [f32; 3], target: [f32; 3], up: [f32; 3]) -> [[f32; 4]; 4] {
    let forward = normalize(subtract(target, eye));
    let side = normalize(cross(forward, up));
    let camera_up = cross(side, forward);

    [
        [side[0], camera_up[0], -forward[0], 0.0],
        [side[1], camera_up[1], -forward[1], 0.0],
        [side[2], camera_up[2], -forward[2], 0.0],
        [
            -dot(side, eye),
            -dot(camera_up, eye),
            dot(forward, eye),
            1.0,
        ],
    ]
}

fn multiply_mat4(left: [[f32; 4]; 4], right: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut out = [[0.0; 4]; 4];
    for column in 0..4 {
        for row in 0..4 {
            out[column][row] = left[0][row] * right[column][0]
                + left[1][row] * right[column][1]
                + left[2][row] * right[column][2]
                + left[3][row] * right[column][3];
        }
    }
    out
}

fn subtract(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

fn dot(left: [f32; 3], right: [f32; 3]) -> f32 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

fn cross(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}

fn normalize(value: [f32; 3]) -> [f32; 3] {
    let length = dot(value, value).sqrt();
    if length <= f32::EPSILON {
        [0.0, 0.0, 0.0]
    } else {
        [value[0] / length, value[1] / length, value[2] / length]
    }
}
