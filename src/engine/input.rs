use crate::engine::objects::{SelectedShape, ShapeType, SpawnEntityEvent};
use bevy::prelude::*;

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    selected_shape: Res<SelectedShape>,
    mut spawn_events: EventWriter<SpawnEntityEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let x = (rand::random::<f32>() - 0.5) * 10.0;
        let z = (rand::random::<f32>() - 0.5) * 10.0;
        spawn_events.send(SpawnEntityEvent {
            position: Vec3::new(x, 4.0, z),
            shape_type: selected_shape.shape_type,
            custom_name: Some("bob".to_string()),
        });
    }
}
