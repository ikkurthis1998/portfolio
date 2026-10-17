use leptos::prelude::*;
use leptos_meta::{Meta, Title};

// TODO(signup provider): paste the form endpoint from Kit or Buttondown here. While it is empty
// the form stays visible but submitting only shows a "not open yet" note; nothing is sent.
//   Buttondown: "https://buttondown.com/api/emails/embed-subscribe/<username>", field "email"
//   Kit:        "https://app.kit.com/forms/<form-id>/subscriptions",          field "email_address"
const SIGNUP_FORM_ACTION: &str = "";
const SIGNUP_EMAIL_FIELD: &str = "email";

// TODO(links): replace with real profile URLs. Empty links are hidden.
const UPWORK_URL: &str = "";
const CONTRA_URL: &str = "";

struct Service {
    name: &'static str,
    price: &'static str,
    summary: &'static str,
    includes: &'static [&'static str],
}

const SERVICES: &[Service] = &[
    Service { name: "AI agent or RAG chatbot", price: "From $150",
        summary: "An assistant that answers from your documents or website and can call your tools, with sources shown for every answer.",
        includes: &["Document and website ingestion", "Retrieval with citations", "Tool calls and integrations", "Embeddable chat widget", "Usage limits and logging"] },
    Service { name: "SaaS MVP", price: "From $300",
        summary: "A working first version of your product that real users can sign up to and pay for.",
        includes: &["Next.js front end", "Go or Node API", "Auth and user accounts", "Stripe or Razorpay payments", "Deployment and admin basics"] },
    Service { name: "Native iOS or Android app", price: "From [price]",
        summary: "A native app built in SwiftUI or Jetpack Compose, ready for the App Store or Play Store.",
        includes: &["SwiftUI or Jetpack Compose", "Offline-first local storage", "Widgets and background work", "Store listing and privacy policy", "Release builds and submission"] },
    Service { name: "Scraper or automation", price: "From $60",
        summary: "A script or scheduled job that collects data or takes repetitive work off your hands.",
        includes: &["Headless browser or API scraping", "Scheduled runs", "Export to CSV, Sheets or a database", "Alerts when something changes", "Notes on how to run it"] },
];

