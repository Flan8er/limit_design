use bevy::prelude::*;
use bevy_third_person_camera::*;

use crate::pages::experiments::galaga::{
    asset_loader::SceneAssets,
    collision_detection::{Collider, CollisionDamage},
    health::Health,
    movement::{Acceleration, MovingObjectBundle, PitchAcceleration, RollAcceleration, Velocity},
    schedule::InGameSet,
    state::GameState,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_ACCELERATION: f32 = 15.0;
const SPACESHIP_ROTATION_ACCELERATION: f32 = 0.00005;
const SPACESHIP_ROLL_ACCELERATION: f32 = 0.00005;
const SPACESHIP_HEALTH: f32 = 100.0;
const SPACESHIP_COLLSION_DAMAGE: f32 = 100.0;
const MISSILE_SPEED: f32 = 25.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const SPACESHIP_RADIUS: f32 = 5.0;
const MISSILE_RADIUS: f32 = 1.0;
const MISSILE_HEALTH: f32 = 1.0;
const MISSILE_COLLISION_DAMAGE: f32 = 5.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(OnEnter(GameState::GameOver), spawn_spaceship)
            .add_systems(
                Update,
                (
                    spaceship_movement_controls,
                    spaceship_weapon_controls,
                    spaceship_shield_controls,
                )
                    .chain()
                    .in_set(InGameSet::UserInput),
            )
            .add_systems(Update, spaceship_destroyed.in_set(InGameSet::EntityUpdates));
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: (
                SceneRoot(scene_assets.spaceship.clone()),
                Transform::from_translation(STARTING_TRANSLATION),
            ),
            collider: Collider::new(SPACESHIP_RADIUS),
            pitch_acceleration: PitchAcceleration::new(0.),
            roll_acceleration: RollAcceleration::new(0.),
        },
        Spaceship,
        Health::new(SPACESHIP_HEALTH),
        CollisionDamage::new(SPACESHIP_COLLSION_DAMAGE),
        ThirdPersonCameraTarget,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<
        (
            &mut Transform,
            &mut Acceleration,
            &mut RollAcceleration,
            &mut PitchAcceleration,
        ),
        With<Spaceship>,
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // time: Res<Time>,
) {
    // let (mut transform, mut acceleration) = query.single_mut();
    let Ok((mut transform, mut acceleration, mut roll, mut pitch)) = query.single_mut() else {
        return;
    };
    let mut default_rotation = 0.0;
    let mut default_roll = 0.0;
    let mut default_movement = 0.0;

    // Rotation movement
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        // default_rotation = -SPACESHIP_ROTATION_ACCELERATION;
        default_roll = SPACESHIP_ROLL_ACCELERATION;
    } else if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        // default_rotation = SPACESHIP_ROTATION_ACCELERATION;
        default_roll = -SPACESHIP_ROLL_ACCELERATION;
    }

    // Forward/Backward movement.
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        default_rotation = -SPACESHIP_ROTATION_ACCELERATION;
    } else if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        default_rotation = SPACESHIP_ROTATION_ACCELERATION;
    }

    // Roll movement
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        default_movement = SPACESHIP_ACCELERATION;
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        default_movement = -SPACESHIP_ACCELERATION;
    }

    // Rotate around the Y-axis.
    // Ignores the Z-axis rotation applied below.
    // transform.rotate_y(rotation);
    pitch.value = default_rotation + pitch.value;
    transform.rotate_local_x(pitch.value);

    // Rotate around the local Z-axis.
    // The rotation is relative to the current rotation.
    roll.value = default_roll + roll.value;
    transform.rotate_local_z(roll.value);

    // Update the spaceship's acceleration based on new direction.
    acceleration.value = -transform.forward() * default_movement;
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let Ok(transform) = query.single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: (
                    SceneRoot(scene_assets.missiles.clone()),
                    Transform::from_translation(
                        transform.translation - transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                ),
                collider: Collider::new(MISSILE_RADIUS),
                pitch_acceleration: PitchAcceleration::new(0.),
                roll_acceleration: RollAcceleration::new(0.),
            },
            SpaceshipMissile,
            Health::new(MISSILE_HEALTH),
            CollisionDamage::new(MISSILE_COLLISION_DAMAGE),
        ));
    }
}

fn spaceship_shield_controls(
    mut commands: Commands,
    query: Query<Entity, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(spaceship) = query.single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(spaceship).insert(SpaceshipShield);
    }
}

fn spaceship_destroyed(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(), With<Spaceship>>,
) {
    if query.single().is_err() {
        next_state.set(GameState::GameOver);
    }
}
