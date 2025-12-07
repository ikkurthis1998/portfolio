use leptos::prelude::*;
use crate::data::{get_portfolio_data, Experience};

#[component]
pub fn AboutPage() -> impl IntoView {
    let data = get_portfolio_data();
    let experiences = data.experiences;

    view! {
        <div class="min-h-screen bg-slate-50 pt-28 pb-16">
            <div class="max-w-4xl mx-auto px-6">
                <h1 class="text-4xl font-bold text-center mb-12 text-slate-900">"About Me"</h1>
                <div class="bg-white rounded-xl p-8 mb-12 shadow-sm">
                    <p class="text-xl text-slate-700 mb-4 leading-relaxed">"My path hasn't been linear—and that's what makes it interesting."</p>
                    <p class="text-lg text-slate-500 leading-relaxed">"I started in aerospace, drawn to systems that demand precision. That same drive led me to software, where I could create and iterate at a pace I never imagined. I believe great work comes from genuine curiosity."</p>
                </div>
                
                <h2 class="text-2xl font-bold mb-8 text-slate-800">"Experience"</h2>
                <div class="border-l-2 border-slate-200 ml-3 space-y-0">
                    {experiences.into_iter().map(|e| view! { <ExperienceCard experience=e/> }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ExperienceCard(experience: Experience) -> impl IntoView {
    let logo = experience.logo.clone();
    view! {
        <div class="relative pl-8 pb-10 last:pb-0 group">
            <div class="absolute -left-[9px] top-1 w-4 h-4 rounded-full bg-blue-500 border-4 border-slate-50"></div>
            <div class="bg-white rounded-xl p-6 shadow-sm hover:shadow-md transition-shadow">
                <div class="flex items-start gap-4 mb-3">
                    {logo.map(|src| {
                        let zoom_class = if experience.logo_zoom { "w-full h-full object-cover scale-[2]" } else { "w-full h-full object-contain" };
                        view! {
                            <div class="w-12 h-12 rounded-lg bg-slate-50 flex-shrink-0 overflow-hidden">
                                <img src=src alt="Company logo" class=zoom_class/>
                            </div>
                        }
                    })}
                    <div class="flex-1">
                        <div class="flex flex-col sm:flex-row sm:items-start gap-2">
                            <div>
                                <h3 class="text-lg font-bold text-slate-900">{experience.title}</h3>
                                <p class="text-blue-600 font-medium">{experience.company}</p>
                            </div>
                            <span class="text-sm font-semibold text-blue-700 bg-blue-50 px-3 py-1 rounded-full whitespace-nowrap w-fit sm:ml-auto">{experience.duration}</span>
                        </div>
                    </div>
                </div>
                <p class="text-slate-600 mb-4 leading-relaxed">{experience.description}</p>
                <div class="flex flex-wrap gap-2">
                    {experience.technologies.into_iter().map(|t| view! {
                        <span class="text-xs text-slate-600 bg-slate-100 px-2 py-1 rounded-md font-medium">{t}</span>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
