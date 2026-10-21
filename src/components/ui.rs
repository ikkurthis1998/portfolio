//! Shared v2 components: icons, product marks, links, chips, section headers and the
//! data states. Styling lives in public/assets/v2.css; tokens there follow the Figma
//! variables, so these components only pick classes.
use leptos::prelude::*;
use leptos_router::hooks::use_location;

pub const EMAIL: &str = "ikkurthis1998@gmail.com";
pub const THELIVI_URL: &str = "https://thelivi.isree.dev";
pub const THELIVI_IOS_URL: &str = "https://apps.apple.com/app/id6790257232";

/// Lucide icons used by the design (24 grid, 1.75 stroke at every rendered size).
#[derive(Clone, Copy, PartialEq)]
pub enum Glyph {
    ArrowUpRight,
    ArrowUp,
    ArrowLeft,
    Menu,
    Close,
    Mail,
    Globe,
    Loader,
}

impl Glyph {
    fn paths(self) -> &'static [&'static str] {
        match self {
            Glyph::ArrowUpRight => &["M7 7h10v10", "M7 17 17 7"],
            Glyph::ArrowUp => &["m5 12 7-7 7 7", "M12 19V5"],
            Glyph::ArrowLeft => &["m12 19-7-7 7-7", "M19 12H5"],
            Glyph::Menu => &["M4 5h16", "M4 12h16", "M4 19h16"],
            Glyph::Close => &["M18 6 6 18", "m6 6 12 12"],
            Glyph::Mail => &[
                "M4 4h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2Z",
                "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7",
            ],
            Glyph::Globe => &[
                "M22 12a10 10 0 1 1-20 0 10 10 0 0 1 20 0Z",
                "M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20",
                "M2 12h20",
            ],
            Glyph::Loader => &["M21 12a9 9 0 1 1-6.219-8.56"],
        }
    }
}

#[component]
pub fn Icon(glyph: Glyph, #[prop(default = 16)] size: u32, #[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <svg class=format!("icon {class}") width=size height=size viewBox="0 0 24 24" aria-hidden="true" focusable="false">
            {glyph.paths().iter().map(|d| view! { <path d=*d/> }).collect_view()}
        </svg>
    }
}

/// Product marks: a Latin initial on an accent tile (radius/md), exported from the
/// Figma Product mark components.
#[derive(Clone, Copy, PartialEq)]
pub enum Brand {
    Isree,
    Thelivi,
    Vihari,
    Suddhi,
    Gamani,
}

