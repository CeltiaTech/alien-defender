use bevy::{prelude::*, window::WindowResolution};


mod gamestate;
fn main() {
   App::new()
        
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Alien Defender".into(),
                canvas: Some("#bevy".to_string()),
                resolution: WindowResolution::new(1344, 768),
                ..default()
            }),
            ..default()
        })
    )
    .init_state::<gamestate::GameState>()
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}