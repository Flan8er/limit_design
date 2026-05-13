use bevy::math::{DQuat, DVec3, Dir3, InvalidDirectionError};
use std::f64::consts::{PI, TAU};

use crate::pages::experiments::threat_prediction::components::{
    orbit_params::OrbitParameters,
    polyline::PolyLinePoint,
    real_time::simulate_reentry,
    simulation_params::{
        SimulationParameters, EARTH_ANGULAR_VELOCITY, EARTH_MASS, EARTH_RADIUS,
        GRAVITATIONAL_CONST, WORLD_SCALE,
    },
};

#[derive(Clone)]
pub struct Orbit(Vec<PolyLinePoint>, OrbitParameters);
impl Orbit {
    pub fn new(params: OrbitParameters, fidelity: usize) -> Self {
        let mut orbital_points: Vec<PolyLinePoint> = Vec::with_capacity(fidelity);

        // Compute the basic trajectory without any aditional mutators.
        for i in 0..=fidelity {
            let theta = (i as f64 / fidelity as f64) * TAU;

            let point = params.to_cartesian(theta).to_world_scale();

            orbital_points.push(point);
        }

        Self(orbital_points, params)
    }

    pub fn points(&self) -> &Vec<PolyLinePoint> {
        &self.0
    }

    pub fn semi_latus_rectum(&self) -> f64 {
        self.1.p
    }

    pub fn argument_of_periapsis(&self) -> f64 {
        self.1.w
    }

    pub fn eccentricity(&self) -> f64 {
        self.1.e
    }

    pub fn semi_major_axis(&self) -> f64 {
        self.1.semi_major_axis()
    }

    pub fn orbital_period(&self) -> f64 {
        self.1.orbital_period()
    }

    pub fn perigee(&self) -> PolyLinePoint {
        self.points()[self.perigee_index()].clone()
    }
    pub fn perigee_index(&self) -> usize {
        let mut shortest_distance = 0.;
        let mut shortest_distance_index = 0;
        let points = self.points();

        for (index, point) in points.iter().enumerate() {
            let distance = point.magnitude();
            if index == 0 {
                shortest_distance = distance;
                shortest_distance_index = index;

                continue;
            }

            if distance < shortest_distance {
                shortest_distance = distance;
                shortest_distance_index = index;
            }
        }

        shortest_distance_index
    }

    pub fn orbital_plane(&self) -> Result<Dir3, InvalidDirectionError> {
        let points = self.points();
        let len = points.len();

        if len < 3 {
            return Err(InvalidDirectionError::Zero);
        };

        let three_points = [points[0], points[len / 3], points[(2 * len) / 3]];
        let u = three_points[1] - three_points[0];
        let v = three_points[2] - three_points[0];

        let w = u.cross(v).normalize();

        Dir3::new(w.to_vec3())
    }

    pub fn normalize_to_perigee(&self, point: PolyLinePoint) -> PolyLinePoint {
        let perigee = self.perigee();

        point - perigee
    }

    pub fn true_anomaly(&self, point: PolyLinePoint) -> f64 {
        let perigee = self.perigee();

        // Get the orbital plane normal
        let normal = match self.orbital_plane() {
            Ok(n) => n.as_vec3().as_dvec3(),
            Err(_) => return 0.0,
        };

        // Use perigee direction as the reference direction (0 angle)
        let reference = perigee.to_dvec3().normalize();

        // Get the perpendicular axis within the orbital plane
        let perp = normal.cross(reference).normalize();

        // Project point onto the orbital plane axes
        let point_vec = point.to_dvec3().normalize();
        let x = point_vec.dot(reference);
        let y = point_vec.dot(perp);

        y.atan2(x)
    }

    pub fn eccentric_anomaly(&self, point: PolyLinePoint) -> f64 {
        let true_anomaly = self.true_anomaly(point);
        let eccentricity = self.eccentricity();

        let e = 2.
            * (((1. - eccentricity) / (1. + eccentricity)).sqrt() * (true_anomaly / 2.).tan())
                .atan();
        e
    }

