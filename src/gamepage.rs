use leptos::prelude::*;
use leptos::ev;
use gloo_timers::future::TimeoutFuture;
use leptos::svg::g;

const GROUND: i16 = 500;
const WALKSPEED: i16 = 10;

#[derive(Clone, Copy)]
struct Pos {
    x: i16,
    y: i16,
}
#[derive(Clone, Copy)]
struct Vel {
    x: i16,
    y: i16,
}
#[component]
pub fn GamePage() -> impl IntoView {
    let (pos, set_pos) = signal(Pos { x: 0, y: 0 });
    let (vel, set_vel) = signal(Vel {x: 0, y: 0 });
    let (grounded, set_grounded) = signal(false);

    window_event_listener(ev::keydown, move |ev| {
        if ev.code() == "Space" && !ev.repeat() {
            ev.prevent_default();
            set_vel.update(|vel| {
                vel.y = -20 * (grounded.get() as i16);
            });
        }
    });
    window_event_listener(ev::keydown, move |ev| {
        set_vel.update(|vel| {
            vel.x += ((ev.code() == "KeyD") as i16 - (ev.code() == "KeyA") as i16) * WALKSPEED;
            vel.x = vel.x.clamp(-WALKSPEED, WALKSPEED);
        });
    });
    window_event_listener(ev::keyup, move |ev| {
        if (ev.code() == "KeyA" || ev.code() == "KeyD") && grounded.get(){
            set_vel.update(|vel| {
                vel.x = 0;
            });
        }
    });
    leptos::task::spawn_local(async move {
        loop {
            TimeoutFuture::new(16).await;

            let new_x = pos.get().x + vel.get().x;
            let mut new_y = pos.get().y;
            if new_y >= GROUND {
                set_grounded.set(true);
            } else {
                set_grounded.set(false);
            }
            // GRAVITY STUFFS
            let mut gforce = vel.get().y + 1;
            new_y += gforce;
            if grounded.get() {
                new_y = GROUND;
                gforce = 0;
            }
            set_vel.update(|vel| {
                vel.y = gforce;
            });
            // END GRAVITY STUFFS

            set_pos.set(Pos { x: new_x, y: new_y,});
        }
    });

    view! {
        <div
            class="player"
            style:position="absolute"
            style:top=move || format!("{}px", pos.get().y)
            style:left=move || format!("{}px", pos.get().x)
        >
            67
        </div>
    }
}