impl Brand {
    fn class(self) -> &'static str {
        match self {
            Brand::Isree => "mark-isree",
            Brand::Thelivi => "mark-thelivi",
            Brand::Vihari => "mark-vihari",
            Brand::Suddhi => "mark-suddhi",
            Brand::Gamani => "mark-gamani",
        }
    }
    fn glyph(self) -> (&'static str, &'static str) {
        match self {
            Brand::Isree => ("translate(14.13 8.86)", "M1.86096 14.28C1.32926 14.28 0.917819 14.1218 0.626649 13.8053C0.335479 13.4888 0.189894 13.033 0.189894 12.438V6.43739C0.189894 5.84239 0.335479 5.38665 0.626649 5.07016C0.917819 4.75367 1.32926 4.59543 1.86096 4.59543C2.40532 4.59543 2.82309 4.75367 3.11426 5.07016C3.41809 5.38665 3.57 5.84239 3.57 6.43739V12.438C3.57 13.033 3.42441 13.4888 3.13324 13.8053C2.84207 14.1218 2.41798 14.28 1.86096 14.28ZM1.86096 3.24718C1.26596 3.24718 0.803883 3.10793 0.474734 2.82941C0.158245 2.5509 0 2.1458 0 1.6141C0 1.09505 0.158245 0.696276 0.474734 0.417766C0.803883 0.139255 1.26596 0 1.86096 0C2.48128 0 2.94968 0.139255 3.26617 0.417766C3.58266 0.696276 3.7409 1.09505 3.7409 1.6141C3.7409 2.1458 3.58266 2.5509 3.26617 2.82941C2.94968 3.10793 2.48128 3.24718 1.86096 3.24718Z"),
            Brand::Thelivi => ("translate(9.5 8.86)", "M6.5 14.28C5.91333 14.28 5.46 14.12 5.14 13.8C4.82 13.4667 4.66 13.0067 4.66 12.42V3H1.52C1.02667 3 0.646667 2.87333 0.38 2.62C0.126667 2.35333 0 1.98 0 1.5C0 1.00667 0.126667 0.633333 0.38 0.38C0.646667 0.126667 1.02667 0 1.52 0H11.48C11.9733 0 12.3467 0.126667 12.6 0.38C12.8667 0.633333 13 1.00667 13 1.5C13 1.98 12.8667 2.35333 12.6 2.62C12.3467 2.87333 11.9733 3 11.48 3H8.34V12.42C8.34 13.0067 8.18667 13.4667 7.88 13.8C7.57333 14.12 7.11333 14.28 6.5 14.28Z"),
            Brand::Vihari => ("translate(8.92 8.77)", "M7.09273 14.46C6.58606 14.46 6.17273 14.3533 5.85273 14.14C5.54606 13.9267 5.28606 13.5867 5.07273 13.12L0.232727 2.44C0.0193939 1.97333 -0.0472727 1.55333 0.0327273 1.18C0.112727 0.806667 0.299394 0.52 0.592727 0.320001C0.886061 0.106667 1.25273 0 1.69273 0C2.23939 0 2.64606 0.12 2.91273 0.36C3.19273 0.6 3.43273 0.953334 3.63273 1.42L7.71273 10.86H6.59273L10.6727 1.4C10.8727 0.933334 11.1127 0.586667 11.3927 0.36C11.6727 0.12 12.0661 0 12.5727 0C12.9861 0 13.3327 0.106667 13.6127 0.320001C13.8927 0.52 14.0661 0.806667 14.1327 1.18C14.2127 1.55333 14.1461 1.97333 13.9327 2.44L9.07273 13.12C8.87273 13.5867 8.61939 13.9267 8.31273 14.14C8.00606 14.3533 7.59939 14.46 7.09273 14.46Z"),
            Brand::Suddhi => ("translate(9.6 8)", "M6.21814 15.994C5.63148 15.994 5.01548 15.95 4.37014 15.862C3.73948 15.7887 3.13814 15.6713 2.56614 15.51C1.99414 15.3487 1.48081 15.158 1.02614 14.938C0.630143 14.7473 0.351476 14.498 0.190143 14.19C0.0288095 13.8673 -0.0298572 13.53 0.0141429 13.178C0.0581429 12.826 0.18281 12.518 0.388143 12.254C0.593476 11.9753 0.86481 11.792 1.20214 11.704C1.53948 11.6013 1.92081 11.6453 2.34614 11.836C2.90348 12.1 3.53414 12.3053 4.23814 12.452C4.94214 12.5987 5.60214 12.672 6.21814 12.672C7.18614 12.672 7.86081 12.5473 8.24214 12.298C8.63814 12.034 8.83614 11.7113 8.83614 11.33C8.83614 10.9927 8.69681 10.7213 8.41814 10.516C8.15414 10.3107 7.67748 10.1347 6.98814 9.988L4.41414 9.438C3.00614 9.14467 1.95748 8.63133 1.26814 7.898C0.578809 7.16467 0.234143 6.21867 0.234143 5.06C0.234143 4.29733 0.388143 3.608 0.696143 2.992C1.01881 2.36133 1.46614 1.826 2.03814 1.386C2.62481 0.946 3.31414 0.608666 4.10614 0.373999C4.91281 0.124666 5.80748 0 6.79014 0C7.55281 0 8.32281 0.0879997 9.10014 0.263999C9.89214 0.425333 10.5741 0.674667 11.1461 1.012C11.4835 1.188 11.7255 1.43 11.8721 1.738C12.0188 2.046 12.0775 2.36867 12.0481 2.706C12.0188 3.02867 11.9015 3.31467 11.6961 3.564C11.5055 3.81333 11.2415 3.982 10.9041 4.07C10.5815 4.14333 10.1928 4.08467 9.73814 3.894C9.31281 3.70333 8.82881 3.564 8.28614 3.476C7.75814 3.37333 7.24481 3.322 6.74614 3.322C6.20348 3.322 5.74148 3.388 5.36014 3.52C4.97881 3.63733 4.68548 3.81333 4.48014 4.048C4.28948 4.28267 4.19414 4.554 4.19414 4.862C4.19414 5.17 4.31881 5.434 4.56814 5.654C4.83214 5.85933 5.30881 6.03533 5.99814 6.182L8.55014 6.732C9.97281 7.04 11.0361 7.546 11.7401 8.25C12.4441 8.954 12.7961 9.87067 12.7961 11C12.7961 11.7627 12.6421 12.452 12.3341 13.068C12.0261 13.684 11.5861 14.212 11.0141 14.652C10.4421 15.0773 9.75281 15.4073 8.94614 15.642C8.13948 15.8767 7.23014 15.994 6.21814 15.994Z"),
            Brand::Gamani => ("translate(9.58 8.73)", "M7.56 14.54C5.94667 14.54 4.57333 14.24 3.44 13.64C2.32 13.04 1.46667 12.2 0.88 11.12C0.293333 10.04 0 8.78 0 7.34C0 6.22 0.173333 5.21333 0.52 4.32C0.88 3.41333 1.39333 2.64 2.06 2C2.72667 1.34667 3.54 0.853333 4.5 0.519999C5.47333 0.173333 6.57333 0 7.8 0C8.46667 0 9.14667 0.0666666 9.84 0.2C10.5333 0.32 11.2133 0.553333 11.88 0.9C12.2267 1.06 12.46 1.28667 12.58 1.58C12.7133 1.86 12.7533 2.16 12.7 2.48C12.66 2.78667 12.5467 3.07333 12.36 3.34C12.1867 3.59333 11.9467 3.76667 11.64 3.86C11.3333 3.94 10.9867 3.89333 10.6 3.72C10.2 3.53333 9.76667 3.39333 9.3 3.3C8.83333 3.19333 8.34 3.14 7.82 3.14C6.92667 3.14 6.18 3.3 5.58 3.62C4.99333 3.94 4.55333 4.41333 4.26 5.04C3.96667 5.66667 3.82 6.43333 3.82 7.34C3.82 8.7 4.15333 9.72667 4.82 10.42C5.48667 11.1133 6.47333 11.46 7.78 11.46C8.18 11.46 8.61333 11.42 9.08 11.34C9.56 11.26 10.04 11.1467 10.52 11L9.84 12.38V8.96H8.44C7.98667 8.96 7.63333 8.84667 7.38 8.62C7.14 8.39333 7.02 8.07333 7.02 7.66C7.02 7.24667 7.14 6.93333 7.38 6.72C7.63333 6.49333 7.98667 6.38 8.44 6.38H11.4C11.8667 6.38 12.22 6.50667 12.46 6.76C12.7133 7 12.84 7.35333 12.84 7.82V12.24C12.84 12.64 12.7533 12.98 12.58 13.26C12.42 13.54 12.16 13.74 11.8 13.86C11.1733 14.0733 10.4933 14.24 9.76 14.36C9.02667 14.48 8.29333 14.54 7.56 14.54Z"),
        }
    }
}

