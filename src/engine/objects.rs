use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeType {
    Ball,
    Cube,
    Capsule,
    Cylinder,
    Cone,
}

impl ShapeType {
    // Get all available shapes for dropdown
    pub fn all() -> Vec<ShapeType> {
        vec![
            ShapeType::Ball,
            ShapeType::Cube,
            ShapeType::Capsule,
            ShapeType::Cylinder,
            ShapeType::Cone,
        ]
    }

    // Get display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            ShapeType::Ball => "Ball",
            ShapeType::Cube => "Cube",
            ShapeType::Capsule => "Capsule",
            ShapeType::Cylinder => "Cylinder",
            ShapeType::Cone => "Cone",
        }
    }

    // Create collider with default parameters
    pub fn create_collider(&self) -> Collider {
        match self {
            ShapeType::Ball => Collider::ball(0.5),
            ShapeType::Cube => Collider::cuboid(0.5, 0.5, 0.5),
            ShapeType::Capsule => Collider::capsule_y(1.0, 0.3),
            ShapeType::Cylinder => Collider::cylinder(1.0, 0.5),
            ShapeType::Cone => Collider::cone(1.0, 0.5),
        }
    }

    // Create visual mesh
    pub fn create_mesh(&self, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        match self {
            ShapeType::Ball => meshes.add(Sphere::new(0.5)),
            ShapeType::Cube => meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            ShapeType::Capsule => meshes.add(Capsule3d::new(0.3, 2.0)),
            ShapeType::Cylinder => meshes.add(Cylinder::new(0.5, 2.0)),
            ShapeType::Cone => meshes.add(Cone::new(0.5, 2.0)),
        }
    }
}

// Resource to store currently selected shape
#[derive(Resource)]
pub struct SelectedShape {
    pub shape_type: ShapeType,
}

impl Default for SelectedShape {
    fn default() -> Self {
        Self {
            shape_type: ShapeType::Ball,
        }
    }
}

#[derive(Event)]
pub struct SpawnEntityEvent {
    pub position: Vec3,
    pub shape_type: ShapeType,
}

pub fn spawn_entity_system(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntityEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in spawn_events.read() {
        let collider = event.shape_type.create_collider();
        let mesh = event.shape_type.create_mesh(&mut meshes);
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>(),
            ),
            ..default()
        });

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(event.position),
            RigidBody::Dynamic,
            collider,
            Restitution::coefficient(0.7),
        ));
    }
}

// UI system for shape selection
pub fn shape_selection_ui(
    mut selected_shape: ResMut<SelectedShape>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Cycle through shapes with Tab key
    if keyboard_input.just_pressed(KeyCode::Tab) {
        let shapes = ShapeType::all();
        let current_index = shapes
            .iter()
            .position(|&s| s == selected_shape.shape_type)
            .unwrap_or(0);
        let next_index = (current_index + 1) % shapes.len();
        selected_shape.shape_type = shapes[next_index];
        info!(
            "Selected shape: {}",
            selected_shape.shape_type.display_name()
        );
    }
}
