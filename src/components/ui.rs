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
    Download,
}

impl Glyph {
    fn paths(self) -> &'static [&'static str] {
        match self {
            Glyph::ArrowUpRight => &["M7 7h10v10", "M7 17 17 7"],
            Glyph::ArrowUp => &["m5 12 7-7 7 7", "M12 19V5"],
            Glyph::ArrowLeft => &["m12 19-7-7 7-7", "M19 12H5"],
            Glyph::Download => &["M12 4v11", "m7 10 5 5 5-5", "M5 20h14"],
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

/// Product marks, from the final isree-design v2 assets (24 grid, 1.8 round strokes,
/// 1.6 pad dot, drawn in currentColor). isree.dev shows its "si" mark untiled in
/// accent-text; the products show their rounded accent-fill tile (favicon.svg) with a
/// white mark.
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

    /// The mark's paths (mark.svg), in currentColor.
    fn paths(self) -> AnyView {
        match self {
            // "si": circuit trace with 45° bends from a via ring up to a pad dot.
            Brand::Isree => view! {
                <g transform="translate(0.2 -0.2)">
                    <circle cx="6.6" cy="18.4" r="1.3" stroke="currentColor" stroke-width="1.5" fill="none"/>
                    <path d="M8.1 18.4H12.4L14.6 16.2V15.6L13.4 14.4H10.6L9.4 13.2V12.6L11.6 10.4H14.6L16.4 8.6" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
                    <circle cx="16.4" cy="5.5" r="1.6" fill="currentColor"/>
                </g>
            }
            .into_any(),
            Brand::Thelivi => view! {
                <path d="M18.04 10.21A6.3 6.3 0 1 1 13.79 5.96" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
                <circle cx="16.45" cy="7.55" r="1.6" fill="currentColor"/>
            }
            .into_any(),
            Brand::Vihari => view! {
                <g transform="translate(0.6 0.1)">
                    <path d="M4.5 18C8.8 18 8.2 12.4 11.4 12.4C13.4 12.4 14.6 10.6 15.58 9.06" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
                    <circle cx="17.5" cy="6.5" r="1.6" fill="currentColor"/>
                </g>
            }
            .into_any(),
            Brand::Suddhi => view! {
                <path d="M6.85 13.25L10.10 17.15L14.74 10.13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
                <circle cx="16.45" cy="7.55" r="1.6" fill="currentColor"/>
            }
            .into_any(),
            Brand::Gamani => view! {
                <g transform="translate(0.2 -0.2)">
                    <path d="M6.8 16.4C7.9 15.4 8.2 14 8.2 12.2C8.2 9.6 9.7 7.9 11.6 7.9C13.5 7.9 15 9.6 15 12.2C15 14 15.3 15.4 16.4 16.4Z" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
                    <path d="M10.2 19H13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
                    <circle cx="16.6" cy="7.0" r="1.6" fill="currentColor"/>
                </g>
            }
            .into_any(),
        }
    }
}

#[component]
pub fn ProductMark(brand: Brand) -> impl IntoView {
    let tiled = brand != Brand::Isree;
    view! {
        <svg class=format!("mark {}", brand.class()) width="32" height="32" viewBox="0 0 24 24" fill="none" aria-hidden="true" focusable="false">
            {tiled.then(|| view! { <rect class="mark-tile" width="24" height="24" rx="5.5"/> }.into_any())}
            {brand.paths()}
        </svg>
    }
    .into_any()
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
