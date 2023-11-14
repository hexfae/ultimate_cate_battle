use crate::Settings;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use ultimate_cate_battle::GameState;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_menu);
    }
}

fn debug_menu(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<GameState>>,
    mut settings: ResMut<Settings>,
) {
    egui::Window::new("debug").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("menu").clicked() {
                state.set(GameState::Menu);
            };
            if ui.button("maze").clicked() {
                state.set(GameState::Maze);
            };
        });
        ui.add(egui::Slider::new(&mut settings.walk_speed, 0.0..=3.0));
        ui.add(egui::Slider::new(&mut settings.turn_speed, 0..=3));
        ui.add(egui::Slider::new(&mut settings.field_of_view, 0.0..=180.0));
    });
}
