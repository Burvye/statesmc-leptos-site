use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos(u32, u32);
#[component]
pub fn GamePage() -> impl IntoView {
    let (pos, set_pos) = signal(Pos(0, 0));
    view! {
        <div
            class="player"
            style:position="absolute"
            style:top=move || format!("{}px", pos.get().0)
            style:left=move || format!("{}px", pos.get().1)
        >
            67
        </div>
    }
}
