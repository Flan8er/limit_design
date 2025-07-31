use bevy::prelude::*;
use std::f32::consts::E;

use crate::pages::experiments::reentry::earth::EARTH_DIAMETER;

pub fn get_current_atmospheric_density(surface_altitude: f32) -> f32 {
    let (air_temp, air_pressure) = if surface_altitude > 100000. {
        let temp = 10. + 0.002 * (surface_altitude - 100000.);
        let pressure = 0.;
        (temp, pressure)
    } else if surface_altitude >= 25000. {
        let temp = -131.21 + 0.00299 * surface_altitude;
        let pressure = 2.488 * ((temp + 273.1) / 216.6).powf(-11.388);
        (temp, pressure)
    } else if 11000. < surface_altitude && surface_altitude < 25000. {
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

pub fn get_current_altitude(current_position: Vec3) -> f32 {
    let vector_magnitude =
        (current_position.x.powi(2) + current_position.y.powi(2) + current_position.z.powi(2))
            .sqrt();
    let relative_altitude = vector_magnitude - (EARTH_DIAMETER / 2.);

    relative_altitude
}

fn calculate_drag_acceleration(
    velocity: Vec3,
    air_density: f32,
    coefficient_of_drag: f32,
    cross_sectional_area: f32,
    vehicle_mass: f32,
) -> Vec3 {
    // Calculate the magnitude of velocity
    let velocity_magnitude = velocity.length();

    // If velocity magnitude is near zero, return zero vector (avoid division by zero)
    if velocity_magnitude.abs() < f32::EPSILON {
        return Vec3::ZERO;
    }

    // Drag force per unit mass (acceleration due to drag)
    let drag_magnitude =
        (coefficient_of_drag * air_density * velocity_magnitude.powi(2) * cross_sectional_area)
            / (2.0 * vehicle_mass);

    // Drag acceleration vector (opposes velocity direction)
    velocity.normalize() * drag_magnitude
}

fn calculate_gravitational_acceleration(position: Vec3) -> Vec3 {
    let gravitational_constant: f32 = 6.6743 * 10_f32.powi(-11); // [Nm^2/kg^2]
    let earth_mass: f32 = 5.97219 * 10_f32.powi(24); // [kg]

    // Calculate the magnitude of the position vector
    let position_magnitude: f32 =
        (position.x.powi(2) + position.y.powi(2) + position.z.powi(2)).sqrt();

    // Unit vector pointing toward the Earth's center (negative direction of position vector)
    let position_unit_vector = -position / position_magnitude;

    // Calculate gravitational acceleration magnitude
    let acceleration_magnitude = (gravitational_constant * earth_mass) / position_magnitude.powi(2);

    // Gravitational acceleration vector
    -position_unit_vector * acceleration_magnitude
}

pub fn calculate_cumulative_acceleration(
    air_density: f32,
    drag_coefficient: f32,
    cross_sectional_area: f32,
    object_mass: f32,
    velocity: Vec3,
    position: Vec3,
) -> Vec3 {
    let acceleration_drag = calculate_drag_acceleration(
        velocity,
        air_density,
        drag_coefficient,
        cross_sectional_area,
        object_mass,
    );
    let acceleration_gravity = calculate_gravitational_acceleration(position);

    let acceleration_total = Vec3::new(
        acceleration_drag.x + acceleration_gravity.x,
        acceleration_drag.y + acceleration_gravity.y,
        acceleration_drag.z + acceleration_gravity.z,
    );

    acceleration_total
}
