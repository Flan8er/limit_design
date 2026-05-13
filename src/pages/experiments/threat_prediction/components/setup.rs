use bevy::prelude::*;

use crate::pages::experiments::threat_prediction::components::{
    asset_loader::SceneAssets, orbit_params::OrbitParameters,
    simulation_params::SimulationParameters,
};

pub fn setup(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    // Insert a SimulationParameter resource to be used to calculate default trajectory.
    // This can then be watched for changed for when recalculaiton is necessary.
    {
        commands.insert_resource(SimulationParameters::default());
    }

    // We have a central resource for holding the elliptical orbit data for the simulation.
    // On startup we either calculate a predefined trajectory or initialize a default one
    // to be updated at a later date.
    {
        // let midcourse_profile = match OrbitParameters::predict([
        //     PolyLinePoint::new(6_750_000., 0., 0.),
        //     PolyLinePoint::new(6_500_000., 2_000_000., 0.),
        //     PolyLinePoint::new(-1_500_000., 6_000_000., 0.),
        // ]) {
        //     Some(t) => t,
        //     None => OrbitParameters::default(),
        // };
        // commands.insert_resource(midcourse_profile);

        // While converting to the new stuff this just needs to be set to default.
        // On first render this will immediately get overwritten before calculations
        // are performed on it so it doesnt matter if it's valid data or not.
        commands.insert_resource(OrbitParameters::default());
    }

    // Spawn the earth at some defined world scale.
    {
        let offset_rotation: f32 = 71.5_f32.to_radians();
        commands.spawn((
            SceneRoot(scene_assets.earth.clone()),
            Transform {
                translation: Vec3::ZERO,
                rotation: Quat::from_rotation_y(offset_rotation),
                scale: Vec3::splat(1.275),
                ..default()
            },
        ));
    }
}
