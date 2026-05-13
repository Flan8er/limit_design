use bevy::ecs::{event::Event, resource::Resource};

pub const WORLD_SCALE: f64 = 1.0 / 10_000_.0; // one bevy unit to every 10 thousand meters.
pub const EARTH_MASS: f64 = 5.97219e24; // kg
pub const EARTH_RADIUS: f64 = 6_371_000.0; // meters
pub const GRAVITATIONAL_CONST: f64 = 6.6743e-11; // (m^3)(kg^-1)(s^-2)
pub const EARTH_ANGULAR_VELOCITY: f64 = 7.2921e-5; //0.1; //7.2921e-5;

#[derive(Resource, Debug, Clone, Event)]
pub struct SimulationParameters {
    pub inclination: u16,         // 0-90
    pub separation_altitude: u32, // meter
    pub separation_velocity: u32, // < escape velocity
    pub heading: f64,             // 10-80
    pub launch_latitude: f32,
    pub launch_logitude: f32,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            inclination: 90, //35,
            separation_altitude: 125_000,
            separation_velocity: 7_000,
            heading: 15.,
            launch_latitude: 28.608280,
            launch_logitude: -80.604133,
            // launch_logitude: -100.,
            // launch_latitude: -10.,
        }
    }
}

impl SimulationParameters {
    pub fn new(
        inclination: u16,
        separation_altitude: u32,
        separation_velocity: u32,
        heading: f64,
        launch_latitude: f32,
        launch_logitude: f32,
    ) -> Result<Self, &'static str> {
        let new = Self {
            inclination,
            separation_altitude,
            separation_velocity,
            heading,
            launch_latitude,
            launch_logitude,
        };

        // new.validate()?;

        Ok(new)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.inclination > 90 || self.heading > 90. {
            return Err("Angle out of range");
        }

        let mu = 3.986e14;

        let r = EARTH_RADIUS + self.separation_altitude as f64;

        let v_orbital = (mu / r).sqrt();

        // core constraint: must be sub-orbital
        if self.separation_velocity as f64 >= v_orbital {
            return Err("Velocity would result in orbital or escape trajectory");
        }

        Ok(())
    }
}
