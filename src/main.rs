use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

fn main() {
    mount_to_body(App);
}

#[derive(Clone)]
struct Video {
    label: String,
    id: i32,
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <p>"Page not found."</p> }>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/about") view=AboutPage />
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Header />
        <Body />
        <MapSection />
        <LinkSection />
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! { <div>"About"</div> }
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
            <VideoSection />
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
fn VideoSection() -> impl IntoView {
    let videos = video_choices();
    let (selected_id, set_selected_id) = signal(videos[0].id);
    let (url, set_url) = signal(video_url_for(videos[0].id));

    view! {
        <div class="child body-child body-main">
            <VideoPlayer url=url />
            <div class="body-two">
                <div id="main-button-info">"Click one of these buttons below!"</div>
                <VideoButtons
                    videos=videos
                    selected_id=selected_id
                    set_selected_id=set_selected_id
                    set_url=set_url
                />
            </div>
        </div>
    }
}

#[component]
fn VideoPlayer(url: ReadSignal<&'static str>) -> impl IntoView {
    view! {
        <iframe
            width="560"
            height="315"
            src=move || url.get()
            title="YouTube video player"
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
            referrerpolicy="strict-origin-when-cross-origin"
            allowfullscreen
            class="body-two"
        ></iframe>
    }
}

#[component]
fn VideoButtons(
    videos: Vec<Video>,
    selected_id: ReadSignal<i32>,
    set_selected_id: WriteSignal<i32>,
    set_url: WriteSignal<&'static str>,
) -> impl IntoView {
    view! {
        <div class="body-child">
            {videos
                .into_iter()
                .map(|video| {
                    view! {
                        <button
                            on:click=move |_| {
                                set_selected_id.set(video.id);
                                set_url.set(video_url_for(video.id));
                            }
                            class="main-button"
                        >
                            {video.label}
                        </button>
                    }
                })
                .collect::<Vec<_>>()}
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
                            <p class="text">
                                <A href=link.href>{link.label}</A>
                            </p>
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

    links
}

fn video_choices() -> Vec<Video> {
    let mut output = vec![];
    for i in 1..=12 {
        output.push({
            Video {
                id: i,
                label: format!("Video {}", i),
            }
        });
    }
    output
}

/// Match an ID to a video URL.
fn video_url_for(id: i32) -> &'static str {
    match id {
        1 => "https://www.youtube.com/embed/3ZTdoWPDiOc?si=33aJbEe2FcbV5Fga",
        2 => "https://www.youtube.com/embed/CdYtavp2h24?si=oBFzoC4V169_UVxp",
        3 => "https://www.youtube.com/embed/Lcupxa72cLk?si=s2IgYDHDwJKvt4rB",
        4 => "https://www.youtube.com/embed/cqd00g20bkI?si=znIN52dyM1A07JH9",
        5 => "https://www.youtube.com/embed/HlnNSW_Yi70?si=krmzKolIwR7EZOAq",
        6 => "https://www.youtube.com/embed/wcRfV6ahkh0?si=LeE3M3i8_Va-XDAl",
        7 => "https://www.youtube.com/embed/3ZTdoWPDiOc?si=33aJbEe2FcbV5Fga",
        8 => "https://www.youtube.com/embed/CdYtavp2h24?si=oBFzoC4V169_UVxp",
        9 => "https://www.youtube.com/embed/Lcupxa72cLk?si=s2IgYDHDwJKvt4rB",
        10 => "https://www.youtube.com/embed/cqd00g20bkI?si=znIN52dyM1A07JH9",
        11 => "https://www.youtube.com/embed/HlnNSW_Yi70?si=krmzKolIwR7EZOAq",
        12 => "https://www.youtube.com/embed/wcRfV6ahkh0?si=LeE3M3i8_Va-XDAl",
        _ => "https://www.youtube.com/embed/3ZTdoWPDiOc?si=33aJbEe2FcbV5Fga",
    }
}
