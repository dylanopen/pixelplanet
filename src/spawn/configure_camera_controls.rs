use bevy::{ecs::system::ResMut, input::mouse::MouseButton};
use bevy_map_camera::{CameraControllerSettings, inputs::InputButton};

pub fn configure_camera_controls(
    mut camera_settings: ResMut<CameraControllerSettings>,
) {
    camera_settings.buttons.pan = vec![InputButton::Mouse(MouseButton::Right)];
    camera_settings.buttons.rotate = vec![InputButton::Mouse(MouseButton::Middle)];
    camera_settings.buttons.rotate_alt = None;
}
