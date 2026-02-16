mod inputs;
mod title;
mod ingame;
mod particles;
mod enemy;
use bevy::core_pipeline::oit::OrderIndependentTransparencyPlugin;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_hanabi::HanabiPlugin;
use bevy_seedling::prelude::*;

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use inputs::Player;
use inputs::InputsPlugin;
use crate::ingame::InGamePlugin;
use crate::particles::ParticlesPlugin;
use crate::title::TitlePlugin;

const IDOL: &str = "idol.glb";

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Title,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(HanabiPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(SeedlingPlugin::default())
        .add_plugins(EnhancedInputPlugin)
        .add_input_context::<Player>()
        .add_plugins(InputsPlugin)
        .add_plugins(TitlePlugin)
        .add_plugins(InGamePlugin)
        .add_plugins(ParticlesPlugin)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, anim_setup)
        .add_systems(Update, check_anim)
        .run();
}

fn setup(mut commands: Commands,
mut meshes: ResMut<Assets<Mesh>>,
mut materials: ResMut<Assets<StandardMaterial>>,
asset_server: Res<AssetServer>,
mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Add a light source so we can see clearly.
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn(
        (
            Name::new("Ski Clubber"),
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(IDOL)
        ))));

    let mut animation_graph = AnimationGraph::new();
    let idle = animation_graph.add_clip(asset_server.load(GltfAssetLabel::Animation(1).from_asset(IDOL)),
                             1.0,
                             animation_graph.root
    );

    let down = animation_graph.add_clip(asset_server.load(GltfAssetLabel::Animation(0).from_asset(IDOL)),
                             1.0,
                             idle,
    );

    let left = animation_graph.add_clip(asset_server.load(GltfAssetLabel::Animation(2).from_asset(IDOL)),
                             1.0,
                             idle,
    );

    let right = animation_graph.add_clip(asset_server.load(GltfAssetLabel::Animation(3).from_asset(IDOL)),
                             1.0,
                             idle,
    );
    let up =animation_graph.add_clip(asset_server.load(GltfAssetLabel::Animation(4).from_asset(IDOL)),
                             1.0,
                             idle
    );

    commands.insert_resource(PCAnimationGraph {
        graph_handle: animation_graphs.add(animation_graph),
        idle: idle,
        left: left,
        down: down,
        right: right,
        up: up,
    });
}

fn anim_setup(
    mut commands: Commands,
    mut players: Query<(Entity, & mut AnimationPlayer), Added<AnimationPlayer>>,
    graphs: Res<Assets<AnimationGraph>>,
    pc_anim_graph: Res<PCAnimationGraph>
) {
    for (entity, mut player) in &mut players {
        let graph = graphs.get(&pc_anim_graph.graph_handle).unwrap();
        commands.entity(entity)
            .insert(AnimationGraphHandle(pc_anim_graph.graph_handle.clone()));

        println!("Inserted!");
    };
}

fn check_anim(
    mut players: Query<&mut AnimationPlayer>
) {
    if let Ok(mut player) = players.single_mut() {
    }
}

#[derive(Resource)]
pub struct PCAnimationGraph {
    graph_handle: Handle<AnimationGraph>,
    idle: AnimationNodeIndex,
    left: AnimationNodeIndex,
    down: AnimationNodeIndex,
    up: AnimationNodeIndex,
    right: AnimationNodeIndex,
}