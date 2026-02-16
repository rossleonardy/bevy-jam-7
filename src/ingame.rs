use std::f64::consts::PI;
use bevy::animation::RepeatAnimation;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_seedling::pool::Sampler;
use bevy_seedling::prelude::*;
use rand::{Rng, RngExt};

use crate::{AppState, PCAnimationGraph};
use crate::inputs::{StartSelect, Left, Down, Up, Right};
use crate::particles::{Sparker, SparksFire};

// timing scores in ms
const BAD: f64 = 128.0;
const GOOD: f64 = 64.0;
const GREAT: f64 = 32.0;
// const PERFECT: f64 = 32.0;

#[derive(Debug)]
enum Score {
    Miss,
    Bad,
    Good,
    Great
}

#[derive(Component)]
pub struct StageTrack {
    bpm: f64,
}

#[derive(Component, Default)]
pub struct Metronome {
    amplitude: f32,
}

// all times in ms
#[derive(Component, Default)]
pub struct StageTrackPlayhead {
    current_time: f64,
    current_note_beat: f64,
    current_note_time: f64,
    last_note_beat: f64,
    next_note_beat: f64,
    last_note_time: f64,
    next_note_time: f64,
}

pub fn score_diff(diff: f64) -> Score {
    if diff.abs() < GREAT {
        Score::Great
    } else if diff.abs() < GOOD {
        Score::Good
    } else if diff.abs() < BAD {
        Score::Bad
    } else {
        Score::Miss
    }
}

fn setup_stage(mut commands: Commands,
               mut asset_server: ResMut<AssetServer>,
               mut meshes: ResMut<Assets<Mesh>>,
               mut materials: ResMut<Assets<StandardMaterial>>,
               ) {

    // REFACTOR PLZZZ
    commands.spawn(Observer::new(dance_left));
    commands.spawn(Observer::new(dance_right));
    commands.spawn(Observer::new(dance_up));
    commands.spawn(Observer::new(dance_down));
    commands.spawn(Observer::new(check_dance_move));

    // MUSIC
    commands.spawn((
       SamplePlayer::new(asset_server.load("icyday.ogg")).looping(),
       StageTrack {
           bpm: 120.0,
       },
        StageTrackPlayhead::default()
    ));

    // METRONOME
    commands.spawn((
        Metronome::default(),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(Vec3::ZERO),
    ));

    // ENEMY
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0.. std::f32::consts::TAU);
    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(Color::srgb(1.0,0.0, 0.0))),
        Transform::from_translation(Vec3::new(
            (Vec2::from_angle(angle) * 10.0).x,
            0.0,
            (Vec2::from_angle(angle) * 10.0).y,)),
    ));
    println!("Setup stage");
}

fn handle_sampler(mut query_player: Query<(&StageTrack, &Sampler, &mut StageTrackPlayhead)>) {
    match query_player.single_mut() {
        Ok((_, sampler, mut playhead)) => {
            let beats_per_ms = (120.0 / 60.0) / 1000.0;
            let ms_time = sampler.playhead_seconds().0 * 1000.0;

            let beat_time = ms_time * beats_per_ms;
            let last_note_beat = beat_time.floor();
            let next_note_beat = beat_time.ceil();

            let last_note_time = last_note_beat / beats_per_ms;
            let next_note_time = next_note_beat / beats_per_ms;

            playhead.current_time = ms_time;

            let (current_note_beat, current_note_time) =
                if (ms_time - last_note_time).abs() < (ms_time - next_note_time).abs() {
                    (last_note_beat, last_note_time)
                } else {
                    (next_note_beat, next_note_time)
                };

            playhead.last_note_beat = last_note_beat;
            playhead.next_note_beat = next_note_beat;
            playhead.last_note_time = last_note_time;
            playhead.next_note_time = next_note_time;
            playhead.current_note_beat = current_note_beat;
            playhead.current_note_time = current_note_time;
        }
        _ => {

        }
    }
}

