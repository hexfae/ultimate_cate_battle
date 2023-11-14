use bevy::prelude::*;
use bevy_tweening::{
    lens::TransformScaleLens, Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween,
};
use std::time::Duration;
use ultimate_cate_battle::GameState;

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct Jinx;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), build_menu)
            .add_systems(Update, move_jinx.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), exit_menu);
    }
}

fn build_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), Menu));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "menu",
                TextStyle {
                    font_size: 100.0,
                    ..Default::default()
                },
            ));
        });
    let jiggle = Tween::new(
        EaseFunction::SineInOut,
        Duration::from_secs_f64(0.1),
        TransformScaleLens {
            start: Vec3::new(0.95, 1.05, 0.95),
            end: Vec3::new(1.10, 0.95, 1.10),
        },
    )
    .with_repeat_count(RepeatCount::Infinite)
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("jinx.webp"),
            ..Default::default()
        },
        Animator::new(jiggle),
        Jinx,
        Menu,
    ));
}

fn move_jinx(mut jinx: Query<&mut Transform, With<Jinx>>, time: Res<Time>) {
    for mut transform in &mut jinx {
        let elapsed = time.elapsed().as_secs_f32();
        transform.translation.x = (elapsed * 200.0).to_radians().sin() * 150.0;
        transform.translation.y = (elapsed * 110.0).to_radians().cos() * 200.0;
    }
}

fn exit_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for camera in query.iter() {
        commands.entity(camera).despawn_recursive();
    }
}
