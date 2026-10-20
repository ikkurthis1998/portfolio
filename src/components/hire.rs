use leptos::prelude::*;
use leptos_meta::{Meta, Title};

const UPWORK_URL: &str = "https://www.upwork.com/freelancers/~012bfd2d1589829612";
const FIVERR_URL: &str = "https://www.fiverr.com/ikkurthis1998";

struct Tier {
    name: &'static str,
    price: &'static str,
    scope: &'static str,
    delivery: &'static str,
}

struct Offer {
    name: &'static str,
    summary: &'static str,
    tiers: [Tier; 3],
}

const OFFERS: &[Offer] = &[
    Offer {
        name: "AI assistant on your documents",
        summary: "An AI assistant that answers from your manuals, contracts, research or policies, and shows which source each answer came from.",
        tiers: [
            Tier { name: "Ready on Thelivi", price: "$99", delivery: "2 days",
                scope: "I set it up on Thelivi, the AI assistant I built and run: your documents in a project, custom instructions, cited answers and scheduled runs such as a weekly summary. Thelivi is currently free to use." },
            Tier { name: "Custom", price: "$600", delivery: "10 days",
                scope: "An assistant built into your own website or app: up to 500 pages, chat with cited answers, a clear “not found in your documents” reply, and an admin upload page. You own the code and data." },
            Tier { name: "Custom + integrations", price: "$1,800", delivery: "21 days",
                scope: "Everything in Custom, plus automatic sync from Google Drive, Notion, Dropbox or a database, exact answers from structured data, a test set of your real questions, and team access." },
        ],
    },
    Offer {
        name: "Audit of an AI-built app, with a fixed quote to launch",
        summary: "Built with Lovable, Replit, Bolt or Cursor and almost working? I find what stands between you and launch and give you a fixed price instead of open-ended hourly billing.",
        tiers: [
            Tier { name: "Audit", price: "$150", delivery: "3 days",
                scope: "I run the app and read the code, then send a written list of launch blockers (security, auth, payments, data, deployment) with a fixed quote to fix them." },
            Tier { name: "Audit + quick fixes", price: "$450", delivery: "5 days",
                scope: "The audit, plus fixes for up to 5 blockers." },
            Tier { name: "Launch package", price: "$1,200", delivery: "10 days",
                scope: "The audit, plus fixing blockers, auth and payments, deploying to your domain and setting up basic monitoring." },
        ],
    },
    Offer {
        name: "App Store / Play Store review readiness",
        summary: "Stuck before submitting, or rejected? I check your app against the review rules that most often block launches and fix what’s needed.",
        tiers: [
            Tier { name: "Check", price: "$120", delivery: "2 days",
                scope: "A review against store requirements (account deletion, privacy labels, permission prompts, metadata) and a list of fixes." },
            Tier { name: "Fix + submit", price: "$500", delivery: "7 days",
                scope: "I fix the issues, submit the app and handle one round of review feedback." },
            Tier { name: "Full launch", price: "$900", delivery: "10 days",
                scope: "Fix and submit, with more time and more rounds of revisions." },
        ],
    },
];