fn cleanup_stage(mut samples: Query<&mut PlaybackSettings>) {
    for mut params in samples.iter_mut() {
        params.pause();
    }
    println!("Cleanup stage");
}

fn stage_fixed_update(stage: Single<(&StageTrack, &StageTrackPlayhead)>, mut metronome: Single<(&mut Metronome)>) {
    let (_, playhead) = stage.into_inner();

    let omega: f64 = 2.0 * PI * (120.0 / 60000.0) / 2.0;
    let amp: f64 = f64::sin(omega * playhead.current_time);
    let sign = amp.signum();
    let eased = amp.abs().powf(0.7);

    metronome.amplitude = (sign * eased) as f32;
}

fn stage_update(
        time: Res<Time<Fixed>>,
        mut metronome: Single<(&Metronome, &mut Transform)>) {
    let (metronome, mut transform) = metronome.into_inner();
    transform.translation.x = transform.translation.x.lerp( metronome.amplitude, time.overstep_fraction());
}

fn check_dance_move(
    dance: On<DanceEvent>,
    mut commands: Commands,
    query: Query<(&StageTrack, &StageTrackPlayhead)>) {
    let (_, playhead) = query.single().unwrap();
    println!("Check time lnt {}  nnt {} lnb {} nnb {}", playhead.last_note_time, playhead.next_note_time, playhead.last_note_beat, playhead.next_note_beat);
    let diff = playhead.current_time - playhead.current_note_time;
    println!("Dir: {:?}", dance.event());
    println!("Score: {:?} {}", score_diff(diff), diff);
    let score = score_diff(diff);

    match score {
        Score::Miss => commands.trigger(SparksFire(Sparker::RED)),
        Score::Bad => commands.trigger(SparksFire(Sparker::ORANGE)),
        Score::Good => commands.trigger(SparksFire(Sparker::GREEN)),
        Score::Great => commands.trigger(SparksFire(Sparker::RAINBOW)),
    }
}

#[derive(Event, Debug)]
enum DanceEvent {
    Left,
    Down,
    Up,
    Right
}

// this is so silly and wrong please dont judge me lol
fn dance_left(left: On<Complete<Left>>,
              mut commands: Commands,
                mut query: Query<(&mut AnimationPlayer)>,
              anims: Res<PCAnimationGraph>) {
    commands.trigger(DanceEvent::Left);
    let mut player = query.single_mut().unwrap();
    player.stop_all();
    player.start(anims.left).set_repeat(RepeatAnimation::Never);
}

fn dance_down(down: On<Complete<Down>>, mut commands: Commands,
              mut query: Query<(&mut AnimationPlayer)>,
                anims: Res<PCAnimationGraph>) {
    commands.trigger(DanceEvent::Down);
    let mut player = query.single_mut().unwrap();
    player.stop_all();
    player.start(anims.down).set_repeat(RepeatAnimation::Never);
}

fn dance_up(up: On<Complete<Up>>, mut commands: Commands,
            mut query: Query<(&mut AnimationPlayer)>,
            anims: Res<PCAnimationGraph>) {
    commands.trigger(DanceEvent::Up);
    let mut player = query.single_mut().unwrap();
    player.stop_all();
    player.start(anims.up).set_repeat(RepeatAnimation::Never);
}

fn dance_right(right: On<Complete<Right>>, mut commands: Commands,
mut query: Query<(&mut AnimationPlayer)>,
               anims: Res<PCAnimationGraph>) {
    commands.trigger(DanceEvent::Right);
    let mut player = query.single_mut().unwrap();
    player.stop_all();
    player.start(anims.right).set_repeat(RepeatAnimation::Never);
}



pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_stage);
        app.add_systems(Update, handle_sampler.run_if(in_state(AppState::InGame)));
        app.add_systems(FixedUpdate, stage_fixed_update);
        app.add_systems(Update, stage_update);
        app.add_systems(OnExit(AppState::InGame), cleanup_stage);

    }
}
