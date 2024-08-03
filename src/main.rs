use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Resource, Debug)]
struct Inc {
    pub counter: f32,
}

impl Inc {
    pub fn add(&mut self, v: f32) {
        self.counter += v;
        self.counter %= 1.0
    }
}

fn main() {
    App::new()
        .insert_resource(Inc { counter: 0.0 })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run()
}

fn update(mut q: Query<&mut Transform, With<Camera>>, mut counter: ResMut<Inc>) {
    counter.add(1.0);
    if counter.counter != 0.0 {
        return;
    }

    for mut camera in &mut q {
        let prev = camera.rotation;
        camera.rotate_around(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Quat::from_rotation_z(0.001),
        );
        println!("{:?}", camera.rotation - prev)
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(1.0, 0.8, 0.3).into()),
        ..default()
    });
    // // cubes
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(1.5, 0.5, 1.5),
    //     ..default()
    // });
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(1.5, 0.5, -1.5),
    //     ..default()
    // });
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(-1.5, 0.5, 1.5),
    //     ..default()
    // });
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(-1.5, 0.5, -1.5),
    //     ..default()
    // });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 10.0,
            scaling_mode: ScalingMode::FixedHorizontal(1.0),
            ..OrthographicProjection::default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::X),
        ..default()
    });
}
