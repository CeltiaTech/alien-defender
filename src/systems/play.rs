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
#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2
}
pub fn spawn_walls(mut commands: Commands){
    let wall_thickness = 16.0;
    let screen_width = 1344.0;
    let screen_height = 768.0;

    let half_w = screen_width * 0.5;
    let half_h = screen_height * 0.5;

    // Mur haut
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.8, 1.0), Vec2::new(screen_width, wall_thickness)),
        Transform::from_xyz(0.0, half_h - wall_thickness * 0.5, 10.0),
        GameEntity,
    ));

    // Mur bas
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.8, 1.0), Vec2::new(screen_width, wall_thickness)),
        Transform::from_xyz(0.0, -half_h + wall_thickness * 0.5, 10.0),
        GameEntity,
    ));

    // Mur gauche
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.8, 1.0), Vec2::new(wall_thickness, screen_height)),
        Transform::from_xyz(-half_w + wall_thickness * 0.5, 0.0, 10.0),
        GameEntity,
    ));

    // Mur droit
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.8, 1.0), Vec2::new(wall_thickness, screen_height)),
        Transform::from_xyz(half_w - wall_thickness * 0.5, 0.0, 10.0),
        GameEntity,
    ));

}
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
pub fn player_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>
) {
    let dt = time.delta_secs();

    let left_limit = -1344.0/2.0+WALL_THICKNESS;
    let right_limit = 1344.0/2.0 -WALL_THICKNESS;
    let player_half_width = 24.0;

    for (player, mut transform) in &mut query {
        let mut direction = 0.0;

        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            direction -= 1.0;
        }

        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            direction += 1.0;
        }
        
        transform.translation.x += direction * player.speed * dt;

        let min_x = left_limit + player_half_width;
        let max_x = right_limit - player_half_width;
        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
    }
}
fn spawn_projectile(commands: &mut Commands, position: Vec2, velocity: Vec2) {
    commands.spawn((
        GameEntity,
        Projectile {
            velocity,
        },
        Sprite {
            color: Color::srgb(1.0, 1.0, 0.2),
            custom_size: Some(Vec2::new(6.0, 6.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 5.0),
        GlobalTransform::default(),
    ));
}
pub fn projectile_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile)>,
) {
    let dt = time.delta_secs();

    
    let top = 768.0/2.0 - WALL_THICKNESS;
    
    for (entity, mut transform, projectile) in &mut projectile_query {

        transform.translation.y += projectile.velocity.y * dt;
        

        
        if transform.translation.y > top {
            commands.entity(entity).despawn();
            
        }
        

        
        }

}
pub fn player_shoot_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut PlayerWeapon), With<Player>>,
) {
    for (transform, mut weapon) in &mut query {
        
        if !keyboard.pressed(KeyCode::Space) {
            
            continue;
        } 
        
        if !weapon.timer.is_finished() {
            
            continue;
        }
        
        let origin = transform.translation.truncate() + Vec2::new(0.0, 20.0);

        
        spawn_projectile(&mut commands, origin, Vec2::new(0.0, 500.0));
            
        

        weapon.timer.reset();
    }
}
pub fn weapon_cooldown_system(
    time: Res<Time>,
    mut query: Query<&mut PlayerWeapon>,
) {
    for mut weapon in &mut query {
        weapon.timer.tick(time.delta());
    }
}