#[component]
pub fn ProductMark(brand: Brand) -> impl IntoView {
    let (transform, d) = brand.glyph();
    view! {
        <svg class=format!("mark {}", brand.class()) width="32" height="32" viewBox="0 0 32 32" aria-hidden="true" focusable="false">
            <rect class="mark-tile" width="32" height="32" rx="12"/>
            <path class="mark-glyph" transform=transform d=d/>
        </svg>
    }
}

/// Product lockup/isree.dev: mark + name, linking home.
#[component]
pub fn Lockup() -> impl IntoView {
    view! { <a class="lockup" href="/" aria-label="isree.dev home"><ProductMark brand=Brand::Isree/><span>"isree.dev"</span></a> }
}

/// Text link: Headline label with a trailing 16 icon. External links open in a new tab.
#[component]
pub fn TextLink(
    href: String,
    label: &'static str,
    #[prop(default = Glyph::ArrowUpRight)] glyph: Glyph,
    #[prop(default = false)] external: bool,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] aria_label: Option<&'static str>,
) -> impl IntoView {
    view! {
        <a class=format!("text-link {class}") href=href target=external.then_some("_blank") rel=external.then_some("noopener noreferrer") aria-label=aria_label>
            <span class="label">{label}</span><Icon glyph=glyph/>
        </a>
    }
}

