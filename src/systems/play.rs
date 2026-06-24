use bevy::prelude::*;
use rand::rng;

use rand::RngExt;
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

pub struct Alien;
#[derive(Component)]

pub struct AlienAnimation {
    pub timer: Timer,
}
#[derive(Component)]
pub struct GameEntity;
#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2
}
#[derive(Resource)]

pub struct AlienFleet {
    pub direction: f32,
    pub base_speed: f32,
    pub max_speed: f32,
    pub step_down: f32,
    pub initial_count: usize,
}

impl Default for AlienFleet {
    fn default() -> Self {
        Self {
            direction: 1.0,
            base_speed: 40.0,
            max_speed: 300.0,
            step_down: 12.0,
            initial_count: 55,
        }
    }

}


#[derive(Resource)]
pub struct AlienShootTimer {
    pub timer: Timer,
}

impl Default for AlienShootTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.2, TimerMode::Repeating),
        }
    }
}
#[derive(Component)]
pub struct ShieldBlock;
pub fn alien_shoot_system(
    mut commands: Commands,
    time: Res<Time>,
    mut shoot_timer: ResMut<AlienShootTimer>,
    alien_query: Query<&Transform, With<Alien>>,
) {
    shoot_timer.timer.tick(time.delta());

    if !shoot_timer.timer.just_finished() {
        return;
    }

    let aliens: Vec<&Transform> = alien_query.iter().collect();

    if aliens.is_empty() {
        return;
    }

    let mut rng = rng();

    let index = rng.random_range(0..aliens.len());

    let random_alien = aliens[index];
    let column_x = random_alien.translation.x;

    let column_tolerance = 8.0;

    let Some(shooter) = aliens
        .iter()
        .filter(|alien| {
            (alien.translation.x - column_x).abs() < column_tolerance
        })
        .min_by(|a, b| {
            a.translation
                .y
                .partial_cmp(&b.translation.y)
                .unwrap()
        })
    else {
        return;
    };

    let origin = shooter.translation.truncate() + Vec2::new(0.0, -28.0);

    spawn_projectile(
        &mut commands,
        origin,
        Vec2::new(0.0, -300.0),
    );
}
pub fn spawn_background(mut commands: Commands,asset_server:  Res<AssetServer>){
    let texture = asset_server.load("backgrounds/level1.png");


    // Créer un sprite avec cette image
    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(0.0, 0.0, -10.0),
        GameEntity,
        
    ));
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
pub fn spawn_aliens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    let texture = asset_server.load("sprites/alien_walk.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(276, 266),
        4,
        1,
        Some(UVec2::new(41, 0)),
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);
    let columns = 11;
    let rows = 5;
    let sprite_size = Vec2::new(48.0, 48.0);
    let spacing = Vec2::new(32.0, 24.0);
    let start_x = -((columns as f32 - 1.0) * (sprite_size.x + spacing.x)) / 2.0;
    let start_y = 300.0;
    for row in 0..rows {
        for col in 0..columns {
            let x = start_x + col as f32 * (sprite_size.x + spacing.x);
            let y = start_y - row as f32 * (sprite_size.y + spacing.y);
            commands.spawn((
                Alien,
                AlienAnimation {
                    timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                },
                Sprite {
                    image: texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: layout_handle.clone(),
                        index: 0,
                    }),
                    custom_size: Some(sprite_size),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));

            

        }

    }

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
pub fn move_aliens(
    time: Res<Time>,
    mut fleet: ResMut<AlienFleet>,
    mut alien_query: Query<&mut Transform, With<Alien>>,
    window_query: Query<&Window>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let alien_count = alien_query.iter().count();

    if alien_count == 0 {
        return;
    }

    let remaining_ratio = alien_count as f32 / fleet.initial_count as f32;

    let speed = fleet.base_speed
        + (1.0 - remaining_ratio) * (fleet.max_speed - fleet.base_speed);

    let half_width = window.width() / 2.0;
    let margin = 40.0;

    let left_limit = -half_width + margin;
    let right_limit = half_width - margin;

    let mut should_turn = false;

    for transform in alien_query.iter() {
        let x = transform.translation.x;

        if fleet.direction > 0.0 && x >= right_limit {
            should_turn = true;
            break;
        }

        if fleet.direction < 0.0 && x <= left_limit {
            should_turn = true;
            break;
        }
    }

    if should_turn {
        fleet.direction *= -1.0;

        for mut transform in alien_query.iter_mut() {
            transform.translation.y -= fleet.step_down;
        }
    } else {
        let dx = fleet.direction * speed * time.delta_secs();

        for mut transform in alien_query.iter_mut() {
            transform.translation.x += dx;
        }
    }
}

