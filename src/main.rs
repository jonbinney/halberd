use bevy::{prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Debug Info").show(contexts.ctx_mut(), |ui| {
        ui.label("TODO");
    });
}

