use std::time::Duration;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::{
    Ccd, Collider, ExternalImpulse, GravityScale, NoUserData, RapierPhysicsPlugin, RigidBody,
    Sleeping, Velocity,
};
use bevy_turborand::{GlobalRng, rng::Rng, GenCore, RngPlugin, RngComponent, DelegatedRng};

#[derive(Component)]
struct PlayerComponent {
    name: String,
    health: f32,
}

#[derive(Component)]
struct PipeSpawner;

#[derive(Resource)]
struct PipeSpawnerResource {
    /// track when the bomb should explode (non-repeating timer)
    timer: Timer,
}

fn spawn_pipes(
    mut commands: Commands,
    time: Res<Time>,
    mut spawner: ResMut<PipeSpawnerResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut global_rng: ResMut<GlobalRng>
) {
    spawner.timer.tick(time.delta());


    if spawner.timer.finished() {

        let rand_y = global_rng.i32(-200..=200) as f32;

        println!("Spawing psaiwn");

        let mut pipes_container = commands
            .spawn_empty()
            .insert(RngComponent::from(&mut global_rng))
            .insert(PipeSpawner)
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(500., rand_y, 0.)),
                ..default()
            })
            .id();

        let pipe_up = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Box::new(50.0, 500.0, 1.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(0., 350., 0.)),
                ..default()
            })
            .insert(RigidBody::KinematicPositionBased)
            .id();

        let pipe_bottom = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Box::new(50.0, 500.0, 1.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(0., -350., 0.)),
                ..default()
            })
            .insert(RigidBody::KinematicPositionBased)
            .id();

        commands.entity(pipes_container).add_child(pipe_up);
        commands.entity(pipes_container).add_child(pipe_bottom);
    }
}


fn move_pipes (mut pipes: Query<(&mut Transform , &mut RngComponent), (With<PipeSpawner>)>, time: Res<Time>) {

    for mut pipe in pipes.iter_mut() {

        pipe.0.translation.x -= 5.0;
    }

}



fn add_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(PipeSpawnerResource {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    });

    let mut camera = Camera2dBundle::default();
    camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::Rgba {
        red: (0.2f32),
        green: (0.2f32),
        blue: (0.2f32),
        alpha: (1.0f32),
    });
    commands.spawn(camera);

    // Circle
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-250., 0., 0.)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 2.0),
            angvel: 0.2,
        })
        .insert(GravityScale(9.81))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 200.0),
            torque_impulse: 14.0,
        })
        .insert(PlayerComponent {
            name: "Birdy 1".into(),
            health: 3.0,
        });
}

fn move_player(
    mut query: Query<&mut ExternalImpulse, With<PlayerComponent>>,
    keys: Res<Input<KeyCode>>,
) {
    for mut ext_impulse in query.iter_mut() {
        if keys.just_released(KeyCode::Space) {
            ext_impulse.impulse = Vec2::new(0.0, 800.0);
            ext_impulse.torque_impulse = 0.0;
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_plugin(RngPlugin::new().with_rng_seed(12345));
    app.add_startup_system(add_entities);
    app.add_system(move_player);
    app.add_system(spawn_pipes);
    app.add_system(move_pipes);
    app.run();
}