pub fn animate_aliens(
    time: Res<Time>,
    mut query: Query<(&mut AlienAnimation, &mut Sprite), With<Alien>>,
) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % 4;
            }
        }
    }
}pub fn projectile_alien_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    alien_query: Query<(Entity, &Transform), With<Alien>>,
) {
    let projectile_size = Vec2::new(6.0, 6.0);
    let alien_size = Vec2::new(48.0, 48.0);

    for (projectile_entity, projectile_transform) in &projectile_query {
        let projectile_pos = projectile_transform.translation.truncate();

        let mut hit = false;

        for (alien_entity, alien_transform) in &alien_query {
            let alien_pos = alien_transform.translation.truncate();

            let collision =
                (projectile_pos.x - alien_pos.x).abs()
                    < (projectile_size.x + alien_size.x) * 0.5
                && (projectile_pos.y - alien_pos.y).abs()
                    < (projectile_size.y + alien_size.y) * 0.5;

            if collision {
                commands.entity(projectile_entity).despawn();
                commands.entity(alien_entity).despawn();

                hit = true;
                break;
            }
        }

        if hit {
            continue;
        }
    }
}
pub fn projectile_player_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Projectile)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    let projectile_size = Vec2::new(6.0, 12.0);
    let player_size = Vec2::new(48.0, 48.0);

    let Ok((player_entity, player_transform)) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();

    for (projectile_entity, projectile_transform, projectile) in &projectile_query {
        // seuls les projectiles qui descendent peuvent toucher le joueur
        if projectile.velocity.y >= 0.0 {
            continue;
        }

        let projectile_pos = projectile_transform.translation.truncate();

        let collision =
            (projectile_pos.x - player_pos.x).abs() < (projectile_size.x + player_size.x) * 0.5
            && (projectile_pos.y - player_pos.y).abs() < (projectile_size.y + player_size.y) * 0.5;

        if collision {
            commands.entity(projectile_entity).despawn();
            commands.entity(player_entity).despawn();

            println!("GAME OVER");

            break;
        }
    }
}
pub fn spawn_shields(mut commands: Commands) {
    let shield_positions = [-360.0, 0.0, 360.0];

    let block_size = Vec2::new(14.0, 14.0);
    let rows = 4;
    let cols = 8;

    for shield_x in shield_positions {
        for row in 0..rows {
            for col in 0..cols {
                // petit trou central en bas, style abri
                if row == 0 && (col == 3 || col == 4) {
                    continue;
                }

                let x = shield_x + (col as f32 - cols as f32 / 2.0) * block_size.x;
                let y = -210.0 + row as f32 * block_size.y;

                commands.spawn((
                    GameEntity,
                    ShieldBlock,
                    Sprite::from_color(
                        Color::srgb(0.2, 1.0, 0.2),
                        block_size,
                    ),
                    Transform::from_xyz(x, y, 2.0),
                ));
            }
        }
    }
}
pub fn projectile_shield_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    shield_query: Query<(Entity, &Transform), With<ShieldBlock>>,
) {
    let projectile_size = Vec2::new(6.0, 12.0);
    let block_size = Vec2::new(14.0, 14.0);

    for (projectile_entity, projectile_transform) in &projectile_query {
        let projectile_pos = projectile_transform.translation.truncate();

        for (block_entity, block_transform) in &shield_query {
            let block_pos = block_transform.translation.truncate();

            let collision =
                (projectile_pos.x - block_pos.x).abs() < (projectile_size.x + block_size.x) * 0.5
                && (projectile_pos.y - block_pos.y).abs() < (projectile_size.y + block_size.y) * 0.5;

            if collision {
                commands.entity(projectile_entity).despawn();
                commands.entity(block_entity).despawn();
                break;
            }
        }
    }
}