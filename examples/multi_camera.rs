use bevy::app::App;
use bevy::asset::Assets;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::pbr::*;
use bevy::prelude::*;
use bevy::utils::default;
use bevy::DefaultPlugins;
use bevy_health_bar3d::prelude::{BarBundle, BarOffset, BarWidth, HealthBarPlugin, Percentage};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component, Reflect)]
struct Health {
    max: f32,
    current: f32,
}

impl Percentage for Health {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}

fn main() {
    App::new()
        .register_type::<Health>()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            HealthBarPlugin::<Health>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .insert_resource(Msaa::Sample4)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 0,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    let radius = 0.2;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                TryInto::<Mesh>::try_into(shape::Icosphere {
                    radius,
                    ..default()
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(1., 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 1., 0.0),
            ..Default::default()
        },
        Health {
            max: 10.,
            current: 8.,
        },
        BarBundle::<Health> {
            offset: BarOffset::new(radius * 1.5),
            width: BarWidth::new(radius * 2.),
            ..default()
        },
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                TryInto::<Mesh>::try_into(shape::Icosphere {
                    radius,
                    ..default()
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(1., 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0 + 3. * radius, 0.5, 0.0),
            ..Default::default()
        },
        Health {
            max: 10.,
            current: 2.,
        },
        BarBundle::<Health> {
            offset: BarOffset::new(radius * 1.5),
            width: BarWidth::new(radius * 2.),
            ..default()
        },
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        Rotate,
    ));

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1., 2., 5.),
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        camera: Camera {
            // renders after / on top of the main camera
            order: 1,
            ..default()
        },
        ..default()
    });
}

#[derive(Component)]
struct Rotate;

fn rotate_camera(
    mut camera_query: Query<&mut Transform, With<Rotate>>,
    mut angle: Local<f32>,
    time: Res<Time>,
) {
    let mut transform = camera_query.single_mut();
    let mut target_angle = *angle + 10. * time.delta_seconds();

    if target_angle > 360. {
        target_angle = 0.;
    }

    transform.translation.x = 5. * target_angle.to_radians().cos();
    transform.translation.z = 5. * target_angle.to_radians().sin();

    *angle = target_angle;
    *transform = transform.looking_at(Vec3::ZERO, Vec3::Y);
}
