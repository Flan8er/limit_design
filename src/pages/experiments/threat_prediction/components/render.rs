use bevy::prelude::*;
use bevy_polyline::prelude::*;

use crate::pages::experiments::threat_prediction::{
    components::{
        orbit::Orbit,
        orbit_params::OrbitParameters,
        polyline::{PolyLinePoint, PolyLineVecExt},
        real_time::simulate_three_points,
        simulation_params::{SimulationParameters, WORLD_SCALE},
    },
    threat_prediction::OrbitalNotification,
};

#[derive(Component)]
pub struct OrbitVisual;

pub fn clear_midcourse(
    mut commands: Commands,
    query: Query<Entity, With<OrbitVisual>>,
    simulation_parameters: Res<SimulationParameters>,
    mut orbit_parameters: ResMut<OrbitParameters>,
) {
    if !simulation_parameters.is_changed() {
        return;
    }

    // Despawn the existing scene.
    for entity in &query {
        commands.entity(entity).despawn();
    }

    // Send these parameters to the simulation to gather the 3 points.
    let points = simulate_three_points(&simulation_parameters);
    let new_orbit_parameters = match OrbitParameters::predict([
        PolyLinePoint::new(points[0].x, points[0].y, points[0].z),
        PolyLinePoint::new(points[1].x, points[1].y, points[1].z),
        PolyLinePoint::new(points[2].x, points[2].y, points[2].z),
    ]) {
        Some(t) => t,
        None => OrbitParameters::default(),
    };

    // Update the resource
    *orbit_parameters = new_orbit_parameters;
}

pub fn draw_midcourse(
    orbit_parameters: Res<OrbitParameters>,
    mut command: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    simulation_parameters: Res<SimulationParameters>,
    mut polylines: ResMut<Assets<Polyline>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event_writer: EventWriter<OrbitalNotification>,
) {
    if !orbit_parameters.is_changed() {
        return;
    }

    let fidelity = 10000;
    let mut orbit = Orbit::new(orbit_parameters.into_inner().clone(), fidelity);

    // Validate sub-orbital trajectory
    let mut sub_orbital = false;
    for point in orbit.points().iter() {
        if point.altitude() < 0. {
            // We intersect the surface
            sub_orbital = true;
            break;
        }
    }
    if !sub_orbital {
        // We need to break out of the simulation or the simulation will break out.
        event_writer.write(OrbitalNotification);
        return;
    }

    orbit.normalize_inclination();
    orbit.apply_inclination(simulation_parameters.inclination as f64);

    // We need to find the point in the orbit that is closest to the separation altitude.
    // We will use this as our point to set latitude and longitude at.
    // We will then need to re-orient to where the focus is at (0, 0, 0) ENSURING THE LAT/LONG/ALTITUDE of the point dont shift.
    let mut separation_point = orbit.separation(&simulation_parameters);
    separation_point.apply_lat_long(&simulation_parameters);
    let ideal_separation_point: PolyLinePoint = PolyLinePoint::compute_separation_point(
        &simulation_parameters,
        separation_point.magnitude(),
    );

    // Find the required rotation matrix to perform the clocking procedure on the separation point.
    // This rotation matrix will then be used on all the points.
    orbit.clock(&simulation_parameters);
    orbit.construct_timeline(&separation_point);

    // Apply relavant mutators - Coriolis
    // From the time it takes to go from one point to a new point, how much has the earth shifted (delta from your velocity)?
    let original_orbit = orbit.clone();
    orbit.apply_coriolis(&simulation_parameters);
    // orbit.add_origin();
    orbit.apply_reentry();

    command.spawn((
        PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline {
                vertices: orbit.points().to_vec3(),
            })),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 2.0,
                color: LinearRgba::WHITE,
                ..default()
            })),
            ..default()
        },
        OrbitVisual,
    ));

    command.spawn((
        PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline {
                vertices: original_orbit.points().to_vec3(),
            })),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 2.0,
                color: LinearRgba::RED,
                ..default()
            })),
            ..default()
        },
        OrbitVisual,
    ));

    let earth_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1., 0.4, 0.2),
        ..default()
    });

    let sphere = meshes.add(Sphere::default().mesh().uv(128, 64));

    command.spawn((
        Mesh3d(sphere),
        MeshMaterial3d(earth_material),
        Transform {
            translation: ideal_separation_point.to_vec3(),
            scale: Vec3::splat((35000. * WORLD_SCALE * 2.) as f32),
            ..default()
        },
        OrbitVisual,
    ));
}
