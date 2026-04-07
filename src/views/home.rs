use crate::components::Frame;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Frame {
            h1 {
                class: "text-4xl sm:text-5xl lg:text-6xl font-black text-[#ff8c32] tracking-tighter drop-shadow-[0_0_15px_rgba(255,140,50,0.5)]",
                "TETRAFORM"
            }
        }
    }
}
