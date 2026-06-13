use leptos::prelude::*;
use leptos::ev;
use gloo_timers::future::TimeoutFuture;

const GROUND: i16 = 500;
const WALKSPEED: i16 = 10;
const PBOUND: [i16; 2] = [50, 50];

#[derive(Clone, Copy)]
struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    fn sub(&self, op: &Pos) -> Pos {
        Pos {
            x: op.x - self.x,
            y: op.y - self.y,
        }
    }
    fn from_vector(vec: Vector) -> Pos {
        Pos {
            x: vec.x,
            y: vec.y,
        }
    }
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
    let (pos, set_pos) = signal(Pos { x: 0.0, y: 0.0 });
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
        c1: Pos { x: 0.0, y: 500.0 },
        c2: Pos { x: 500.0, y: 550.0 },
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
            let new_x = pos.get().x.round() as i16 + vel.get().x;
            let mut new_y = pos.get().y.round() as i16 + gforce;
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
            set_pos.set(Pos { x: new_x as f32, y: new_y as f32,});
        }
    });

    view! {
        <div>
            <For
                each=move || collis.get()
                key=|colli| (
                    colli.c1.x.round() as i16,
                    colli.c1.y.round() as i16,
                    colli.c2.x.round() as i16,
                    colli.c2.y.round() as i16,
                )
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
            // TODO: check if the player pos actually represents
            // the center or else we cooked
            style:top=move || format!("{}px", pos.get().y)
            style:left=move || format!("{}px", pos.get().x)
            on:click=move |_| set_toggle.set(!toggle.get())
        >
            67
        </button>
    }
}
/// Stores the endpoints of an edge
struct EPoints {
    p1: Pos,
    p2: Pos,
}
struct Vector {
    x: f32,
    y: f32,
}
impl Vector {
    fn dot(&self, ov: &Vector) -> f32 {
        self.x * ov.x + self.y * ov.y
    }
    fn scale(&self, scalar: f32) -> Vector {
        Vector { x: self.x * scalar, y: self.y * scalar }
    }
    fn add(&self, ov: &Vector) -> Vector {
        Vector { x: self.x + ov.x, y: self.y + ov.y }
    }
    /// u should probably only use this on normalized pos
    fn from_pos(pos: &Pos) -> Vector {
        Vector { x: pos.x as f32, y: pos.y as f32 }
    }
}

fn normal_vel(pos: Pos, collis: &[Box]) -> Vel {
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
        .filter_map(
            |eps| {
                eps.iter().map(
                    |ep| { // ep is &EPoints
                        let ppos = projected_point(&pos, ep);
                        // AABB check
                        if (
                            67==67
                            // check if ppos is within the bounds of the player
                        ) {
                            Some(ep)
                        } else {
                            None
                        }

                    }
                )
            }
        ) // then we get all the vectors and add them up
    Vel {
        x: 67,
        y: 67,
    }
}

/// Finds the ppos point if projected on the edge of eps
/// This also just so happens to be the nearest point
fn projected_point(pos: &Pos, eps: &EPoints) -> Pos {
    let (edgevec, playvec) = (
        Vector::from_pos(&eps.p2.sub(&eps.p1)),
        Vector::from_pos(&pos.sub(&eps.p1))
    );
    Pos::from_vector(Vector::from_pos(&eps.p1).add(&edgevec.scale(playvec.dot(&edgevec) / edgevec.dot(&edgevec))))
}

fn is_grounded(y: i16) -> bool {
    // TODO: more usable grounded
    if (y + PBOUND[1]) >= GROUND {
        true
    } else {
        false
    }
}
