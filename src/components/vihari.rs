use super::ui::{Glyph, TextLink, EMAIL};
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

/// Privacy policy for the Vihari trip-journal app (linked from App Store Connect and Play Console).
#[component]
pub fn VihariPrivacyPage() -> impl IntoView {
    view! {
        <Title text="Vihari privacy policy — Sreemannarayana Ikkurthi"/>
        <Meta name="description" content="Vihari is a personal trip journal. Your location history stays on your phone: no account, no servers, no analytics, no ads."/>
        <div class="wrap page">
            <header class="page-heading">
                <div class="heading-text">
                    <div class="heading-main"><p class="eyebrow">"Vihari / Privacy policy"</p><h1 class="t-display-web">"Your history"<br/>"stays on your phone."</h1></div>
                    <p class="t-prose measure">"Vihari is a personal trip journal. It records where you go so you can look back on your trips. This policy explains what the app does with that data."</p>
                    <p class="t-caption c-3">"Last updated · 30 September 2026"</p>
                </div>
            </header>
            <div class="policy">
                <PolicySection number="01" title="What Vihari records">
                    <ul class="policy-list t-prose">
                        <li><span><strong>"Location"</strong>" — while you move, including in the background when the app is closed, to build your trips, stops and routes."</span></li>
                        <li><span><strong>"Motion activity"</strong>" — whether you are walking, driving or cycling, to know when to start and stop recording and to save battery."</span></li>
                        <li><span><strong>"Photos (read-only, optional)"</strong>" — the time and place of photos in your library, to show them on the trips where you took them."</span></li>
                        <li><span><strong>"Calendar (read-only, optional)"</strong>" — event titles, to name trips (for example “Goa offsite”)."</span></li>
                    </ul>
                </PolicySection>
                <PolicySection number="02" title="Where it is stored">
                    <p class="t-prose">"Everything Vihari records is stored only on your device. Vihari has no account, no servers, no analytics, no ads and no tracking."</p>
                    <p class="t-prose">"I, the developer, never receive your location history, photos or calendar data, and nothing is sold or shared with third parties."</p>
                </PolicySection>
                <PolicySection number="03" title="The one thing that leaves your phone">
                    <p class="t-prose">"To show place names (for example “Madhapur”), Vihari asks your phone’s built-in map service for the address of a stop. On iPhone this is Apple Maps; on Android it is Google, through Google Play services."</p>
                    <p class="t-prose">"Only the stop’s coordinates are sent, and the lookup is covered by Apple’s or Google’s own privacy policy."</p>
                </PolicySection>
                <PolicySection number="04" title="Your control">
                    <ul class="policy-list t-prose">
                        <li><span>"Pause or turn off recording at any time in Settings."</span></li>
                        <li><span>"Mark private zones where nothing is recorded."</span></li>
                        <li><span>"Export your history (GPX or JSON) or delete all of it in Settings."</span></li>
                        <li><span>"Uninstalling the app deletes all of its data."</span></li>
                        <li><span>"Withdraw location, motion, photo or calendar access at any time in your phone’s settings."</span></li>
                    </ul>
                </PolicySection>
                <PolicySection number="05" title="Children">
                    <p class="t-prose">"Vihari is not directed at children under 13."</p>
                </PolicySection>
                <PolicySection number="06" title="Changes and contact">
                    <p class="t-prose">"If this policy changes, the new version will be posted at this address with a new date."</p>
                    <p class="policy-contact"><span class="t-prose">"Questions:"</span><TextLink href=format!("mailto:{EMAIL}") label=EMAIL glyph=Glyph::Mail/></p>
                </PolicySection>
            </div>
        </div>
    }
}

#[component]
fn PolicySection(number: &'static str, title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="policy-section">
            <div class="lead"><p class="eyebrow">{number}</p><h2 class="t-headline">{title}</h2></div>
            <div class="prose">{children()}</div>
        </section>
    }
}