    pub fn mean_anomaly(&self, point: PolyLinePoint) -> f64 {
        let eccentricity = self.eccentricity();
        let point_eccentric_anomaly = self.eccentric_anomaly(point);

        point_eccentric_anomaly - (eccentricity * point_eccentric_anomaly.sin())
    }

    pub fn construct_timeline(&mut self, separation_point: &PolyLinePoint) {
        let points = self.points().clone();

        let radius = EARTH_RADIUS * WORLD_SCALE;

        let mut points_above_surface = points
            .iter()
            .filter(|point| point.magnitude() >= radius)
            .cloned()
            .collect::<Vec<PolyLinePoint>>();

        let mut closest_index: usize = 0;
        let mut closest_range = PolyLinePoint::new(
            points_above_surface[0].x - separation_point.x,
            points_above_surface[0].y - separation_point.y,
            points_above_surface[0].z - separation_point.z,
        )
        .magnitude();
        for (index, point) in points_above_surface.iter().enumerate() {
            let range = PolyLinePoint::new(
                point.x - separation_point.x,
                point.y - separation_point.y,
                point.z - separation_point.z,
            )
            .magnitude();

            if range < closest_range {
                closest_range = range;
                closest_index = index
            }
        }

        // Shift the vec to put the separation point at the start.
        points_above_surface.rotate_left(closest_index);

        // Find the biggest gap and truncate everything after it.
        // Basically removing any points that occure prior to separation
        let biggest_gap_index = points_above_surface
            .windows(2)
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                let da = (a[1].to_dvec3() - a[0].to_dvec3()).length();
                let db = (b[1].to_dvec3() - b[0].to_dvec3()).length();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i + 1)
            .unwrap_or(points_above_surface.len());

        points_above_surface.truncate(biggest_gap_index);

