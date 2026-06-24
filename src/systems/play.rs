use bevy::prelude::*;
// Épaisseur des murs
const WALL_THICKNESS: f32 = 16.0;
#[derive(Component)]
pub struct Player {
    pub speed: f32,
    
}
#[derive(Component)]
pub struct PlayerWeapon {
    
    pub cooldown: f32,
    pub timer: Timer,
}
#[derive(Component)]
pub struct GameEntity;
pub fn spawn_player(mut commands: Commands,asset_server:  Res<AssetServer>) {
    let texture = asset_server.load("sprites/playerchar.png");
    
    commands.spawn((
        GameEntity,
        Player { speed: 300.0 },
        PlayerWeapon{
            cooldown:120.0,
            timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        },
        Sprite {
            image: texture.clone(),
            
            ..default()
        },
        Transform::from_xyz(0.0, -320.0+WALL_THICKNESS, 10.0),
        GlobalTransform::default(),
    ));
}