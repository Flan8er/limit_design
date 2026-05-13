use bevy::{math::DVec3, prelude::*};
use std::f64::consts::E;

use crate::pages::experiments::threat_prediction::components::simulation_params::{
    SimulationParameters, EARTH_MASS, EARTH_RADIUS, GRAVITATIONAL_CONST,
};

pub const DT: f64 = 0.01;
// pub const CAPSULE_MASS: f64 = 77110.7;
// pub const CAPSULE_RADIUS: f64 = 4.5;
// pub const CAPSULE_DRAG_COEFFICIENT: f64 = 2.16;
pub const CAPSULE_MASS: f64 = 26520.0;
pub const CAPSULE_RADIUS: f64 = 2.5;
pub const CAPSULE_DRAG_COEFFICIENT: f64 = 1.3;

#[derive(Debug)]
pub struct CapsuleState {
    pub position: DVec3,
    pub velocity: DVec3,
}

fn build_initial_state(params: &SimulationParameters) -> CapsuleState {
    let r = EARTH_RADIUS + params.separation_altitude as f64;
    let separation_velocity = params.separation_velocity as f64;
    let heading = (params.heading as f64).to_radians();

    CapsuleState {
        position: DVec3::new(r, 0.0, 0.0),
        velocity: DVec3::new(
            separation_velocity * heading.sin(),
            separation_velocity * heading.cos(),
            0.0,
        ),
    }
}

fn altitude(position: DVec3) -> f64 {
    position.length() - EARTH_RADIUS
}

fn step_capsule(state: &mut CapsuleState, dt: f64) -> bool {
    let pos = state.position;
    let vel = state.velocity;

    let alt = altitude(pos);

    if alt <= 0.0 {
        return false;
    }

    let air_density = get_current_atmospheric_density(alt);

    let area = std::f64::consts::PI * CAPSULE_RADIUS * CAPSULE_RADIUS;

    let accel = calculate_cumulative_acceleration(
        air_density,
        CAPSULE_DRAG_COEFFICIENT,
        area,
        CAPSULE_MASS,
        vel,
        pos,
    );

    state.velocity = vel + accel * dt;
    state.position = pos + state.velocity * dt;

    true
}

fn simulate_internal(
    params: &SimulationParameters,
    spacing: f64,
    three_points_only: bool,
) -> Vec<DVec3> {
    let mut state = build_initial_state(params);

    let mut points = Vec::new();

    let mut prev = state.position;
    let mut accumulated = 0.0;

    points.push(state.position);

    let mut i = 0;

    loop {
        if !step_capsule(&mut state, DT) {
            break;
        }

        accumulated += state.velocity.length() * DT;

        if accumulated >= spacing {
            points.push(state.position);
            accumulated -= spacing;
        }

        if three_points_only && points.len() == 3 {
            break;
        }
    }

    points
}

pub fn simulate_three_points(params: &SimulationParameters) -> [DVec3; 3] {
    let spacing = 5000.0 / 2.0;

    let mut points = simulate_internal(params, spacing, true);

    let last = *points.last().unwrap();

    while points.len() < 3 {
        points.push(last);
    }

    [points[0], points[1], points[2]]
}

pub fn simulate_reentry(position: DVec3, velocity: DVec3, spacing: f64) -> Vec<DVec3> {
    let mut state = CapsuleState { position, velocity };

    let mut points = Vec::new();
    let mut accumulated = 0.0;

    points.push(state.position);

    loop {
        if !step_capsule(&mut state, DT) {
            break;
        }

        accumulated += state.velocity.length() * DT;

        if accumulated >= spacing {
            points.push(state.position);
            accumulated -= spacing;
        }
    }

    points
}

fn get_current_atmospheric_density(surface_altitude: f64) -> f64 {
    let (air_temp, air_pressure) = if surface_altitude > 100_000. {
        let temp = 10. + 0.002 * (surface_altitude - 100_000.);
        let pressure = 0.;
        (temp, pressure)
    } else if surface_altitude >= 25_000. {
        let temp = -131.21 + 0.00299 * surface_altitude;
        let pressure = 2.488 * ((temp + 273.1) / 216.6).powf(-11.388);
        (temp, pressure)
    } else if 11_000. < surface_altitude && surface_altitude < 25_000. {
        let temp = -56.46;
        let pressure = 22.65 * E.powf(1.73 - 0.000157 * surface_altitude);
        (temp, pressure)
    } else {
        let temp = 15.04 - 0.00649 * surface_altitude;
        let pressure = 101.29 * ((temp + 273.1) / 288.08).powf(5.256);
        (temp, pressure)
    };

    let air_density = air_pressure / (0.2869 * (air_temp + 273.1));

    air_density
}

fn get_current_altitude(current_position: DVec3) -> f64 {
    let vector_magnitude =
        (current_position.x.powi(2) + current_position.y.powi(2) + current_position.z.powi(2))
            .sqrt();
    let relative_altitude = vector_magnitude - (EARTH_RADIUS);

    relative_altitude
}

fn calculate_drag_acceleration(
    velocity: DVec3,
    air_density: f64,
    coefficient_of_drag: f64,
    cross_sectional_area: f64,
    vehicle_mass: f64,
) -> DVec3 {
    // Calculate the magnitude of velocity
    let velocity_magnitude = velocity.length();

    // If velocity magnitude is near zero, return zero vector (avoid division by zero)
    if velocity_magnitude.abs() < f64::EPSILON {
        return DVec3::ZERO;
    }

    // Drag force per unit mass (acceleration due to drag)
    let drag_magnitude =
        (coefficient_of_drag * air_density * velocity_magnitude.powi(2) * cross_sectional_area)
            / (2.0 * vehicle_mass);

    // Drag acceleration vector (opposes velocity direction)
    -velocity.normalize() * drag_magnitude
}

fn calculate_gravitational_acceleration(position: DVec3) -> DVec3 {
    let gravitational_constant = GRAVITATIONAL_CONST; // [Nm^2/kg^2]
    let earth_mass = EARTH_MASS; // [kg]

    // Calculate the magnitude of the position vector
    let position_magnitude = (position.x.powi(2) + position.y.powi(2) + position.z.powi(2)).sqrt();

    // Unit vector pointing toward the Earth's center (negative direction of position vector)
    let position_unit_vector = position / position_magnitude;

    // Calculate gravitational acceleration magnitude
    let acceleration_magnitude =
        -(gravitational_constant * earth_mass) / position_magnitude.powi(2);

    // Gravitational acceleration vector
    position_unit_vector * acceleration_magnitude
}

fn calculate_cumulative_acceleration(
    air_density: f64,
    drag_coefficient: f64,
    cross_sectional_area: f64,
    object_mass: f64,
    velocity: DVec3,
    position: DVec3,
) -> DVec3 {
    let acceleration_drag = calculate_drag_acceleration(
        velocity,
        air_density,
        drag_coefficient,
        cross_sectional_area,
        object_mass,
    );
    let acceleration_gravity = calculate_gravitational_acceleration(position);

    let acceleration_total = DVec3::new(
        acceleration_drag.x + acceleration_gravity.x,
        acceleration_drag.y + acceleration_gravity.y,
        acceleration_drag.z + acceleration_gravity.z,
    );

    acceleration_total
}
