use crate::engine::input::handle_input;
use crate::engine::objects::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // Add custom events
        .add_event::<SpawnEntityEvent>()
        // Initialize resources
        .init_resource::<SelectedShape>()
        // Startup systems
        .add_systems(Startup, (setup_graphics, setup_physics))
        // Update systems
        .add_systems(
            Update,
            (handle_input, spawn_entity_system, shape_selection_ui),
        )
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create the ground
    let ground_mesh = meshes.add(Cuboid::new(200.0, 0.2, 200.0));
    let ground_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.5, 0.3),
        ..default()
    });

    commands.spawn((
        Mesh3d(ground_mesh),
        MeshMaterial3d(ground_material),
        Transform::from_xyz(0.0, -2.0, 0.0),
        Collider::cuboid(100.0, 0.1, 100.0),
        RigidBody::Fixed,
    ));

    // Create the initial bouncing ball
    let initial_mesh = meshes.add(Sphere::new(0.5));
    let initial_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });

    commands.spawn((
        Mesh3d(initial_mesh),
        MeshMaterial3d(initial_material),
        Transform::from_translation(Vec3::new(0.0, 4.0, 0.0)),
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Restitution::coefficient(0.7),
    ));
}
