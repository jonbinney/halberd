use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Halberd".into(),
            resolution: (500., 300.).into(),
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }));

    app.add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system);

    app.run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Debug Info").show(contexts.ctx_mut(), |ui| {
        ui.label("TODO");
    });
}
