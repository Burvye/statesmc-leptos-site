use leptos::prelude::*;
use leptos::ev;
use gloo_timers::future::TimeoutFuture;

const GROUND: i16 = 500;
const WALKSPEED: i16 = 10;
const PBOUND: [i16; 2] = [50, 50];

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

/// Box for the world
/// just put in your corners c1 and c2
#[derive(Clone, Copy)]
struct Box {
    c1: Pos,
    c2: Pos,
}
#[component]
pub fn GamePage() -> impl IntoView {
    let (pos, set_pos) = signal(Pos { x: 0, y: 0 });
    let (vel, set_vel) = signal(Vel {x: 0, y: 0 });
    // prevents the player from falling and allows them to jump
    let (grounded, set_grounded) = signal(false);
    // this controls the player button thing
    let (toggle, set_toggle) = signal(false);
    // to detect if left or right pressed
    let (lp, set_lp) = signal(false);
    let (rp, set_rp) = signal(false);

    // vec to store boxes to load into the world
    // collis is meant to mean colliders (aabb colliders!!!!)
    let (collis, set_collis) = signal(Vec::new());
    set_collis.update(|vec| {vec.push(Box {
        c1: Pos { x: 0, y: 500 },
        c2: Pos { x: 500, y: 550 },
    })});

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
                new_y -= gforce;
                gforce = 0;
            }
            set_vel.update(|vel| {
                vel.y = gforce;
            });
            // END GRAVITY STUFFS
            // XINPUT STUFFS
            window_event_listener(ev::keydown, move |ev| {
                match ev.code().as_str() {
                    "KeyA" => set_lp.set(true),
                    "KeyD" => set_rp.set(true),
                    _ => {}
                }
            });
            window_event_listener(ev::keyup, move |ev| {
                match ev.code().as_str() {
                    "KeyA" => set_lp.set(false),
                    "KeyD" => set_rp.set(false),
                    _ => {}
                }
            });
            let xinput = (rp.get() as i16) - (lp.get() as i16);
            set_vel.update(|vel| {
                if xinput != 0 {
                    vel.x = xinput * WALKSPEED;
                } else if grounded.get() {
                    vel.x = 0;
                }
            });
            // END XINPUT STUFFS
            set_pos.set(Pos { x: new_x, y: new_y,});
        }
    });

    view! {
        <div>
            <For
                each=move || collis.get()
                key=|colli| (colli.c1.x, colli.c1.y, colli.c2.x, colli.c2.y)
                let(colli)
            >
                <div
                    class="grass"
                    style:position="absolute"
                    style:height=move || format!("{}px", colli.c2.y - colli.c1.y)
                    style:width=move || format!("{}px", colli.c2.x - colli.c1.x)
                    style:top=move || format!("{}px", colli.c1.y)
                    style:left=move || format!("{}px", colli.c1.x)
                >
                    67
                </div>
            </For>
        </div>
        <button
            class="player"
            style:position="absolute"
            class:player-green=move || toggle.get()
            class:player-red=move || !toggle.get()
            style:top=move || format!("{}px", pos.get().y)
            style:left=move || format!("{}px", pos.get().x)
            on:click=move |_| set_toggle.set(!toggle.get())
        >
            67
        </button>
    }
}
struct EPoints {
    p1: Pos,
    p2: Pos,
}
struct Edge {
    ps: EPoints,
    m: f32,
    b: f32
}
fn normal_force(pos: Pos, collis: &[Box]) -> Vel {
    collis
        .iter()
        .map(
            |colli|
            [
                colli.c1,
                Pos {x: colli.c2.x, y: colli.c1.y },
                colli.c2,
                Pos {x: colli.c1.x, y: colli.c2.y }
            ]
        )
        .map(
            |cs| {
                (0..=3)
                    .into_iter()
                    .map(
                        |c|
                        EPoints {
                            p1: cs[c],
                            p2: cs[(c + 1) % 4]
                        })
                    .collect::<[EPoints; 4]>()
            }
        )
        .map(
            |es| {
                es
                .into_iter()
                .map(
                    |e|
                    {
                        let slope: f32 = (e.p1.y-e.p2.y) as f32/(e.p1.x-e.p2.x) as f32;
                        Edge {
                            ps: e,
                            m: slope,
                            b: ((e.p1.y as f32)-slope*(e.p1.x as f32)) // intercept
                        }
                    }
                )
                .collect::<[Edge; 4]>()
            }
        )
    Vel {
        x: 67,
        y: 67,
    }
}

fn is_grounded(y: i16) -> bool {
    // TODO: more usable grounded
    if (y + PBOUND[1]) >= GROUND {
        true
    } else {
        false
    }
}
