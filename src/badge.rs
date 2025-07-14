use leptos::prelude::*;
use leptos_verlet::prelude::*;

use crate::components::id_card::SpawnIdCard;

#[component]
pub fn BadgeSimulation() -> impl IntoView {
    match window().document().unwrap().body() {
        Some(body) => {
            let _ = body.style().set_property("background", "none transparent");
        }
        None => (),
    };
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
