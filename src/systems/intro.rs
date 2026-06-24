use bevy::prelude::*;

use crate::gamestate;
#[derive(Component)]
pub struct Blink {
    timer: Timer,
}
#[derive(Component)]
pub struct IntroEntity;
pub fn setup_game(mut commands: Commands,
    asset_server: Res<AssetServer>,){
    //affichage de l'image d'intro,
   

    let texture = asset_server.load("backgrounds/introscreen.png");


    // Créer un sprite avec cette image
    commands.spawn((
        Sprite::from_image(texture),
        IntroEntity,
    ));
    
    let font = asset_server.load("fonts/Super Starfish.ttf");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            IntroEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Press any key to continue"),
                TextFont {
                font: FontSource::Handle(font.clone()),
                font_size: FontSize::Px(40.0),
                ..default()
            },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                Blink {
                    timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                }
            ));
        });
}
pub fn blink_text(
    time: Res<Time>,
    mut query: Query<(&mut TextColor, &mut Blink)>,
) {
    for (mut color, mut blink) in &mut query {
        blink.timer.tick(time.delta());

        let t = blink.timer.elapsed_secs().sin() * 0.5 + 0.5;

        let r = 1.0;
        let g = 1.0 - t;
        let b = 1.0 - t;

        color.0 = Color::srgb(r, g, b);
    }
}
pub fn setup_intro_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let music = asset_server.load("musics/AlienDefender.ogg");

    commands.spawn((
        AudioPlayer::new(music),
        PlaybackSettings::LOOP,
        IntroEntity,
    ));
}
pub fn cleanup_screen(
    mut commands: Commands,
    query: Query<Entity, With<IntroEntity>>,
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
pub fn cleanup_music(
    mut commands: Commands,
    query: Query<Entity, With<IntroEntity>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
pub fn press_any_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<gamestate::GameState>>,
) {
    if keyboard.any_just_pressed([
        KeyCode::Space,
        KeyCode::Enter,
        KeyCode::KeyA,
    ]) {
        next_state.set(gamestate::GameState::Playing);
    }
}