use leptos::prelude::*;
use leptos_verlet::prelude::*;

use crate::components::id_card::SpawnIdCard;

#[component]
pub fn BadgeSimulation() -> impl IntoView {
    view! {
        <div class="w-full h-full">
            <VerletConfigProvider
                simulation_settings=SimulationSettings{
                    jerk_damping: 0.5,
                    simulation_bounds: SimulationBounds::new(false, false, false),
                    ..default()
                }
            />

            <SpawnIdCard/>
        </div>
    }
}
