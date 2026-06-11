use leptos::prelude::*;
use leptos::ev;
use gloo_timers::future::TimeoutFuture;

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
    // this controls the player button thing
    let (toggle, set_toggle) = signal(false);
    // TODO: use this to detect whether the player is moving or not
    let mut (moving, set_moving) = signal(false);

    window_event_listener(ev::keydown, move |ev| {
        if ev.code() == "Space" && !ev.repeat() && grounded.get() {
            ev.prevent_default();
            set_vel.update(|vel| {
                vel.y = -20;
            });
        }
    });
    leptos::task::spawn_local(async move {
        loop {
            TimeoutFuture::new(16).await;
            let mut gforce = vel.get().y + 1;
            let new_x = pos.get().x + vel.get().x;
            let mut new_y = pos.get().y + gforce;
            set_grounded.set(is_grounded(new_y));
            // GRAVITY STUFFS
            if grounded.get() {
                new_y = GROUND;
                gforce = 0;
            }
            set_vel.update(|vel| {
                vel.y = gforce;
            });
            // END GRAVITY STUFFS

            window_event_listener(ev::keydown, move |ev| {
                set_vel.update(|vel| {
                    if ev.code() == "KeyD" || ev.code() == "KeyA" {
                        if ev.code() == "KeyD" {
                            vel.x += WALKSPEED;
                        } else {
                            vel.x -= WALKSPEED;
                        }
                        vel.x = vel.x.clamp(-WALKSPEED, WALKSPEED);
                    }
                });
            });
            set_pos.set(Pos { x: new_x, y: new_y,});
        }
    });
    fn is_grounded(y: i16) -> bool {
        // more usable grounded
        if y >= GROUND {
            true
        } else {
            false
        }
    }

    view! {
        <div
            class="player"
            style:position="absolute"
            class:player-green=move || toggle.get()
            class:player-red=move || !toggle.get()
            style:top=move || format!("{}px", pos.get().y)
            style:left=move || format!("{}px", pos.get().x)
        >
            <button on:click=move |_| set_toggle.set(!toggle.get())>67</button>
        </div>
    }
}
