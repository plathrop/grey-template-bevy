use bevy::{prelude::*, window::WindowResolution};

// I'm a sucker for Solarized Dark
const BACKGROUND_COLOR: Color = Color::srgb(0.0, 43.0 / 255.0, 54.0 / 255.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy!".to_string(),
                resolution: WindowResolution::new(1200.0, 600.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Msaa::Sample4) // TODO: Ensure I actually understand this.
        .add_systems(Startup, setup_camera)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn close_on_esc(mut commands: Commands, windows: Query<(Entity, &Window)>, input: Res<ButtonInput<KeyCode>>) {
    for (window, focus) in windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
