use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;

#[derive(Clone, Copy, PartialEq)]
struct Pos(u16, u16);

#[derive(Clone, Copy, PartialEq)]
struct Vel(i16,i16);

#[component]
pub fn GamePage() -> impl IntoView {
    let (pos, set_pos) = signal(Pos(0, 0));
    let (vel, set_vel) = signal(Vel(0,0));

    leptos::task::spawn_local(async move {
        loop {
            TimeoutFuture::new(16).await;
            set_vel.update(|v| v.1 += 1);
            set_pos.update(|p| p.0 = (p.0 as i16 + vel.get().1).max(0) as u16);
        }
    });

    view! {
        <div
            class="player"
            style:position="absolute"
            style:top=move || format!("{}px", pos.get().0)
            style:left=move || format!("{}px", pos.get().1)
        ></div>
    }
}
