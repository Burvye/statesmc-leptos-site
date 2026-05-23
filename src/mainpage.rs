use crate::elements;
use leptos::prelude::*;
use leptos_router::components::*;
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Header />
        <Body />
        <MapSection />
        <LinkSection />
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <div class="header parent">
            <div class="child header-child title title1 simage">"StatesMC"</div>
            <div class="child header-child title title2" id="ip-parent">
                "Best Gun Server in the World!!!"
                <div class="infant">"IP: statesmc.us"</div>
            </div>
        </div>
    }
}

#[component]
fn Body() -> impl IntoView {
    view! {
        <div class="body parent">
            <Description />
            <elements::videoselector::VideoSection />
        </div>
    }
}

#[component]
fn Description() -> impl IntoView {
    view! {
        <div class="child body-child body-side">
            "Why don't civilization servers ever feature firearms? StatesMC makes this vision a reality. Unlike other (unmodded) SMPs, you can craft guns, technology, and nations capable of dominating continents. With the tools we give you, anything is possible. Lead a ruthless dictatorship set on conquering lands, or a thriving democracy serving the interests of its people. The choice is yours, and choose wisely. When chaos inevitably erupts, brace yourself for intense full scale gunfights that test the true capabilities of your nation. In the end, only the strongest factions may survive. The question is:
            Will it be yours?"
        </div>
    }
}
#[component]
fn MapSection() -> impl IntoView {
    view! {
        <div class="parent map-parent">
            <iframe
                style="width: 100%; height: 100%; overflow: hidden;"
                src="https://map.statesmc.us"
                width="400px"
                height="400px"
                scrolling="no"
                class="map-child"
            >
                "Failed to load the server map"
            </iframe>
            <LinkSection />
        </div>
    }
}

#[component]
fn LinkSection() -> impl IntoView {
    let links = link_choices();
    view! {
        <div>
            <div>
                {links
                    .into_iter()
                    .map(|link| {
                        view! {
                            <button class="text butt title title1 space">
                                <A href=link.href>{link.label}</A>
                            </button>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
struct Link {
    label: &'static str,
    href: &'static str,
}
fn link_choices() -> Vec<Link> {
    let mut links = vec![];
    links.push(Link {
        label: "About",
        href: "/about",
    });
    links.push(Link {
        label: "Adventure",
        href: "/adventure",
    });

    links
}