#[component]
pub fn HirePage() -> impl IntoView {
    view! {
        <Title text="Hire me — Sreemannarayana Ikkurthi"/>
        <Meta name="description" content="AI agent and full-stack engineer who ships complete products: AI agents and RAG chatbots, SaaS MVPs, native mobile apps, scrapers and automation."/>
        <div class="page-wrap">
            <header class="page-heading"><p class="eyebrow">"Hire me / Freelance"</p><h1>"AI agent and full-stack engineer"<br/><span>"who ships complete products."</span></h1><p>"From the first written plan to a deployed product: back end, front end, mobile apps and the AI in between."</p><div class="actions"><a class="button" href="#hire-contact">"Start a project ↗"</a><a class="text-link" href="#services">"See services"</a></div></header>

            <section id="services" class="hire-services" aria-labelledby="services-title"><div class="section-bar"><h2 id="services-title" class="eyebrow">"Services"</h2><span class="mono">"Fixed estimate before work starts"</span></div>
                <div class="service-grid">{SERVICES.iter().enumerate().map(|(i, s)| view! {
                    <article class="service-card"><span class="mono">{format!("{:02}", i + 1)}</span><h3>{s.name}</h3><p class="service-price">{s.price}</p><p>{s.summary}</p><ul class="topic-list">{s.includes.iter().map(|item| view! { <li>{*item}</li> }).collect_view()}</ul></article>
                }).collect_view()}</div>
            </section>

            <section class="narrative-row"><div><p class="eyebrow">"Proof / Thelivi"</p><h2>"An AI agent platform"<br/><span>"I built and run alone."</span></h2></div><div class="prose">
                <p>"Thelivi is a research agent that reads your documents, searches the web and runs recurring tasks on a schedule. It has a Go back end on Temporal and SurrealDB, a Next.js web app, native iOS and Android apps, and an embeddable chat widget for other websites."</p>
                <p>"Answers cite the exact lines they came from, highlighted on the page image. Anonymous widget visitors get a locked-down toolset that is enforced twice, so a prompt-injected request can’t run anything outside it."</p>
                <ul class="topic-list"><li>"Go"</li><li>"Temporal"</li><li>"SurrealDB"</li><li>"Next.js"</li><li>"SwiftUI"</li><li>"Jetpack Compose"</li></ul>
                <div class="actions"><a class="text-link" href="https://thelivi.isree.dev/" target="_blank" rel="noopener noreferrer">"Try Thelivi ↗"</a><a class="text-link" href="/projects/intelligence">"Read more"</a></div>
            </div></section>

            <section class="narrative-row"><div><p class="eyebrow">"Proof / Vihari"</p><h2>"A trip journal that keeps"<br/><span>"your history on your phone."</span></h2></div><div class="prose">
                <p>"Vihari is two native apps, SwiftUI on iOS and Jetpack Compose on Android, built from one written spec. It records trips in the background, turns them into stops, routes and trips, and sends no location history to any server."</p>
                <p>"GPS runs only while you are moving, so it fits a small daily battery budget. Both apps are tested against the same shared set of recorded journeys, so they produce the same numbers."</p>
                <ul class="topic-list"><li>"SwiftUI"</li><li>"SwiftData"</li><li>"Jetpack Compose"</li><li>"Room"</li><li>"MapLibre"</li></ul>
                <div class="actions"><a class="text-link" href="/vihari/privacy">"Privacy policy"</a></div>
            </div></section>

            <section class="approach-section" aria-labelledby="process-title">
                <p class="eyebrow section-label">"How I work"</p>
                <div class="approach-grid"><h2 id="process-title">"No surprises."<br/><span>"You see progress as it happens."</span></h2>
                    <div><span class="mono">"01"</span><h3>"Plan and fixed estimate"</h3><p>"You get a written plan with scope, milestones and a fixed price before any code is written."</p></div>
                    <div><span class="mono">"02"</span><h3>"Demos every few days"</h3><p>"A working build you can click through every few days, so feedback lands early, not at the end."</p></div>
                    <div><span class="mono">"03"</span><h3>"Clean, tested code"</h3><p>"Readable code with tests and docs, handed over in your repository so anyone can pick it up."</p></div>
                </div>
            </section>

            <section id="hire-contact" class="contact-section"><p class="eyebrow">"Start a project"</p><div class="contact-grid"><div><h2>"Tell me what you need built."</h2><p>"Send a few lines about the problem. I’ll reply with questions or a written plan and estimate."</p><p class="mono hire-email">"ikkurthis1998@gmail.com"</p><a class="text-link" href="mailto:ikkurthis1998@gmail.com">"Email me ↗"</a></div>
                <div class="social-links"><span class="mono">"Also on"</span>
                    {(!UPWORK_URL.is_empty()).then(|| view! { <a href=UPWORK_URL target="_blank" rel="noopener noreferrer">"Upwork ↗"</a> })}
                    {(!CONTRA_URL.is_empty()).then(|| view! { <a href=CONTRA_URL target="_blank" rel="noopener noreferrer">"Contra ↗"</a> })}
                    <a href="https://github.com/ikkurthis1998" target="_blank" rel="noopener noreferrer">"GitHub ↗"</a><a href="https://linkedin.com/in/ikkurthis1998" target="_blank" rel="noopener noreferrer">"LinkedIn ↗"</a>
                </div></div></section>

            <SignupForm/>
        </div>
    }
}

/// "Get notified" email signup. Posts straight to the newsletter provider once SIGNUP_FORM_ACTION is set.
#[component]
pub fn SignupForm() -> impl IntoView {
    let (note, set_note) = signal(None::<&'static str>);
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        if SIGNUP_FORM_ACTION.is_empty() {
            ev.prevent_default();
            set_note.set(Some("Signups aren’t open yet. Email me and I’ll add you to the list."));
        }
    };
    view! {
        <section id="signup" class="signup-section" aria-labelledby="signup-title">
            <div><p class="eyebrow">"Newsletter"</p><h2 id="signup-title">"Get notified when I launch new tools."</h2><p>"One short email per launch. No spam, unsubscribe any time."</p></div>
            <form class="signup-form" method="post" action=SIGNUP_FORM_ACTION target="_blank" on:submit=on_submit>
                <label class="mono" for="signup-email">"Email address"</label>
                <div class="signup-row"><input id="signup-email" type="email" name=SIGNUP_EMAIL_FIELD placeholder="you@example.com" autocomplete="email" required/><button class="button" type="submit">"Notify me ↗"</button></div>
                {move || note.get().map(|n| view! { <p class="mono signup-note" role="status">{n}</p> })}
            </form>
        </section>
    }
}
