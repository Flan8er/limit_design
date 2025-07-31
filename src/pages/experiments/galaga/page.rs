use leptos::prelude::*;
use leptos_bevy_canvas::prelude::BevyCanvas;
use thaw::{
    Button, ButtonAppearance, ConfigProvider, Dialog, DialogActions, DialogBody, DialogContent,
    DialogSurface, DialogTitle,
};

use crate::pages::experiments::{galaga::system::render_game, page::ExpandedView};

#[component]
pub fn GalagaExperiment() -> impl IntoView {
    view! {
        <ExpandedView title="Galaga" description="A 3D Galaga that uses real kinematics.">
            <iframe class="m-0 p-0 w-full h-full z-[0]" src="/galaga-frame"/>
        </ExpandedView>
    }
}

#[component]
pub fn GalagaFrame() -> impl IntoView {
    let controls_open = RwSignal::new(true);

    view! {
        <div class="w-full h-full relative">
            <ConfigProvider>
                <Dialog open=controls_open>
                    <DialogSurface>
                        <DialogBody>
                            <DialogTitle>"Controls"</DialogTitle>
                            <DialogContent>
                                <p>"Pitch up: 'S'"</p>
                                <p>"Pitch down: 'W'"</p>
                                <p>"Roll right: 'D'"</p>
                                <p>"Roll left: 'A'"</p>
                                <p>"Forward: 'Left Shift'"</p>
                                <p>"Backward: 'Left Ctrl'"</p>
                                <p>"Pause: 'Esc'"</p>
                            </DialogContent>
                            <DialogActions>
                                <Button
                                    appearance=ButtonAppearance::Primary
                                    on_click=move |_| controls_open.set(false)
                                >"Gotcha!"</Button>
                            </DialogActions>
                        </DialogBody>
                    </DialogSurface>
                </Dialog>
            </ConfigProvider>

            <BevyCanvas
                init=move || {
                    render_game()
                }
            />
        </div>
    }
}
