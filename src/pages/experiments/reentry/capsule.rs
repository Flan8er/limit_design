use bevy::prelude::*;
use std::f32::consts::PI;

use crate::pages::experiments::reentry::{
    asset_loader::SceneAssets,
    calculation::{
        calculate_cumulative_acceleration, get_current_altitude, get_current_atmospheric_density,
    },
    earth::EARTH_DIAMETER,
    movement::Rotatable,
    notification::CapsuleState,
};

// pub const INITIAL_POSITION: Vec3 = Vec3::new((EARTH_DIAMETER / 2.) + 200_000., 0., 0.);
pub const INITIAL_POSITION: Vec3 = Vec3::new((EARTH_DIAMETER / 2.) + 190_000., 0., 0.);
// pub const INITIAL_VELOCITY: Vec3 = Vec3::new(0., 7500., 0.);
pub const INITIAL_VELOCITY: Vec3 = Vec3::new(0., 7309.7222, 0.);
// pub const CAPSULE_MASS: f32 = 4.; // [kg]
pub const CAPSULE_MASS: f32 = 77110.7;
// pub const CAPSULE_RADIUS: f32 = 0.125; // [m]
pub const CAPSULE_RADIUS: f32 = 4.5;
// pub const CAPSULE_DRAG_COEFFICIENT: f32 = 0.42; // Standard C_d for a half-sphere
pub const CAPSULE_DRAG_COEFFICIENT: f32 = 2.16;

#[derive(Resource, Debug, Clone)]
pub struct Capsule {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Capsule {
    pub fn new(position: Vec3, velocity: Vec3) -> Self {
        Self {
            position: position,
            velocity: velocity,
        }
    }
}

#[derive(Component)]
pub struct Vehicle;

pub fn spawn_capsule(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands
        .spawn((
            Vehicle,
            SceneRoot(scene_assets.capsule.clone()),
            Transform {
                ..Default::default()
            },
            Rotatable {
                speed: 0.05,
                axis: "y".to_string(),
            },
            Name::new("Capsule"),
        ))
        .insert(Transform {
            translation: INITIAL_POSITION, // Move to the global INITIAL_POSITION
            ..Default::default()
        });
}

pub fn update_capsule_position(
    mut previous_state: ResMut<Capsule>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Vehicle>>,
    mut event_writer: EventWriter<CapsuleState>,
) {
    let delta_time: f32 = time.delta().as_secs_f32();
    if delta_time == 0.0 {
        return;
    }

    // Get the single capsule entity from the query.
    let Ok(mut capsule_transform) = query.single_mut() else {
        return;
    };

    let cross_sectional_area: f32 = PI * CAPSULE_RADIUS.powi(2);

    // Collect the previous state's position and velocity.
    let position = previous_state.position;
    let velocity = previous_state.velocity;

    // Calculate the previous altitude.
    let altitude = get_current_altitude(position);

    // Don't calculate change if vehicle is on the ground.
    if altitude <= 50.0 {
        return;
    }

    // Calculate the air density based off the previous altitude.
    let air_density = get_current_atmospheric_density(altitude);

    // Calculate the previous acceleration.
    let acceleration_total = calculate_cumulative_acceleration(
        air_density,
        CAPSULE_DRAG_COEFFICIENT,
        cross_sectional_area,
        CAPSULE_MASS,
        velocity,
        position,
    );

    // Apply the acceleration to find the new position using the previous cycle time (this assumes cycle times are roughly equal).
    // New velocity is the velocity plus the change in velocity (acceleration times change in time).
    let updated_velocity = Vec3::new(
        velocity.x - acceleration_total.x * delta_time,
        velocity.y - acceleration_total.y * delta_time,
        velocity.z - acceleration_total.z * delta_time,
    );

    let updated_position = Vec3::new(
        position.x + updated_velocity.x * delta_time,
        position.y + updated_velocity.y * delta_time,
        position.z + updated_velocity.z * delta_time,
    );

    let updated_velocity_magnitude =
        (updated_velocity.x.powi(2) + updated_velocity.y.powi(2) + updated_velocity.z.powi(2))
            .sqrt();
    println!(
        "Altitude: {:6.0} - Velocity: {:5.0}",
        altitude, updated_velocity_magnitude
    );
    event_writer.write({
        CapsuleState {
            altitude,
            velocity: updated_velocity_magnitude,
        }
    });

    // Update states for next cycle.
    previous_state.position = updated_position;
    previous_state.velocity = updated_velocity;

    // Reflect the updated position and orientation.
    let capsule_forward = updated_velocity.normalize();
    let rotation = Quat::from_rotation_arc(Vec3::Y, capsule_forward);
    *capsule_transform = Transform::from_translation(updated_position).with_rotation(rotation);
}
