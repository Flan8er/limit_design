use bevy::{
    ecs::resource::Resource,
    math::{DQuat, DVec3, Vec3},
};
use std::ops::Sub;

use crate::pages::experiments::threat_prediction::components::{
    orbit_params::OrbitParameters,
    simulation_params::{
        SimulationParameters, EARTH_MASS, EARTH_RADIUS, GRAVITATIONAL_CONST, WORLD_SCALE,
    },
};

#[derive(Resource, Copy, Clone, Debug)]
pub struct PolyLinePoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Sub for PolyLinePoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        PolyLinePoint::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl PolyLinePoint {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn velocity(&self, profile: &OrbitParameters) -> f64 {
        let gravitational_param = GRAVITATIONAL_CONST * EARTH_MASS;
        let distance = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        let a = profile.semi_major_axis();

        (gravitational_param * ((2. / distance) - (1. / a))).sqrt()
    }

    pub fn to_world_scale(self) -> Self {
        Self::new(self.x * WORLD_SCALE, self.y * WORLD_SCALE, 0.)
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    pub fn to_dvec3(&self) -> DVec3 {
        DVec3::new(self.x, self.y, self.z)
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();

        if len == 0.0 {
            return self;
        }

        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn compute_separation_point(
        simulation_parameters: &SimulationParameters,
        radius: f64,
    ) -> Self {
        // Convert degrees -> radians
        let latitude = (simulation_parameters.launch_latitude as f64).to_radians();

        let longitude = (simulation_parameters.launch_logitude as f64).to_radians();

        // Spherical -> Cartesian
        let x = radius * latitude.cos() * longitude.cos();
        let y = radius * latitude.sin();
        let z = radius * latitude.cos() * longitude.sin();

        Self { x, y, z }
    }

    pub fn apply_lat_long(&mut self, simulation_parameters: &SimulationParameters) {
        let longitude = (simulation_parameters.launch_logitude as f64).to_radians();
        let latitude = (simulation_parameters.launch_latitude as f64).to_radians();
        let rotation_matrix = DQuat::from_rotation_y(-longitude) * DQuat::from_rotation_z(latitude);

        let rotated = rotation_matrix * self.to_dvec3();
        *self = Self::new(rotated.x, rotated.y, rotated.z);
    }

    /// Altitude of a point in meters
    pub fn altitude(&self) -> f64 {
        (self.magnitude() / WORLD_SCALE) - EARTH_RADIUS
    }
}

pub trait PolyLineVecExt {
    fn to_vec3(&self) -> Vec<Vec3>;
}
impl PolyLineVecExt for Vec<PolyLinePoint> {
    fn to_vec3(&self) -> Vec<Vec3> {
        let mut new_vec = Vec::with_capacity(self.len());

        for polyline in self {
            new_vec.push(Vec3::new(
                polyline.x as f32,
                polyline.y as f32,
                polyline.z as f32,
            ))
        }

        new_vec
    }
}
