use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

//mod mallard;

fn start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    for n in 1..10 {
        // Cube
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0,1.0,1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Collider::cuboid(0.5,0.5,0.5),
            RigidBody::Dynamic,
            Restitution::coefficient(0.0),
            Transform::from_xyz(0.0,n as f32,0.0)
        ));
    }
    // Floor
    commands.spawn((
        Collider::cuboid(5.0,0.5,5.0),
        Mesh3d(meshes.add(Cuboid::new(10.0,1.0,10.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::IDENTITY
    ));
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0)
    ));
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn update_system() {
    //println!("frame!");
}

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin::default(),
    ))
    .add_systems(Startup,start_system)
    .add_systems(Update,update_system)
    .run();
}