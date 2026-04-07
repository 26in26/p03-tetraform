use dioxus::prelude::*;
use rand::{Rng, RngExt};

#[component]
pub fn Frame(children: Element) -> Element {
    rsx! {
        // 1. MAIN CONTAINER: Deep Obsidian Base
        div {
            class: "relative min-h-screen w-full bg-[#120c0a] overflow-hidden font-mono",

            // 2. THE GRID LAYER: Warm Amber "Frames"
            div {
                class: "absolute inset-0 opacity-20 pointer-events-none
                        bg-[linear-gradient(to_right,#ff8c3233_4px,transparent_1px),linear-gradient(to_bottom,#ff8c3233_4px,transparent_1px)] 
                        bg-[size:40px_40px]" 
            }

            // 3. THE VIGNETTE: Soft glow in the center, dark at the edges
            div {
                class: "absolute inset-0 pointer-events-none bg-radial from-transparent to-[#120c0a]"
            }

            // 4. THE LAVA SHIMMER: Subtle orange glow rising from the bottom
            div {
                class: "absolute inset-0 pointer-events-none animate-lava-glow bg-linear-to-t from-[#c42b1015] to-transparent"
            }

            // 5. CONTENT AREA: Where landing page lives
            div { class: "relative z-10 flex flex-col items-center justify-center min-h-screen",
                {children}
            }

            // CLIFFS: Positioned absolutely so they stay on the edges
            div { class: "hidden md:block",
                CurvedCliff { side: Side::Left, }
                CurvedCliff { side: Side::Right }
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

#[component]
pub fn CliffSide(side: Side, depth: usize) -> Element {
    let cliff_asset = asset!("/assets/cliff.png"); // Simplified for brevity
    let mut is_loaded = use_signal(|| false);

    use_effect(move || {
        is_loaded.set(true);
    });

    let opacity = if is_loaded() {
        "opacity-100"
    } else {
        "opacity-0"
    };

    let cliff_data = use_memo(move || {
        let mut rng = rand::rng();
        let mut layers = Vec::new();
        for _ in 0..depth {
            let mut tiles = Vec::new();
            let mut current_y = -20;
            while current_y < 2000 {
                tiles.push((current_y, rng.random_range(-10..10)));
                current_y += 45;
            }
            layers.push(tiles);
        }
        layers
    });

    // Determine if we are on the left or right
    let side_style = match side {
        Side::Left => "left-0 -scale-x-100",
        Side::Right => "right-0 ", // Flip it for the right side
    };

    rsx! {
        // We give the container an explicit width so children don't collapse
        div { class: "absolute top-0 {side_style} {opacity} h-full w-[128px] pointer-events-none",
            for (layer_idx, tiles) in cliff_data().iter().enumerate() {
                div {
                    class: "relative h-full",
                    // Use 'layer_idx' to push back layers further "into" the cliff
                    style: "margin-left: -{layer_idx * 25}px; z-index: {10 - layer_idx}; filter: brightness({1.0 - (layer_idx as f32 * 0.2)});",

                    for (y, x_jitter) in tiles {
                        img {
                            // CRITICAL: max-w-none stops the image from shrinking to 0
                            class: "absolute pixel-perfect max-w-none",
                            style: "top: {y}px; left: {x_jitter}px;",
                            src: "{cliff_asset}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CurvedCliff(side: Side) -> Element {
    let cliff_asset = asset!("/assets/cliff.png");
    let is_left = side == Side::Left;

    let cliff_data = use_memo(move || {
        let mut rng = rand::rng();
        let mut points = Vec::new();
        let screen_h = 1200.0;

        let mut current_y: f32 = -64.0;
        while current_y < screen_h {
            let progress = (current_y / 1080.0).clamp(0.0, 1.0);
            // Cubic curve: stay thin at top, wide at bottom
            let flare_amount = progress.powi(3) * 350.0;
            let noise = rng.random_range(-15.0..15.0);

            points.push((current_y, flare_amount + noise));
            current_y += 50.0; // Vertical density
        }
        points
    });

    // Mirroring: Left side uses -scale-x, Right side is normal
    // Position: Left side pins to left: 0, Right side pins to right: 0
    let container_style = if is_left {
        "left-0 "
    } else {
        "right-0 -scale-x-100"
    };

    rsx! {
        div {
            class: "absolute top-0 {container_style} h-full w-[600px] pointer-events-none",

            for (y, flare) in cliff_data().iter() {
                {
                    // To avoid "missing chunks", we render multiple rocks horizontally
                    // starting from the "flare" edge and going back toward the screen border.
                    (0..4).map(|i| {
                        // Offset each rock by 64px (half-width) to create a solid wall
                        let x_pos = flare - (i as f32 * 80.0);
                        let brightness = 1.0 - (i as f32 * 0.2);

                        rsx! {
                            img {
                                key: "{y}-{i}",
                                class: "absolute pixel-perfect max-w-none -scale-x-100",
                                style: "
                                    top: {y}px; 
                                    left: {x_pos}px; 
                                    width: 128px; 
                                    height: 128px; 
                                    filter: brightness({brightness});
                                    z-index: {i};
                                ",
                                src: "{cliff_asset}"
                            }
                        }
                    })
                }
            }
        }
    }
}
