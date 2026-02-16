use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
#[derive(Component)]
pub struct Player;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Left;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Down;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Up;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Right;

#[derive(InputAction)]
#[action_output(bool)]
pub struct StartSelect;

fn setup_inputs(mut commands: Commands,) {
    commands.spawn((
        Player,
        actions!(Player[
            (
            Action::<Left>::new(),
                bindings![KeyCode::ArrowLeft]
            ),
            (
            Action::<Down>::new(),
                bindings![KeyCode::ArrowDown]
            ),
            (
            Action::<Up>::new(),
                bindings![KeyCode::ArrowUp]
            ),
            (
            Action::<Right>::new(),
                bindings![KeyCode::ArrowRight]
            ),
            (Action::<StartSelect>::new(),
                bindings![KeyCode::Space]
            ),
        ])
    ));
}

pub struct InputsPlugin;

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_inputs);
    }
}

