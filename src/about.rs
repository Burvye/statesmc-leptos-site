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
            <div>
                <p class="text">
                    "
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                    "
                </p>
            </div>
        </div>
    }
}
