use bytemuck::{Pod, Zeroable};
use dungeon_crawler_world::save_game::SaveGame;
use glam::{Mat4, Vec3};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn identity() -> Self {
        Self {
            view_projection: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}

pub fn camera_for_save(save: &SaveGame, width: u32, height: u32) -> CameraUniform {
    let aspect = (width as f32 / height.max(1) as f32).max(0.1);
    let eye = Vec3::new(
        save.player.position.x,
        save.player.position.y + 48.0,
        save.player.position.z,
    );
    let yaw = save.player.look.yaw;
    let pitch = save.player.look.pitch;
    let look_direction = Vec3::new(
        yaw.sin() * pitch.cos(),
        pitch.sin(),
        -yaw.cos() * pitch.cos(),
    );
    let target = eye + look_direction * 500.0;
    let view = Mat4::look_at_rh(eye, target, Vec3::Y);
    let projection = Mat4::perspective_rh(78.0_f32.to_radians(), aspect, 0.1, 7_500.0);

    CameraUniform {
        view_projection: (opengl_to_wgpu_matrix() * projection * view).to_cols_array_2d(),
    }
}

fn opengl_to_wgpu_matrix() -> Mat4 {
    Mat4::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    ])
}
