use leptos::{html::Div, prelude::*};

use crate::pages::about::components::{page::Page, scroll::compute_scroll_animation_progress};

#[component]
pub fn Skills() -> impl IntoView {
    let parent_scroll = expect_context::<Signal<f64>>();

    // Seeded initial transform positions for deterministic behavior
    // let initial_transforms = vec![
    //     ("React", 890.42, -945.17, -28.3),
    //     ("Next.js", -1023.76, 874.29, 17.9),
    //     ("TypeScript", 712.33, -813.64, -12.4),
    //     ("Node.js", -832.61, -663.38, 7.5),
    //     ("Python", -978.02, -707.45, 22.1),
    //     ("Rust", 623.59, -924.71, 11.3),
    //     ("Leptos", 1005.88, -989.67, 29.4),
    //     ("Swift", 850.91, -509.63, -26.8),
    //     ("Electron", 1032.25, -554.87, -21.5),
    //     ("Tauri", -748.61, 951.13, 14.7),
    //     ("Tailwind CSS", 792.84, 836.52, -18.2),
    //     ("Bevy", 950.18, -461.94, 30.0),
    // ];
    // let initial_transforms = vec![
    //     ("React", -998.14, 735.65, -17.2),
    //     ("Next.js", 841.90, 895.26, 25.4),
    //     ("TypeScript", -761.22, -984.77, -10.5),
    //     ("Node.js", 1028.63, -658.09, 11.9),
    //     ("Python", 682.49, 1034.76, -28.7),
    //     ("Rust", -879.83, 712.38, 14.3),
    //     ("Leptos", 958.17, -1005.93, 30.0),
    //     ("Swift", -1022.11, 824.50, -19.8),
    //     ("Electron", 834.06, -742.88, 7.1),
    //     ("Tauri", -699.57, 951.90, 22.0),
    //     ("Tailwind CSS", 989.27, -694.61, -15.4),
    //     ("Bevy", -832.91, -861.02, 5.7),
    // ];
    let initial_transforms = vec![
        ("React", 975.30, -856.42, 18.6),
        ("Next.js", -1004.83, -733.91, -12.3),
        ("TypeScript", 684.27, 1002.65, 30.0),
        ("Node.js", -896.04, 958.81, -25.9),
        ("Python", 744.56, -1050.14, 16.1),
        ("Rust", -943.80, -724.66, -29.7),
        ("Leptos", 872.90, 766.99, 9.8),
        ("Swift", -1033.17, 697.72, 13.3),
        ("Electron", 1001.47, -686.04, -7.0),
        ("Tauri", 816.21, 933.80, -23.6),
        ("Tailwind CSS", -765.14, -1021.58, 21.4),
        ("Bevy", 915.88, 829.34, -6.2),
    ];

    let transforms: RwSignal<Vec<(String, f64, f64, f64)>> = RwSignal::new(
        initial_transforms
            .clone()
            .into_iter()
            .map(|(s, x, y, r)| (s.to_string(), x, y, r))
            .collect::<Vec<_>>(),
    );

    let el = NodeRef::<Div>::new();

    Effect::new({
        let initial_transforms = initial_transforms.clone();
        move |_| {
            let scroll_y = parent_scroll.get();
            let Some(progress) = compute_scroll_animation_progress(&el, scroll_y, 0.2, 0.75) else {
                return;
            };

            // Update the transforms to the % value from their current position to 0
            transforms.set(
                initial_transforms
                    .iter()
                    .map(|(name, x0, y0, r0)| {
                        let x = x0 * (1.0 - progress);
                        let y = y0 * (1.0 - progress);
                        let r = r0 * (1.0 - progress);
                        (name.to_string(), x, y, r)
                    })
                    .collect(),
            );
        }
    });

    view! {
        <Page>
            <div node_ref=el class="w-full h-full flex items-center justify-center bg-gradient-to-b from-[rgba(183,45,177,0.1)] to-pink-600/10 py-6">
                <div class="max-w-4xl px-8 z-10">
                    <h2 class="text-5xl font-bold mb-12 text-center bg-gradient-to-r from-orange-400 to-yellow-400 bg-clip-text text-transparent">
                        "Skills & Technologies"
                    </h2>

                    <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
                        <For
                            each=move || transforms.get()
                            key=|(name, x_val, y_val, r_val)| format!("{}-{:.2}-{:.2}-{:.2}", name, x_val, y_val, r_val)
                            children=move |(name, x, y, r)| {
                                let transform_style = format!(
                                    "transform: translate({:.2}px, {:.2}px) rotate({:.2}deg); transition: transform 0.3s ease-out;",
                                    x, y, r
                                );
                                view! {
                                    <div style=transform_style class="bg-white/5 backdrop-blur-sm rounded-lg p-4 text-center border border-white/10">
                                        <span class="text-primary-text font-medium">{name}</span>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
            </div>
        </Page>
    }
}

#[component]
pub fn SkillLabel(name: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white/5 backdrop-blur-sm rounded-lg p-4 text-center border border-white/10">
            <span class="text-white font-medium">{name}</span>
        </div>
    }
}
