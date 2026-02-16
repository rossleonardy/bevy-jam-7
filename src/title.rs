use bevy::prelude::*;
use crate::inputs::StartSelect;
use bevy_enhanced_input::prelude::*;
use crate::AppState;

fn title_start(start: On<Complete<StartSelect>>, mut next_state: ResMut<NextState<AppState>>) {
    println!("Title start!");
    next_state.set(AppState::InGame);
}

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(title_start);
    }
}
