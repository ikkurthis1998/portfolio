use super::ui::{Chips, StepCopy, Steps, TextLink, EMAIL, THELIVI_IOS_URL, THELIVI_URL};
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
        <div class="wrap page">
            <header class="page-heading">
                <div class="heading-text">
                    <div class="heading-main"><p class="eyebrow">"Hire me / Fixed-price packages"</p>
                        <h1 class="t-display-web">"AI Document"<br class="br-md"/>" Assistants"<br/>"Fix & Launch"<br class="br-md"/>" AI-Built Apps"<br/>"Full-Stack"</h1></div>
                    <p class="t-prose measure">"Three fixed-price packages, each with a clear scope and delivery time. Hire me through Upwork or Fiverr."</p>
                </div>
                <div class="actions hire-actions">
                    <a class="btn btn-primary" href=UPWORK_URL target="_blank" rel="noopener noreferrer">"Hire me on Upwork"</a>
                    <a class="btn btn-secondary" href=FIVERR_URL target="_blank" rel="noopener noreferrer">"Hire me on Fiverr"</a>
                </div>
            </header>

            <section id="offers" class="offers" aria-labelledby="offers-title">
                <div class="section-bar"><h2 id="offers-title" class="eyebrow">"Packages"</h2><span class="t-caption c-3">"Fixed price · USD"</span></div>
                {OFFERS.iter().enumerate().map(|(i, o)| view! {
                    <article class="offer">
                        <div class="offer-head">
                            <div class="offer-name"><p class="num">{format!("{:02}", i + 1)}</p><h3 class="t-display">{o.name}</h3></div>
                            <p class="t-prose">{o.summary}</p>
                        </div>
                        <ul class="grid-3 tiers">{o.tiers.iter().map(|t| view! {
                            <li class="card tier-card">
                                <p class="tier-name">{t.name}</p>
                                <p class="t-display">{t.price}</p>
                                <p class="t-body c-2">{t.scope}</p>
                                <p class="t-caption c-3">{format!("Delivery · {}", t.delivery)}</p>
                            </li>
                        }).collect_view()}</ul>
                    </article>
                }).collect_view()}
            </section>

            <section class="split" aria-labelledby="proof-thelivi">
                <div class="lead"><p class="eyebrow">"Proof / Thelivi"</p><h2 id="proof-thelivi" class="t-display">"An AI assistant"<br/>"I built solo and run."</h2></div>
                <div class="prose">
                    <p class="t-prose">"Thelivi answers from your uploaded documents and shows the sources behind each answer. It also searches the web, remembers context between conversations and runs scheduled tasks."</p>
                    <p class="t-prose">"It has a web app and an iPhone app on the App Store. I designed, built and run it on my own."</p>
                    <Chips items=vec!["Go".into(), "Next.js".into(), "SwiftUI".into()]/>
                    <div class="proof-links">
                        <TextLink href=THELIVI_URL.into() label="Try Thelivi" external=true/>
                        <TextLink href=THELIVI_IOS_URL.into() label="iPhone app" external=true aria_label="Thelivi iPhone app on the App Store"/>
                    </div>
                </div>
            </section>

            <section class="split" aria-labelledby="proof-vihari">
                <div class="lead"><p class="eyebrow">"Proof / Vihari"</p><h2 id="proof-vihari" class="t-display">"A native trip tracker"<br/>"for iOS and Android."</h2></div>
                <div class="prose">
                    <p class="t-prose">"Vihari is two native apps, SwiftUI on iOS and Jetpack Compose on Android. It records trips in the background, turns them into stops, routes and trips, and keeps your location history on your phone."</p>
                    <Chips items=vec!["SwiftUI".into(), "Jetpack Compose".into()]/>
                    <div class="proof-links"><TextLink href="/vihari/privacy".into() label="Privacy policy"/></div>
                </div>
            </section>

            <section class="section" aria-labelledby="process-title">
                <div class="section-head"><div class="section-bar"><p class="eyebrow">"How I work"</p></div><h2 id="process-title" class="t-display">"Fixed scope. Fixed price."</h2></div>
                <Steps steps=HIRE_STEPS/>
            </section>

            <section id="hire-contact" class="contact" aria-labelledby="hire-contact-title">
                <div class="section-text">
                    <div class="section-head"><div class="section-bar"><p class="eyebrow">"Start a project"</p></div><h2 id="hire-contact-title" class="t-display">"Pick a package to get started."</h2></div>
                    <p class="t-prose measure">"Hire me through Upwork or Fiverr. Not sure which package fits? Email me a few lines about your project."</p>
                    <p class="hire-email">{EMAIL}</p>
                </div>
                <div class="actions">
                    <a class="btn btn-primary" href=format!("mailto:{EMAIL}")>"Email me"</a>
                    <a class="btn btn-secondary" href=UPWORK_URL target="_blank" rel="noopener noreferrer">"Upwork"</a>
                    <a class="btn btn-secondary" href=FIVERR_URL target="_blank" rel="noopener noreferrer">"Fiverr"</a>
                </div>
            </section>
        </div>
    }
}

const HIRE_STEPS: &[StepCopy] = &[
    StepCopy { title: "Written plan first", body: "You get the scope in writing and a fixed price before any work starts." },
    StepCopy { title: "Progress you can see", body: "On longer packages you see working demos as I go, so feedback lands early, not at the end." },
    StepCopy { title: "Clean, tested code", body: "Readable code with tests and docs, handed over so anyone can pick it up." },
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hire_copy_never_mentions_the_employer() {
        let mut text = String::new();
        for o in OFFERS {
            text.push_str(o.name);
            text.push_str(o.summary);
            for t in &o.tiers {
                text.push_str(t.name);
                text.push_str(t.scope);
            }
        }
        for s in HIRE_STEPS {
            text.push_str(s.title);
            text.push_str(s.body);
        }
        assert!(!text.to_lowercase().contains("neurofin"));
    }
}
