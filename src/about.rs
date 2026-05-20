use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::mainpage;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Routes fallback=|| view! { <p>"Page not found."</p> }>
            <Route path=path!("/about") view=About />
            <Route path=path!("/") view=mainpage::HomePage />
        </Routes>
    }
}

#[component]
fn About() -> impl IntoView {
    let (image, set_image) = signal(image_selector(1));
    let butts = button_generator();
    view! {
        <div class="header parent">
            <div class="title title1 simage header-child">"About StatesMC"</div>
            <div class="header-child">

                <button class="title title1 butt space">
                    <A href="/">"Home"</A>
                </button>
            </div>
        </div>
        <div class="body-parent">
            <p class="text">
                "
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                "
            </p>
            <div class="limage" style:background-image=format!("url({})", image.get().image)>
                <div class="scroll-container">
                    // TODO: We are here
                    {butts
                        .into_iter()
                        .map(|b| {
                            view! {
                                <li>
                                    <button class="butt-list">{b.label}</button>
                                </li>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </div>
        <div>
            <A href="https://github.com/Burvye/statesmc-leptos-site/tree/main">
                <button class="github-link"></button>
            </A>
        </div>
    }
}

#[derive(Clone)]
struct Button {
    id: i32,
    label: String,
}

#[derive(Clone)]
struct Image {
    image: &'static str,
    message: &'static str,
}

fn button_generator() -> Vec<Button> {
    let mut buttons = vec![];
    for i in 1..=20 {
        buttons.push(Button {
            id: i,
            label: format!("Image {}", i),
        })
    }
    buttons
}

fn image_selector(id: i32) -> Image {
    match id {
        1 => Image {
            image: "assets/backs/1.png",
            message: "EstGlory dominating the hardpoint",
        },
        _ => Image {
            image: "problem",
            message: "please kill yourself",
        },
    }
}
