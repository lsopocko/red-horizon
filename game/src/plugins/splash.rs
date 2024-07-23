use bevy::{log, prelude::*};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Paused,
    Playing,
}

pub struct SplashPlugin;

#[derive(Component)]
pub struct Title;

#[derive(Component)]
pub struct Subtitle;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, pause_system)
            .add_systems(Startup, show_splash_screen);
    }
}

fn pause_system(
    mut commands: Commands,
    mut state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    title_query: Query<Entity, With<Title>>,
    subtitle_query: Query<Entity, With<Subtitle>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(GameState::Paused)
    }

    if keyboard_input.just_pressed(KeyCode::Enter) {
        state.set(GameState::Playing);
        // desawn the title and subtitle
        for entity in title_query.iter() {
            log::info!("Despawning title");
            commands.entity(entity).despawn();
        }
        for entity in subtitle_query.iter() {
            log::info!("Despawning subtitle");
            commands.entity(entity).despawn();
        }
    }
}

fn show_splash_screen(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Red Horizon",
            TextStyle {
                font_size: 96.,
                color: Color::srgb(0.8, 0.2, 0.2),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(35.),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Title,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Press Enter to start",
            TextStyle {
                font_size: 30.,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(67.),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Subtitle,
    ));
}