        self.0 = points_above_surface
    }

    pub fn time_since_separation(
        &self,
        point: PolyLinePoint,
        _simulation_parameters: &SimulationParameters,
    ) -> f64 {
        let index = self
            .0
            .iter()
            .position(|p| {
                (p.x - point.x).abs() < 1e-6
                    && (p.y - point.y).abs() < 1e-6
                    && (p.z - point.z).abs() < 1e-6
            })
            .unwrap_or(0);

        (index as f64 / self.0.len() as f64) * self.orbital_period()
    }

    pub fn apply_coriolis(&mut self, simulation_parameters: &SimulationParameters) {
        let mut new_points: Vec<PolyLinePoint> = Vec::new();

        for point in self.0.iter() {
            let travel_time = self.time_since_separation(point.clone(), simulation_parameters);

            let angle = -EARTH_ANGULAR_VELOCITY * travel_time;
            let rotation = DQuat::from_rotation_y(angle);
            let rotated = rotation * point.to_dvec3();

            new_points.push(PolyLinePoint::new(rotated.x, rotated.y, rotated.z));
        }

        self.0 = new_points
    }

    pub fn normalize_inclination(&mut self) {
        let points = self.points();
        let mut points_prime: Vec<PolyLinePoint> = Vec::with_capacity(points.len());

        for point in points.iter() {
            points_prime.push(PolyLinePoint::new(point.x, 0.0, -point.y));
        }

        self.0 = points_prime
    }

    /// Dont use after clocking!
    pub fn separation(&self, simulation_parameters: &SimulationParameters) -> PolyLinePoint {
        // Inclination has already been applied so we need to rotate it back lol.
        let reverse_inclination = -(simulation_parameters.inclination as f64);
        let mut normalized_orbit = self.clone();
        normalized_orbit.apply_inclination(reverse_inclination);

        let points = normalized_orbit.points();
        let target_altitude =
            (EARTH_RADIUS + simulation_parameters.separation_altitude as f64) * WORLD_SCALE;

        let unadjusted_point = points
            .iter()
            .min_by(|a, b| {
                // let dist_a = (a.x - target_altitude).powi(2);
                // let dist_b = (b.x - target_altitude).powi(2);
                let dist_a = (a.magnitude() - target_altitude).powi(2);
                let dist_b = (b.magnitude() - target_altitude).powi(2);

                dist_a
                    .partial_cmp(&dist_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .expect("Points will exist.")
            .clone();
        unadjusted_point
    }

    pub fn apply_inclination(&mut self, inclination: f64) {
        // let points: &Vec<PolyLinePoint> = &self.0;
        let inclination_rotation = DQuat::from_rotation_x(inclination * (PI / 180.));

        for point in &mut self.0 {
            let rotated = inclination_rotation * DVec3::new(point.x, point.y, point.z);

            point.x = rotated.x;
            point.y = rotated.y;
            point.z = rotated.z;
        }
    }

    pub fn clock(&mut self, simulation_parameters: &SimulationParameters) {
        for point in self.0.iter_mut() {
            point.apply_lat_long(simulation_parameters);
        }
    }

    pub fn add_origin(&mut self) {
        self.0.push(PolyLinePoint::new(0., 0., 0.));
    }

    pub fn apply_reentry(&mut self) {
        let points = self.points();

        let reentry_index_from_end = points
            .iter()
            .rev()
            .position(|point| point.altitude() >= 100_000.);

        let (before_reentry_index, reentry_index) = match reentry_index_from_end {
            Some(rev_index) => {
                // Convert back to forward index
                let forward_index = points.len() - 1 - rev_index;
                (forward_index, forward_index + 1)
            }
            None => return, // No point above altitude found
        };
        if reentry_index >= points.len() {
            return;
        }

        // Index 0 is separation, so if reentry or the point prior to it is 0, then an issue occurred.
        if before_reentry_index == 0 || reentry_index == 0 {
            return; // Return without modifying the points.
        }

        // Build the arguments to pass into the real-time simulation.
        let reentry_point = &points[reentry_index];
        let before_reentry_point = &points[before_reentry_index];

        let speed = self.instantaneous_velocity(reentry_point);
        let velocity_vector =
            Self::instantaneous_velocity_vector(before_reentry_point, reentry_point, speed);

        let reentry_profile = simulate_reentry(
            reentry_point.to_dvec3() / WORLD_SCALE,
            velocity_vector,
            5000.0,
        );
        let reentry_points = reentry_profile
            .iter()
            .map(|p| PolyLinePoint::new(p.x * WORLD_SCALE, p.y * WORLD_SCALE, p.z * WORLD_SCALE))
            .collect::<Vec<PolyLinePoint>>();

        // Trim out the points at, and after, reentry starts.
        // We will then replace them with the simulated reentry points.
        let truncated_points = {
            let mut p = points.clone();
            p.truncate(reentry_index);
            p
        };

        let new_orbit = [truncated_points.as_slice(), reentry_points.as_slice()].concat();
        self.0 = new_orbit;
        // self.0 = truncated_points
    }

    // Vis-Viva calculation.
    pub fn instantaneous_velocity(&self, point: &PolyLinePoint) -> f64 {
        let semi_major_axis = self.semi_major_axis() / WORLD_SCALE;
        let distance = point.magnitude() / WORLD_SCALE;
        let gravitational_parameter = GRAVITATIONAL_CONST * EARTH_MASS;

        let velocity =
            (gravitational_parameter * ((2. / distance) - (1. / semi_major_axis))).sqrt();

        velocity
    }

    pub fn instantaneous_velocity_vector(
        before_point: &PolyLinePoint,
        point: &PolyLinePoint,
        speed: f64,
    ) -> DVec3 {
        let direction = (point.to_dvec3() - before_point.to_dvec3()).normalize();
        direction * speed
    }
}
