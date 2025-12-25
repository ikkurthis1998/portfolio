use leptos::prelude::*;
use crate::data::{get_portfolio_data, Experience, Project};

#[component]
pub fn Resume() -> impl IntoView {
    let data = get_portfolio_data();
    
    view! {
        <div class="min-h-screen bg-gray-100 py-8 pt-24 print:bg-white print:py-0">
            // Print Button (Hidden when printing)
            // Print Button (Floating Action Button)
            <div class="fixed bottom-8 right-8 z-50 print:hidden">
                <button 
                    id="download-resume-btn"
                    class="bg-slate-900 text-white px-6 py-3 rounded-full hover:bg-slate-800 transition-all shadow-xl hover:shadow-2xl flex items-center gap-2 font-medium transform hover:-translate-y-1"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                    </svg>
                    "Download PDF"
                </button>
            </div>

            // A4 Page Container
            <div id="resume-content" class="max-w-[21cm] mx-auto bg-white shadow-lg print:shadow-none print:max-w-none">
                <div class="p-8 md:p-12 print:p-0">
                    // Header
                    <header class="border-b-2 border-slate-900 pb-6 mb-6 flex justify-between items-start gap-6">
                        <div class="flex-1">
                            <h1 class="text-4xl font-bold text-slate-900 mb-2 uppercase tracking-tight">"Sreemannarayana Ikkurthi"</h1>
                            <p class="text-xl text-slate-600 font-medium mb-4">"Software Engineer"</p>
                            
                            <div class="flex flex-wrap gap-4 text-sm text-slate-600">
                                <div class="flex items-center gap-1.5">
                                    <svg width="16" height="16" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path></svg>
                                    <span>"ikkurthis1998@gmail.com"</span> // Placeholder, replace if desired
                                </div>
                                <div class="flex items-center gap-1.5">
                                    <svg width="16" height="16" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"></path></svg>
                                    <a href="https://isree.dev" class="hover:text-blue-600">"isree.dev"</a>
                                </div>
                                <div class="flex items-center gap-1.5">
                                    <svg width="16" height="16" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>
                                    <a href="https://github.com/ikkurthis1998" class="hover:text-blue-600">"github.com/ikkurthis1998"</a>
                                </div>
                                 <div class="flex items-center gap-1.5">
                                    <svg width="16" height="16" class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/></svg>
                                    <a href="https://linkedin.com/in/ikkurthis1998" class="hover:text-blue-600">"linkedin.com/in/ikkurthis1998"</a>
                                </div>
                            </div>
                        </div>
                        <div class="flex-shrink-0">
                            <img src="/assets/IMG_4872.JPG" alt="Sreemannarayana Ikkurthi" class="w-32 h-44 rounded-md object-cover object-top shadow-sm border border-slate-200 print:w-28 print:h-40" />
                        </div>
                    </header>

                    // Summary
                    <section class="mb-6">
                        <h2 class="text-sm font-bold text-slate-900 uppercase tracking-widest border-b border-slate-200 pb-1 mb-3">"Summary"</h2>
                        <p class="text-sm text-slate-700 leading-relaxed text-justify">
                            {data.about}
                        </p>
                    </section>
                    
                    // Technical Skills
                    <section class="mb-6">
                         <h2 class="text-sm font-bold text-slate-900 uppercase tracking-widest border-b border-slate-200 pb-1 mb-3">"Technical Skills"</h2>
                         <div class="flex flex-wrap gap-2">
                            {data.skills.into_iter().map(|s| view! {
                                <span class="text-xs font-semibold text-slate-700 bg-slate-100 px-2.5 py-1 rounded print:border print:border-slate-200 print:bg-white">{s.name}</span>
                            }).collect::<Vec<_>>()}
                         </div>
                    </section>

                    // Experience
                    <section class="mb-6">
                        <h2 class="text-sm font-bold text-slate-900 uppercase tracking-widest border-b border-slate-200 pb-1 mb-4">"Experience"</h2>
                        <div class="space-y-5">
                            {data.experiences.into_iter().map(|e| view! { <ResumeExperienceItem experience=e/> }).collect::<Vec<_>>()}
                        </div>
                    </section>

                    // Selected Projects
                    <section>
                         <h2 class="text-sm font-bold text-slate-900 uppercase tracking-widest border-b border-slate-200 pb-1 mb-4">"Projects"</h2>
                         <div class="grid grid-cols-1 gap-4">
                            {data.projects.into_iter().take(4).map(|p| view! { <ResumeProjectItem project=p/> }).collect::<Vec<_>>()}
                         </div>
                    </section>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ResumeExperienceItem(experience: Experience) -> impl IntoView {
    view! {
        <div class="break-inside-avoid">
            <div class="flex justify-between items-baseline mb-1">
                <h3 class="font-bold text-slate-900 text-base">{experience.title}</h3>
                <span class="text-xs font-medium text-slate-500 whitespace-nowrap ml-4">{experience.duration}</span>
            </div>
            <div class="text-sm font-medium text-blue-700 mb-1">{experience.company}</div>
            <p class="text-sm text-slate-700 leading-relaxed mb-2 text-justify">{experience.description}</p>
            <div class="flex flex-wrap gap-x-3 gap-y-1">
                {experience.technologies.into_iter().map(|t| view! {
                    <span class="text-[10px] text-slate-500 italic">"• " {t}</span>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn ResumeProjectItem(project: Project) -> impl IntoView {
    view! {
        <div class="break-inside-avoid border border-slate-100 rounded p-3 print:border-none print:p-0">
             <div class="flex justify-between items-baseline mb-1">
                <h3 class="font-bold text-slate-900 text-sm">
                    {project.name}
                    <span class="ml-2 font-normal text-xs text-slate-500 italic">" - " {project.language}</span>
                </h3>
                <a href={project.url} target="_blank" class="text-xs text-blue-600 hover:underline print:text-black print:no-underline">"View Code"</a>
            </div>
            <p class="text-xs text-slate-700 leading-relaxed">{project.description}</p>
        </div>
    }
}
