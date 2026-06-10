use leptos::prelude::*;

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
        ></div>
        <button on:click=move |_| { set_pos.set(Pos(6, 7)) }></button>
    }
}
