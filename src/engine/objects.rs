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

// Component to identify game objects
#[derive(Component)]
pub struct GameObjectId {
    pub id: u32,
    pub name: String,
    pub shape_type: ShapeType,
    pub created_at: f64, // timestamp
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
    pub custom_name: Option<String>, // Allow custom naming
}

// Improved GameObject struct
#[derive(Debug, Clone)]
pub struct GameObject {
    pub id: u32,
    pub name: String,
    pub entity: Entity,
    pub shape_type: ShapeType,
    pub position: Vec3,
    pub created_at: f64,
}

#[derive(Resource, Default)]
pub struct GameObjectManager {
    pub objects: Vec<GameObject>,
    pub next_id: u32,
}

impl GameObjectManager {
    pub fn add_object(
        &mut self,
        entity: Entity,
        shape_type: ShapeType,
        position: Vec3,
        custom_name: Option<String>,
        timestamp: f64,
    ) -> u32 {
        let id = self.next_id;
        let name = custom_name.unwrap_or_else(|| format!("{} {}", shape_type.display_name(), id));

        let game_object = GameObject {
            id,
            name: name.clone(),
            entity,
            shape_type,
            position,
            created_at: timestamp,
        };

        self.objects.push(game_object);
        self.next_id += 1;

        info!("Added game object: {} (ID: {}) at {:?}", name, id, position);
        id
    }

    pub fn remove_object(&mut self, entity: Entity) -> Option<GameObject> {
        if let Some(index) = self.objects.iter().position(|obj| obj.entity == entity) {
            let removed = self.objects.remove(index);
            info!("Removed game object: {} (ID: {})", removed.name, removed.id);
            Some(removed)
        } else {
            None
        }
    }

    pub fn get_object_by_entity(&self, entity: Entity) -> Option<&GameObject> {
        self.objects.iter().find(|obj| obj.entity == entity)
    }

    pub fn get_object_by_id(&self, id: u32) -> Option<&GameObject> {
        self.objects.iter().find(|obj| obj.id == id)
    }

    pub fn get_objects_by_type(&self, shape_type: ShapeType) -> Vec<&GameObject> {
        self.objects
            .iter()
            .filter(|obj| obj.shape_type == shape_type)
            .collect()
    }

    pub fn list_objects(&self) -> Vec<String> {
        self.objects
            .iter()
            .map(|obj| {
                format!(
                    "{} (ID: {}, Type: {})",
                    obj.name,
                    obj.id,
                    obj.shape_type.display_name()
                )
            })
            .collect()
    }
}

pub fn spawn_entity_system(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntityEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_manager: ResMut<GameObjectManager>,
    time: Res<Time>,
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

        let entity = commands
            .spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(event.position),
                RigidBody::Dynamic,
                collider,
                Restitution::coefficient(0.7),
            ))
            .id();

        // Add the GameObject ID component and register with manager
        let object_id = game_manager.add_object(
            entity,
            event.shape_type,
            event.position,
            event.custom_name.clone(),
            time.elapsed_secs_f64(),
        );

        // Add the GameObjectId component to the entity
        commands.entity(entity).insert(GameObjectId {
            id: object_id,
            name: event
                .custom_name
                .clone()
                .unwrap_or_else(|| format!("{} {}", event.shape_type.display_name(), object_id)),
            shape_type: event.shape_type,
            created_at: time.elapsed_secs_f64(),
        });
    }
}

// System to handle entity removal and cleanup
pub fn cleanup_destroyed_entities_system(
    mut removed: RemovedComponents<GameObjectId>,
    mut game_manager: ResMut<GameObjectManager>,
) {
    for entity in removed.read() {
        game_manager.remove_object(entity);
    }
}

// UI system for shape selection
pub fn shape_selection_ui(
    mut selected_shape: ResMut<SelectedShape>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_manager: Res<GameObjectManager>,
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

    // Debug: Print all objects with 'L' key
    if keyboard_input.just_pressed(KeyCode::KeyL) {
        info!("Current game objects:");
        for object_info in game_manager.list_objects() {
            info!("  {}", object_info);
        }
        info!("Total objects: {}", game_manager.objects.len());
    }
}

// System to update object positions (useful for tracking moving objects)
pub fn update_object_positions_system(
    mut game_manager: ResMut<GameObjectManager>,
    query: Query<(Entity, &Transform), (With<GameObjectId>, Changed<Transform>)>,
) {
    for (entity, transform) in query.iter() {
        if let Some(obj) = game_manager
            .objects
            .iter_mut()
            .find(|obj| obj.entity == entity)
        {
            obj.position = transform.translation;
        }
    }
}
