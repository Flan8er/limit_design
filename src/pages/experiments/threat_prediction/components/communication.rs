use bevy::prelude::*;

use crate::pages::experiments::threat_prediction::components::simulation_params::SimulationParameters;

pub fn update_simulation_parameters(
    mut events: EventReader<SimulationParameters>,
    mut prev: ResMut<SimulationParameters>,
) {
    for evt in events.read() {
        *prev = evt.clone();
    }
}
