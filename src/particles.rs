use std::cmp::PartialEq;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroupShaderType;
use bevy_hanabi::prelude::*;

pub struct ParticlesPlugin;


#[derive(Component, Eq, PartialEq, Clone, Debug)]
pub enum Sparker {
    RED,
    ORANGE,
    GREEN,
    RAINBOW
}

fn spark_effect() -> EffectAsset {
    let writer = ExprWriter::new();

    // Size over lifetime
    let mut size_gradient = bevy_hanabi::Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.08));
    size_gradient.add_key(1.0, Vec3::splat(0.01));

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(1.).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_velocity = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(5.).expr()
    };

    let lifetime = SetAttributeModifier::new(Attribute::LIFETIME,
                                             writer.lit(0.).uniform(writer.lit(4.)).expr());
    let gravity = AccelModifier::new(writer.lit(Vec3::new(0.0, -9.8, 0.0)).expr());
    let effect = EffectAsset::new(
        32768,
        SpawnerSettings::once(150.0.into()).with_emit_on_start(false),
        writer.finish()
    )
        .init(init_pos)
        .init(init_velocity)
        .init(lifetime)
        .update(gravity) // gravity
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        });

    effect
}

fn setup(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {

    // Gradient for spark color over lifetime
    let mut red_gradient = bevy_hanabi::Gradient::new();
    red_gradient.add_key(0.0, Vec4::new(1.0, 0.1, 0.1, 1.0)); // red
    red_gradient.add_key(1.0, Vec4::new(0.1, 0.1, 0.1, 0.0)); /* fade to dark transparent */

    // Gradient for spark color over lifetime
    let mut orange_gradient = bevy_hanabi::Gradient::new();
    orange_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 0.1, 1.0)); // red
    orange_gradient.add_key(1.0, Vec4::new(0.1, 0.1, 0.1, 0.0)); /* fade to dark transparent */

    let mut green_gradient = bevy_hanabi::Gradient::new();
    green_gradient.add_key(0.0, Vec4::new(0.0, 1.0, 0.1, 1.0)); // green
    green_gradient.add_key(1.0, Vec4::new(0.1, 0.1, 0.1, 0.0)); // fade to dark transparent

    let mut rainbow_gradient = bevy_hanabi::Gradient::new();
    rainbow_gradient.add_key(0.0, Vec4::new(0.0, 0.0, 1.0, 1.0)); // blue
    rainbow_gradient.add_key(0.1, Vec4::new(1.0, 1.0, 1.0, 0.5)); // white
    rainbow_gradient.add_key(0.2, Vec4::new(1.0, 0.0, 1.0, 1.0)); // purple
    rainbow_gradient.add_key(0.3, Vec4::new(1.0, 1.0, 1.0, 0.5)); // white
    rainbow_gradient.add_key(0.4, Vec4::new(0.0, 0.0, 1.0, 1.0)); // blue
    rainbow_gradient.add_key(0.5, Vec4::new(1.0, 1.0, 1.0, 0.5)); // white
    rainbow_gradient.add_key(0.6, Vec4::new(1.0, 0.0, 1.0, 1.0)); // purple
    rainbow_gradient.add_key(0.7, Vec4::new(1.0, 1.0, 1.0, 0.5)); // white
    rainbow_gradient.add_key(0.8, Vec4::new(0.0, 0.0, 1.0, 1.0)); // blue
    rainbow_gradient.add_key(0.9, Vec4::new(1.0, 1.0, 1.0, 1.0)); // white
    rainbow_gradient.add_key(1.0, Vec4::new(0.0, 0.0, 1.0, 0.0)); // fade to dark transparent


    let red_effect = spark_effect().render(ColorOverLifetimeModifier::new(red_gradient));
    let orange_effect = spark_effect().render(ColorOverLifetimeModifier::new(orange_gradient));
    let green_effect = spark_effect().render(ColorOverLifetimeModifier::new(green_gradient));
    let rainbow_effect = spark_effect().render(ColorOverLifetimeModifier::new(rainbow_gradient));

    let red_effect_handle = effects.add(red_effect);
    let orange_effect_handle = effects.add(orange_effect);
    let green_effect_handle = effects.add(green_effect);
    let rainbow_effect_handle = effects.add(rainbow_effect);

    commands.spawn((
        Sparker::RED,
        Name::new("red sparks"),
        ParticleEffect::new(red_effect_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Sparker::ORANGE,
        Name::new("orange sparks"),
        ParticleEffect::new(orange_effect_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Sparker::GREEN,
        Name::new("green sparks"),
        ParticleEffect::new(green_effect_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Sparker::RAINBOW,
        Name::new("rainbow sparks"),
        ParticleEffect::new(rainbow_effect_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

#[derive(Event, Deref)]
pub struct SparksFire(pub Sparker);

pub fn spark_firing(event: On<SparksFire>, mut query: Query<(&Sparker, &mut EffectSpawner)>) {
    let color = event.0.clone();
    if let Some((_, mut spawner)) = query.iter_mut().find(|(sparker, _)| {
        sparker == &&color
    }) {
        spawner.reset();
    }
}

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_observer(spark_firing);
    }
}