#[component]
pub fn HirePage() -> impl IntoView {
    view! {
        <Title text="Hire me — Sreemannarayana Ikkurthi"/>
        <Meta name="description" content="AI document assistants, fixing and launching AI-built apps, and full-stack work. Fixed-price packages on Upwork and Fiverr."/>
        <div class="page-wrap">
            <header class="page-heading"><p class="eyebrow">"Hire me / Fixed-price packages"</p><h1>"AI Document Assistants"<br/><span>"Fix & Launch AI-Built Apps"</span><br/>"Full-Stack"</h1><p>"Three fixed-price packages, each with a clear scope and delivery time. Hire me through Upwork or Fiverr."</p><div class="actions"><a class="button" href=UPWORK_URL target="_blank" rel="noopener noreferrer">"Hire me on Upwork ↗"</a><a class="text-link" href=FIVERR_URL target="_blank" rel="noopener noreferrer">"Hire me on Fiverr ↗"</a></div></header>

            <section id="offers" class="hire-offers" aria-labelledby="offers-title"><div class="section-bar"><h2 id="offers-title" class="eyebrow">"Packages"</h2><span class="mono">"Fixed price · USD"</span></div>
                {OFFERS.iter().enumerate().map(|(i, o)| view! {
                    <article class="offer-row"><div><span class="mono">{format!("{:02}", i + 1)}</span><h3>{o.name}</h3><p>{o.summary}</p></div>
                        <div class="tier-grid">{o.tiers.iter().map(|t| view! {
                            <div class="tier"><p class="mono">{t.name}</p><p class="tier-price">{t.price}</p><p>{t.scope}</p><p class="mono tier-delivery">{format!("Delivery · {}", t.delivery)}</p></div>
                        }).collect_view()}</div>
                    </article>
                }).collect_view()}
            </section>

            <section class="narrative-row"><div><p class="eyebrow">"Proof / Thelivi"</p><h2>"An AI assistant"<br/><span>"I built solo and run."</span></h2></div><div class="prose">
                <p>"Thelivi answers from your uploaded documents and shows the sources behind each answer. It also searches the web, remembers context between conversations and runs scheduled tasks."</p>
                <p>"It has a web app and an iPhone app on the App Store. I designed, built and run it on my own."</p>
                <ul class="topic-list"><li>"Go"</li><li>"Next.js"</li><li>"SwiftUI"</li></ul>
                <div class="actions"><a class="text-link" href="https://thelivi.isree.dev" target="_blank" rel="noopener noreferrer">"Try Thelivi ↗"</a><a class="text-link" href="https://apps.apple.com/app/id6790257232" target="_blank" rel="noopener noreferrer">"iPhone app ↗"</a></div>
            </div></section>

            <section class="narrative-row"><div><p class="eyebrow">"Proof / Vihari"</p><h2>"A native trip tracker"<br/><span>"for iOS and Android."</span></h2></div><div class="prose">
                <p>"Vihari is two native apps, SwiftUI on iOS and Jetpack Compose on Android. It records trips in the background, turns them into stops, routes and trips, and keeps your location history on your phone."</p>
                <ul class="topic-list"><li>"SwiftUI"</li><li>"Jetpack Compose"</li></ul>
                <div class="actions"><a class="text-link" href="/vihari/privacy">"Privacy policy"</a></div>
            </div></section>

            <section class="approach-section" aria-labelledby="process-title">
                <p class="eyebrow section-label">"How I work"</p>
                <div class="approach-grid"><h2 id="process-title">"Fixed scope."<br/><span>"Fixed price."</span></h2>
                    <div><span class="mono">"01"</span><h3>"Written plan first"</h3><p>"You get the scope in writing and a fixed price before any work starts."</p></div>
                    <div><span class="mono">"02"</span><h3>"Progress you can see"</h3><p>"On longer packages you see working demos as I go, so feedback lands early, not at the end."</p></div>
                    <div><span class="mono">"03"</span><h3>"Clean, tested code"</h3><p>"Readable code with tests and docs, handed over so anyone can pick it up."</p></div>
                </div>
            </section>

            <section id="hire-contact" class="contact-section"><p class="eyebrow">"Start a project"</p><div class="contact-grid"><div><h2>"Pick a package to get started."</h2><p>"Hire me through Upwork or Fiverr. Not sure which package fits? Email me a few lines about your project."</p><p class="mono hire-email">"ikkurthis1998@gmail.com"</p><a class="text-link" href="mailto:ikkurthis1998@gmail.com">"Email me ↗"</a></div>
                <div class="social-links"><span class="mono">"Hire me on"</span>
                    <a href=UPWORK_URL target="_blank" rel="noopener noreferrer">"Upwork ↗"</a>
                    <a href=FIVERR_URL target="_blank" rel="noopener noreferrer">"Fiverr ↗"</a>
                </div></div></section>

        </div>
    }
}
