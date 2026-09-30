use leptos::prelude::*;
use leptos_meta::{Meta, Title};

/// Privacy policy for the Vihari trip-journal app (linked from App Store Connect and Play Console).
#[component]
pub fn VihariPrivacyPage() -> impl IntoView {
    view! {
        <Title text="Vihari privacy policy — Sreemannarayana Ikkurthi"/>
        <Meta name="description" content="Vihari is a personal trip journal. Your location history stays on your phone: no account, no servers, no analytics, no ads."/>
        <div class="page-wrap">
            <header class="page-heading"><p class="eyebrow">"Vihari / Privacy policy"</p><h1>"Your history"<br/><span>"stays on your phone."</span></h1><p>"Vihari is a personal trip journal. It records where you go so you can look back on your trips. This policy explains what the app does with that data."</p><p class="mono policy-updated">"Last updated · 30 September 2026"</p></header>
            <section class="narrative-row policy-row"><div><p class="eyebrow">"01"</p><h2>"What Vihari records"</h2></div><div class="prose"><ul class="policy-list">
                <li><strong>"Location"</strong>" — while you move, including in the background when the app is closed, to build your trips, stops and routes."</li>
                <li><strong>"Motion activity"</strong>" — whether you are walking, driving or cycling, to know when to start and stop recording and to save battery."</li>
                <li><strong>"Photos (read-only, optional)"</strong>" — the time and place of photos in your library, to show them on the trips where you took them."</li>
                <li><strong>"Calendar (read-only, optional)"</strong>" — event titles, to name trips (for example “Goa offsite”)."</li>
            </ul></div></section>
            <section class="narrative-row policy-row"><div><p class="eyebrow">"02"</p><h2>"Where it is stored"</h2></div><div class="prose"><p>"Everything Vihari records is stored only on your device. Vihari has no account, no servers, no analytics, no ads and no tracking."</p><p>"I, the developer, never receive your location history, photos or calendar data, and nothing is sold or shared with third parties."</p></div></section>
            <section class="narrative-row policy-row"><div><p class="eyebrow">"03"</p><h2>"The one thing that leaves your phone"</h2></div><div class="prose"><p>"To show place names (for example “Madhapur”), Vihari asks your phone’s built-in map service for the address of a stop. On iPhone this is Apple Maps; on Android it is Google, through Google Play services."</p><p>"Only the stop’s coordinates are sent, and the lookup is covered by Apple’s or Google’s own privacy policy."</p></div></section>
            <section class="narrative-row policy-row"><div><p class="eyebrow">"04"</p><h2>"Your control"</h2></div><div class="prose"><ul class="policy-list">
                <li>"Pause or turn off recording at any time in Settings."</li>
                <li>"Mark private zones where nothing is recorded."</li>
                <li>"Export your history (GPX or JSON) or delete all of it in Settings."</li>
                <li>"Uninstalling the app deletes all of its data."</li>
                <li>"Withdraw location, motion, photo or calendar access at any time in your phone’s settings."</li>
            </ul></div></section>
            <section class="narrative-row policy-row"><div><p class="eyebrow">"05"</p><h2>"Children"</h2></div><div class="prose"><p>"Vihari is not directed at children under 13."</p></div></section>
            <section class="narrative-row policy-row"><div><p class="eyebrow">"06"</p><h2>"Changes and contact"</h2></div><div class="prose"><p>"If this policy changes, the new version will be posted at this address with a new date."</p><p>"Questions: "<a class="text-link" href="mailto:ikkurthis1998@gmail.com">"ikkurthis1998@gmail.com"</a></p></div></section>
        </div>
    }
}
