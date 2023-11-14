use std::time::Duration;

use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
};
use bevy_cursor::prelude::*;
use bevy_mod_raycast::prelude::*;
use ultimate_cate_battle::GameState;

#[derive(Component)]
struct Maze;

#[derive(Component)]
struct Player;

#[derive(Default, Resource)]
struct DegreesToTurn(f32);

#[derive(Default, Resource)]
struct DistanceToMove(f32);

#[derive(Resource)]
struct StepTimer(Timer);

#[derive(Resource, Default)]
struct StepsTaken(u32);

#[derive(Resource)]
pub struct Settings {
    pub turn_speed: u32,
    pub walk_speed: f32,
    pub field_of_view: f32,
}

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Maze), build_maze)
            .add_systems(
                Update,
                (
                    collision_check.before(player_move),
                    player_move,
                    camera,
                    step_sounds,
                    turn_sound,
                    player_turning.before(collision_check),
                )
                    .run_if(in_state(GameState::Maze)),
            )
            .add_systems(OnExit(GameState::Maze), exit_maze)
            .insert_resource(DegreesToTurn::default())
            .insert_resource(DistanceToMove::default())
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 1.0,
            })
            .insert_resource(Settings::default())
            .insert_resource(StepsTaken::default());
    }
}

fn player_turning(
    mut query: Query<&mut Transform, With<Player>>,
    mut degrees_to_turn: ResMut<DegreesToTurn>,
    distance_to_move: Res<DistanceToMove>,
    cursor: Res<CursorInfo>,
    window: Query<&Window>,
    settings: Res<Settings>,
    time: Res<Time>,
) {
    if degrees_to_turn.0 != 0.0 {
        for mut transform in &mut query {
            transform.rotation *= Quat::from_rotation_y(
                degrees_to_turn.0.signum().to_radians()
                    * settings.turn_speed as f32
                    * time.delta_seconds()
                    * 100.0,
            );
            degrees_to_turn.0 -= degrees_to_turn.0.signum()
                * settings.turn_speed as f32
                * time.delta_seconds()
                * 100.0;
            // hack
            if degrees_to_turn.0 < 1.0 && degrees_to_turn.0 > -1.0 {
                degrees_to_turn.0 = 0.0;
            }
        }
    }
    if degrees_to_turn.0 == 0.0 && distance_to_move.0 == 0.0 {
        if let Some(position) = cursor.window_position() {
            let window = window.single();
            let left = window.resolution.width() / 3.0;
            let right = window.resolution.width() - window.resolution.width() / 3.0;

            if position.x < left {
                degrees_to_turn.0 = 90.0;
            } else if position.x > right {
                degrees_to_turn.0 = -90.0;
            }
        }
    }
}

fn step_sounds(
    distance_to_move: Res<DistanceToMove>,
    mut step_timer: ResMut<StepTimer>,
    time: Res<Time>,
    settings: Res<Settings>,
    mut commands: Commands,
    walk_sound: Res<WalkSound>,
    mut steps_taken: ResMut<StepsTaken>,
) {
    if distance_to_move.0 != 0.0 {
        step_timer.0.tick(Duration::from_secs_f32(
            time.delta().as_secs_f32() * settings.walk_speed,
        ));
        if step_timer.0.just_finished() {
            steps_taken.0 += 1;
            let step_sound = match steps_taken.0 % 2 == 0 {
                true => walk_sound.0 .0.clone(),
                false => walk_sound.0 .1.clone(),
            };
            commands.spawn(AudioBundle {
                source: step_sound,
                settings: PlaybackSettings::DESPAWN,
            });
        };
    };
}

fn turn_sound(
    degrees_to_turn: Res<DegreesToTurn>,
    turning_sound: Query<&AudioSink, With<TurningSound>>,
) {
    if let Ok(sink) = turning_sound.get_single() {
        if degrees_to_turn.0 == 0.0 {
            sink.pause();
        } else {
            sink.play();
        };
    };
}

fn player_move(
    mut player: Query<&mut Transform, With<Player>>,
    mut distance_to_move: ResMut<DistanceToMove>,
    time: Res<Time>,
    settings: Res<Settings>,
) {
    // hack
    if distance_to_move.0 < 0.0 {
        distance_to_move.0 = 0.0;
    };
    if distance_to_move.0 > 0.0 {
        for mut transform in &mut player {
            let forward = transform.forward().round();
            transform.translation += forward * settings.walk_speed * time.delta_seconds() * 4.0;
            distance_to_move.0 -= settings.walk_speed * time.delta_seconds() * 4.0;
        }
    }
}

fn camera(mut query: Query<&mut Projection>, settings: Res<Settings>) {
    for projection in &mut query {
        if let Projection::Perspective(projection) = projection.into_inner() {
            projection.fov = settings.field_of_view.to_radians();
        }
    }
}

fn collision_check(
    player: Query<&Transform, With<Player>>,
    degrees_to_turn: Res<DegreesToTurn>,
    mut distance_to_move: ResMut<DistanceToMove>,
    mut raycast: Raycast,
) {
    if degrees_to_turn.0 == 0.0 && distance_to_move.0 == 0.0 {
        for transform in &player {
            let ray = Ray3d::new(transform.translation, transform.forward());
            let hits = raycast.cast_ray(ray, &RaycastSettings::default());
            for hit in hits {
                if hit.1.distance().round() != 1.0 {
                    distance_to_move.0 = 2.0;
                }
            }
        }
    }
}

#[derive(Component)]
struct TurningSound;

#[derive(Resource, Default, Deref, DerefMut)]
struct WalkSound((Handle<AudioSource>, Handle<AudioSource>));

fn build_maze(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("maze.gltf#Scene0"),
            ..Default::default()
        },
        Maze,
    ));
    commands
        .spawn((
            Player,
            Maze,
            TransformBundle::from_transform(Transform::from_xyz(1.0, 1.0, 5.0)),
            VisibilityBundle::default(),
        ))
        .with_children(|p| {
            p.spawn(Camera3dBundle {
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: 90.0,
                    ..Default::default()
                }),
                ..Default::default()
            });
        });
    commands.spawn((
        AudioBundle {
            source: asset_server.load("bgm.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::Relative(VolumeLevel::new(0.2))),
        },
        Maze,
    ));
    commands.spawn((
        AudioBundle {
            source: asset_server.load("turn.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::Relative(VolumeLevel::new(0.2))),
        },
        TurningSound,
    ));
    commands.insert_resource(WalkSound((
        asset_server.load("walk1.ogg"),
        asset_server.load("walk2.ogg"),
    )));
    commands.insert_resource(StepTimer(Timer::new(
        Duration::from_secs_f32(0.25),
        TimerMode::Repeating,
    )));
}

fn exit_maze(mut commands: Commands, query: Query<Entity, With<Maze>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            turn_speed: 1,
            walk_speed: 1.0,
            field_of_view: 90.0,
        }
    }
}
