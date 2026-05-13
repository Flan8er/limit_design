use std::f64::consts::TAU;

use bevy::ecs::resource::Resource;
use nalgebra::Matrix3;

use crate::pages::experiments::threat_prediction::components::{
    polyline::PolyLinePoint,
    simulation_params::{EARTH_MASS, GRAVITATIONAL_CONST},
};

#[derive(Resource, Copy, Clone, Debug)]
pub struct OrbitParameters {
    pub p: f64,
    pub e: f64,
    pub w: f64,
}
impl Default for OrbitParameters {
    fn default() -> Self {
        Self {
            p: Default::default(),
            e: Default::default(),
            w: Default::default(),
        }
    }
}
impl OrbitParameters {
    pub fn predict(points: [PolyLinePoint; 3]) -> Option<Self> {
        let x1 = points[0].x;
        let y1 = points[0].y;
        let x2 = points[1].x;
        let y2 = points[1].y;
        let x3 = points[2].x;
        let y3 = points[2].y;

        let m_inv = Matrix3::new(
            (x1.powi(2) + y1.powi(2)).sqrt(),
            x1,
            y1,
            (x2.powi(2) + y2.powi(2)).sqrt(),
            x2,
            y2,
            (x3.powi(2) + y3.powi(2)).sqrt(),
            x3,
            y3,
        )
        .try_inverse()?;

        let m0 = m_inv.row(0);
        let m1 = m_inv.row(1);
        let m2 = m_inv.row(2);

        let m_prime0 = m0.sum();
        let m_prime1 = m1.sum();
        let m_prime2 = m2.sum();

        // Correct orientation to make sure orbit is point towards earth. lol
        let sign = m_prime0.signum();
        let m_prime0 = m_prime0 * sign;
        let m_prime1 = m_prime1 * sign;
        let m_prime2 = m_prime2 * sign;

        let p = 1.0 / m_prime0;
        let e = (m_prime1.powi(2) + m_prime2.powi(2)).sqrt() / m_prime0;
        let w = m_prime2.atan2(m_prime1);

        Some(Self { p, e, w })
    }

    pub fn semi_major_axis(&self) -> f64 {
        self.p / (1.0 - self.e.powi(2))
    }

    pub fn to_cartesian(&self, relative_angle: f64) -> PolyLinePoint {
        let r = self.p / (1.0 + self.e * (relative_angle - self.w).cos());

        let x = r * relative_angle.cos();
        let y = r * relative_angle.sin();

        PolyLinePoint::new(x, y, 0.)
    }

    pub fn orbital_period(&self) -> f64 {
        TAU * (self.semi_major_axis().powi(3) / (GRAVITATIONAL_CONST * EARTH_MASS)).sqrt()
    }
}
