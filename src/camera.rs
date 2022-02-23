use bevy::prelude::*;
use lerp::Lerp;
use crate::inputs::PlayerData;

/// Moves and zooms the camera based on player positions
#[derive(Component)]
pub struct CameraController {
  pub max_scale: f32,
  pub min_scale: f32,
  pub current_scale: f32,
  pub max_distance: f32,
  pub y_padding: f32,
}

impl CameraController {
  pub fn set_scale_from_distance(&mut self, distance: f32) {

    let scale = if  distance > self.max_distance {
      1.0
    } else {
      distance/self.max_distance
    };
    
    if scale > self.max_scale {
      self.current_scale = self.max_scale;
    }

    if scale < self.min_scale {
      self.current_scale = self.min_scale;
    }

    if scale < self.max_scale && scale > self.min_scale {
      self.current_scale = scale;
    }
  }
}

impl Default for CameraController {
  fn default() -> Self {
    CameraController {
      max_scale: 0.6,
      min_scale: 0.5,
      current_scale: 0.5,
      max_distance: 650.0,
      y_padding: 50.0
    }
  }
}

#[doc(hidden)]
#[allow(unstable_name_collisions)]
pub fn set_camera_scale(
  mut query: Query<(&mut OrthographicProjection, &mut CameraController, &mut Transform)>,
  player_data: Res<PlayerData>
) {
  for (mut proj, mut controller, mut transform) in query.iter_mut() {
    controller.set_scale_from_distance(player_data.get_distance());
    let lerped_scale = proj.scale.lerp(controller.current_scale, 0.05);
    proj.scale = lerped_scale;
    
    let mid_point = player_data.get_mid_point();

    let new_pos = Vec3::new(mid_point.x, mid_point.y + controller.y_padding, transform.translation.z);
    transform.translation = new_pos;
  }
}
