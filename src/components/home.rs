use super::ui::{
    Brand, Chips, Contact, DataError, HowIWork, Loading, ProductMark, SectionHead, TextLink, ThemedImage,
    THELIVI_IOS_URL, THELIVI_URL,
};
use crate::data::{Experience, fetch_portfolio_data};
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Live,
    Testing,
    Development,
}

impl Status {
    fn label(self) -> &'static str {
        match self {
            Status::Live => "Live",
            Status::Testing => "In testing",
            Status::Development => "In development",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Shot {
    Phone,
    Browser,
}

struct Product {
    name: &'static str,
    brand: Brand,
    purpose: &'static str,
    node_platforms: &'static str,
    description: &'static str,
    platforms: &'static str,
    status: Status,
    shot: Shot,
    shot_alt: &'static str,
    light: &'static str,
    dark: &'static str,
}

// Copy from the v2 home frames; statuses per the family map (one status per product).
const PRODUCTS: &[Product] = &[
    Product {
        name: "Thelivi", brand: Brand::Thelivi, purpose: "Personal assistant", node_platforms: "Web · iOS · Android coming soon",
        description: "Your personal assistant, grounded in real sources. Ask anything, answered with cited sources.",
        platforms: "Web and iPhone · Android coming soon", status: Status::Live, shot: Shot::Phone,
        shot_alt: "Thelivi home screen on iPhone", light: "/assets/v2/shot-thelivi-light.webp", dark: "/assets/v2/shot-thelivi-dark.webp",
    },
    Product {
        name: "Vihari", brand: Brand::Vihari, purpose: "Trip journal", node_platforms: "iOS · Android",
        description: "A personal trip journal that records your stops and trips in the background. All data stays on your phone.",
        platforms: "iPhone and Android", status: Status::Testing, shot: Shot::Phone,
        shot_alt: "Vihari Today screen recording a trip", light: "/assets/v2/shot-vihari-light.webp", dark: "/assets/v2/shot-vihari-dark.webp",
    },
    Product {
        name: "Suddhi", brand: Brand::Suddhi, purpose: "Launch check", node_platforms: "Web",
        description: "A launch check for AI-built apps. Scan a live URL, or check an App Store or Play build before you submit.",
        platforms: "Web", status: Status::Development, shot: Shot::Browser,
        shot_alt: "Suddhi marketing home page", light: "/assets/v2/shot-suddhi-light.webp", dark: "/assets/v2/shot-suddhi-dark.webp",
    },
    Product {
        name: "Gamani", brand: Brand::Gamani, purpose: "Automation alerts", node_platforms: "Web",
        description: "Plain-English alerts when n8n, Make or Zapier automations fail: what broke, why, and how to fix it.",
        platforms: "Web", status: Status::Development, shot: Shot::Browser,
        shot_alt: "Gamani marketing home page", light: "/assets/v2/shot-gamani-light.webp", dark: "/assets/v2/shot-gamani-dark.webp",
    },
];

const STACK: &[(&str, &[&str])] = &[
    ("Mobile apps", &["Swift", "SwiftUI", "SwiftData", "Kotlin", "Jetpack Compose", "Room"]),
    ("Web", &["TypeScript", "Next.js", "Leptos (Rust)", "Tailwind CSS"]),
    ("Backend", &["Go", "Node.js", "Express.js", "Temporal", "REST APIs", "Microservices"]),
    ("AI and data", &["Ollama", "Agent workflows", "Embeddings", "SurrealDB", "PostgreSQL", "MongoDB"]),
    ("EV charging", &["OCPP / OCPI", "Payment systems", "OTA updates", "Billing & settlements"]),
    ("Aerospace", &["MATLAB", "Python", "XFOIL", "Optimization"]),
];

fn status_badge(status: Status) -> impl IntoView {
    view! { <span class="badge" class:badge-success=status == Status::Live>{status.label()}</span> }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let data = Resource::new(|| (), |_| fetch_portfolio_data());
    view! {
        <div class="wrap page">
            <section class="hero" aria-labelledby="hero-title">
                <div class="hero-copy">
                    <div class="hero-text">
                        <div class="hero-heading">
                            <p class="eyebrow">"Software architect / Aerospace background"</p>
                            <h1 id="hero-title" class="t-display-web">"From first principles"<br/>"to working products."</h1>
                        </div>
                        <p class="t-prose">"I’m Sreemannarayana. I design and build software systems—from EV charging infrastructure to AI products."</p>
                    </div>
                    <div class="actions">
                        <a class="btn btn-primary" href="/projects">"Explore my work"</a>
                        <a class="btn btn-secondary" href="/about">"About me"</a>
                    </div>
                </div>
                <FamilyMap/>
            </section>

            <section id="products" class="section" aria-labelledby="products-title">
                <SectionHead label="Products" heading="Thelivi, Vihari, Suddhi and Gamani." heading_id="products-title"/>
                <ul class="grid-2 products-grid">{PRODUCTS.iter().map(|p| view! { <ProductCard product=p/> }).collect_view()}</ul>
            </section>

            <HowIWork/>

            <section class="section" aria-labelledby="experience-title">
                <div class="section-text">
                    <div class="section-head">
                        <div class="section-bar"><p class="eyebrow">"Experience"</p><TextLink href="/resume".into() label="View resume"/></div>
                        <h2 class="t-display" id="experience-title">"From rocket motors"<br class="br-md"/>" to EV chargers to AI."</h2>
                    </div>
                    <p class="t-prose measure">"Aerospace engineering, then charging infrastructure at Bolt.Earth, now software architecture for AI products at Neurofin."</p>
                </div>
                <Suspense fallback=|| view! { <Loading/> }>{move || data.get().map(|result| match result {
                    Ok(d) => view! { <ul class="grid-2 career-grid">{group_careers(&d.experiences).into_iter().map(|c| view! { <CareerCard career=c/> }).collect_view()}</ul> }.into_any(),
                    Err(_) => view! { <DataError/> }.into_any(),
                })}</Suspense>
            </section>

            <section class="section" aria-labelledby="stack-title">
                <SectionHead label="Stack" heading="Tools by layer." heading_id="stack-title"/>
                <ul class="grid-3 stack-grid">{STACK.iter().map(|(title, tools)| view! {
                    <li class="card stack-card"><h3 class="t-headline">{*title}</h3><Chips items=tools.iter().map(|t| t.to_string()).collect()/></li>
                }).collect_view()}</ul>
            </section>

            <Contact/>
        </div>
    }
}

#[component]
fn FamilyMap() -> impl IntoView {
    view! {
        <div class="family-map" role="list" aria-label="The isree.dev product family">
            <span class="connector c-left" aria-hidden="true"></span>
            <span class="connector c-right" aria-hidden="true"></span>
            <span class="connector c-bar" aria-hidden="true"></span>
            <span class="family-hub lockup" aria-hidden="true"><ProductMark brand=Brand::Isree/><span>"isree.dev"</span></span>
            {PRODUCTS.iter().map(|p| view! {
                <div class="card family-node" role="listitem">
                    <div class="node-top"><span class="node-id"><ProductMark brand=p.brand/>{p.name}</span>{status_badge(p.status)}</div>
                    <div class="node-text"><p class="t-body">{p.purpose}</p><p class="t-caption c-3">{p.node_platforms}</p></div>
                </div>
            }).collect_view()}
        </div>
    }
}

#[component]
fn ProductCard(product: &'static Product) -> impl IntoView {
    let p = product;
    let shot_class = if p.shot == Shot::Phone { "shot shot-phone" } else { "shot shot-browser" };
    view! {
        <li class="card product-card">
            <div class="well"><div class=shot_class><ThemedImage light=p.light dark=p.dark alt=p.shot_alt.to_string()/></div></div>
            <div class="product-body">
                <div class="product-info">
                    <h3 class="product-title"><ProductMark brand=p.brand/>{p.name}</h3>
                    <div class="product-text"><p class="t-prose">{p.description}</p><p class="t-caption c-3">{p.platforms}</p></div>
                </div>
                <div class="product-footer">
                    {status_badge(p.status)}
                    {(p.brand == Brand::Thelivi).then(|| view! {
                        <TextLink href=THELIVI_URL.into() label="Open Thelivi" external=true class="link-thelivi"/>
                        <TextLink href=THELIVI_IOS_URL.into() label="iPhone app" external=true class="link-thelivi" aria_label="Thelivi iPhone app on the App Store"/>
                    })}
                </div>
            </div>
        </li>
    }
}

/// One organisation on the home Experience band: its roles oldest → newest.
#[derive(Clone, Debug, PartialEq)]
pub struct Career {
    pub company: String,
    pub span: String,
    pub logo: Option<&'static str>,
    pub roles: Vec<Role>,
    pub chips: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Role {
    pub title: String,
    pub start: String,
    pub end: String,
    pub current: bool,
}

fn org_logo(company: &str) -> Option<&'static str> {
    let c = company.to_lowercase();
    if c.contains("neurofin") {
        Some("/assets/v2/logo-neurofin.png")
    } else if c.contains("bolt") {
        Some("/assets/v2/logo-bolt.png")
    } else if c.contains("space technology") {
        Some("/assets/v2/logo-star.png")
    } else if c.contains("amrita") {
        Some("/assets/v2/logo-amrita.png")
    } else {
        None
    }
}

/// "Apr 2026 – Present" → ("Apr 2026", "Now").
fn split_duration(duration: &str) -> (String, String) {
    let mut parts = duration.splitn(2, '–').map(str::trim);
    let start = parts.next().unwrap_or_default().to_string();
    let end = parts.next().unwrap_or_default();
    let end = if end.eq_ignore_ascii_case("present") { "Now".to_string() } else { end.to_string() };
    (start, end)
}

/// The staircase shows short titles: "Member of Technical Staff - 2 (MTS-2-Software)" → "Member of Technical Staff - 2".
fn short_title(title: &str) -> String {
    match title.find(" (") {
        Some(i) if title.ends_with(')') => title[..i].to_string(),
        _ => title.to_string(),
    }
}

/// Groups the newest-first experience list into consecutive organisations.
pub fn group_careers(experiences: &[Experience]) -> Vec<Career> {
    let mut careers: Vec<Career> = Vec::new();
    for e in experiences {
        let (start, end) = split_duration(&e.duration);
        let role = Role { title: short_title(&e.title), current: end == "Now", start, end };
        match careers.last_mut() {
            Some(c) if c.company == e.company => c.roles.insert(0, role),
            _ => careers.push(Career {
                company: e.company.clone(),
                span: String::new(),
                logo: org_logo(&e.company),
                // The latest role's first three topics.
                chips: e.technologies.iter().take(3).cloned().collect(),
                roles: vec![role],
            }),
        }
    }
    for c in &mut careers {
        let first = c.roles.first().map(|r| r.start.clone()).unwrap_or_default();
        let last = c.roles.last().map(|r| r.end.clone()).unwrap_or_default();
        c.span = format!("{first} – {last}");
    }
    careers
}

#[component]
fn CareerCard(career: Career) -> impl IntoView {
    view! {
        <li class="card career-card">
            <div class="org">
                <span class="org-logo">{career.logo.map(|src| view! { <img src=src alt="" width="32" height="32"/> })}</span>
                <div class="org-text"><h3 class="t-headline">{career.company.clone()}</h3><p class="t-caption c-2">{career.span.clone()}</p></div>
            </div>
            <ol class="stair" aria-label=format!("Roles at {}", career.company)>
                {career.roles.into_iter().enumerate().map(|(i, r)| view! {
                    <li class="step" class:is-current=r.current style=format!("padding-bottom:{}px", 16 * i)>
                        <div class="step-label">
                            <p class="role-line"><span class="t-label role-title">{r.title}</span>{r.current.then(|| view! { <span class="badge">"Now"</span> })}</p>
                            <p class="t-caption c-2 role-dates">{format!("{} – {}", r.start, r.end)}</p>
                        </div>
                        <div class="track" aria-hidden="true">{(i > 0).then(|| view! { <span class="riser"></span> })}<span class="dot"></span></div>
                    </li>
                }).collect_view()}
            </ol>
            <Chips items=career.chips/>
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn exp(title: &str, company: &str, duration: &str, tech: &[&str]) -> Experience {
        Experience {
            title: title.into(),
            company: company.into(),
            duration: duration.into(),
            description: String::new(),
            technologies: tech.iter().map(|t| t.to_string()).collect(),
            logo: None,
            logo_zoom: false,
        }
    }
    #[test]
    fn careers_group_by_company_with_oldest_role_first() {
        let careers = group_careers(&[
            exp("Software Architect", "Neurofin AI Technologies", "Apr 2026 – Present", &["A", "B", "C", "D"]),
            exp("Founding Engineer", "Neurofin AI Technologies", "May 2024 – Mar 2026", &["E"]),
            exp("Member of Technical Staff - 2 (MTS-2-Software)", "Bolt.Earth", "Apr 2023 – May 2024", &["TypeScript"]),
            exp("Intern - Full Stack", "Bolt.Earth", "Jul 2021 – Sep 2021", &[]),
            exp("B.Tech, Aerospace Engineering", "Amrita Vishwa Vidyapeetham, Coimbatore", "2016 – 2020", &[]),
        ]);
        assert_eq!(careers.len(), 3);
        assert_eq!(careers[0].span, "May 2024 – Now");
        assert_eq!(careers[0].roles[0].title, "Founding Engineer");
        assert!(careers[0].roles[1].current && !careers[0].roles[0].current);
        assert_eq!(careers[0].chips, ["A", "B", "C"]);
        assert_eq!(careers[1].roles[1].title, "Member of Technical Staff - 2");
        assert_eq!(careers[1].span, "Jul 2021 – May 2024");
        assert_eq!(careers[2].span, "2016 – 2020");
        assert!(careers[2].logo.is_some());
    }
}
