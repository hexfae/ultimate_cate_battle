use bevy::{prelude::*, window::WindowMode};
use bevy_cursor::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_egui::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_tweening::TweeningPlugin;

use debug::DebugPlugin;
use maze::{MazePlugin, Settings};
use menu::MenuPlugin;
use ultimate_cate_battle::GameState;

mod debug;
mod maze;
mod menu;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
            EguiPlugin,
            EditorPlugin::default(),
            CursorInfoPlugin,
            TweeningPlugin,
            //my plugins
            MazePlugin,
            MenuPlugin,
            DebugPlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Update, full_screen)
        .run()
}

fn full_screen(keyboard_input: Res<Input<KeyCode>>, mut window: Query<&mut Window>) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        let mut window = window.single_mut();
        if let WindowMode::BorderlessFullscreen = window.mode {
            window.mode = WindowMode::Windowed;
        } else {
            window.mode = WindowMode::BorderlessFullscreen;
        }
    }
}
