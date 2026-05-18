use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

mod about;
mod elements;
mod mainpage;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <p>"Page not found."</p> }>
                <Route path=path!("/") view=mainpage::HomePage />
                <Route path=path!("/about") view=about::AboutPage />
            </Routes>
        </Router>
    }
}