#[component]
pub fn BackLink(href: &'static str, label: String) -> impl IntoView {
    view! { <a class="back-link" href=href><Icon glyph=Glyph::ArrowLeft/><span>{label}</span></a> }
}

#[component]
pub fn Chips(items: Vec<String>) -> impl IntoView {
    view! { <ul class="chips">{items.into_iter().map(|t| view! { <li class="badge">{t}</li> }).collect_view()}</ul> }
}

/// Section header/Web: a Label, optionally with a "View resume"-style action on the right.
#[component]
pub fn SectionHead(label: &'static str, heading: &'static str, #[prop(optional)] heading_id: &'static str, #[prop(optional)] action: Option<AnyView>) -> impl IntoView {
    view! {
        <div class="section-head">
            <div class="section-bar"><p class="eyebrow">{label}</p>{action}</div>
            <h2 class="t-display" id=(!heading_id.is_empty()).then_some(heading_id)>{heading}</h2>
        </div>
    }
}

pub struct StepCopy {
    pub title: &'static str,
    pub body: &'static str,
}

/// Card/Step grid: numbered steps on surface cards.
#[component]
pub fn Steps(steps: &'static [StepCopy]) -> impl IntoView {
    view! {
        <ol class="grid-3 steps">{steps.iter().enumerate().map(|(i, s)| view! {
            <li class="card step-card"><p class="num">{format!("{:02}", i + 1)}</p>
                <div class="step-text"><h3 class="t-headline">{s.title}</h3><p class="t-prose">{s.body}</p></div>
            </li>
        }).collect_view()}</ol>
    }
}

pub const HOW_I_WORK: &[StepCopy] = &[
    StepCopy { title: "Understand the problem", body: "Who needs it, what it has to do, and the limits it has to live within, written down before any code." },
    StepCopy { title: "Build the smallest thing that works", body: "An architecture that fits the problem, and one real workflow running end to end, tested." },
    StepCopy { title: "Prove it, then improve it", body: "Tests, monitoring and real usage decide what changes next. Complexity that doesn’t earn its place goes." },
];

#[component]
pub fn HowIWork() -> impl IntoView {
    view! {
        <section class="section how-i-work" aria-labelledby="how-i-work-title">
            <SectionHead label="How I work" heading="Understand it. Build it. Prove it." heading_id="how-i-work-title"/>
            <Steps steps=HOW_I_WORK/>
        </section>
    }
}

/// Shared contact band at the end of most pages.
#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section id="contact" class="contact" aria-labelledby="contact-title">
            <div class="section-text">
                <SectionHead label="Let’s connect" heading="Have a problem to solve?" heading_id="contact-title"/>
                <p class="t-prose">"Let’s discuss the requirements and what it would take to build."</p>
            </div>
            <div class="actions">
                <a class="btn btn-primary" href=format!("mailto:{EMAIL}")>"Get in touch"</a>
                <a class="btn btn-secondary" href="/hire">"Hire me"</a>
            </div>
        </section>
    }
}

/// Image with a dark-mode variant; the browser picks by the system setting.
#[component]
pub fn ThemedImage(light: &'static str, dark: &'static str, alt: String, #[prop(optional)] width: u32, #[prop(optional)] height: u32, #[prop(default = true)] lazy: bool) -> impl IntoView {
    view! {
        <picture>
            <source srcset=dark media="(prefers-color-scheme: dark)"/>
            <img src=light alt=alt width=(width > 0).then_some(width) height=(height > 0).then_some(height) loading=if lazy { "lazy" } else { "eager" } decoding="async"/>
        </picture>
    }
}

#[component]
pub fn Loading() -> impl IntoView {
    view! { <div class="loading-state" role="status"><p class="eyebrow">"Loading portfolio"</p><div class="loading-track"><span></span></div></div> }
}

#[component]
pub fn DataError() -> impl IntoView {
    // A full reload of the current route refetches the server data.
    let path = use_location().pathname;
    view! {
        <div class="card data-error" role="alert">
            <h2 class="t-headline">"The content couldn’t load."</h2>
            <p class="t-body c-2">"Please try again in a moment."</p>
            <a class="text-link" href=move || path.get() rel="external"><span class="label">"Reload page"</span><Icon glyph=Glyph::ArrowUpRight/></a>
        </div>
    }
}
