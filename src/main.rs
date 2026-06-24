use bevy::{prelude::*, window::WindowResolution};


mod gamestate;
mod systems;
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
    .add_systems(OnEnter(gamestate::GameState::Intro),systems::intro::setup_intro_music)
    .add_systems(OnEnter(gamestate::GameState::Intro), systems::intro::setup_game)
    .add_systems(Update, systems::intro::press_any_key.run_if(in_state(gamestate::GameState::Intro)))
    
    .add_systems(OnExit(gamestate::GameState::Intro), systems::intro::cleanup_screen)
    .add_systems(OnExit(gamestate::GameState::Intro), systems::intro::cleanup_music)
    .add_systems(Update, systems::intro::blink_text.run_if(in_state(gamestate::GameState::Intro)))
    
    
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}