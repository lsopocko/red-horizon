use bevy::{log, prelude::*};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Paused,
    Playing,
}

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, (pause_system));
    }
}

fn pause_system(
    mut state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(GameState::Paused)
    }

    if keyboard_input.just_pressed(KeyCode::Enter) {
        state.set(GameState::Playing)
    }
}
