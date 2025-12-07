use leptos::prelude::*;
use crate::data::get_portfolio_data;
use super::projects::ProjectCard;

#[component]
pub fn HomePage() -> impl IntoView {
    let data = get_portfolio_data();
    let projects = data.projects;
    
    view! {
        <div class="min-h-screen bg-slate-50">
            <section class="max-w-4xl mx-auto px-6 py-32 md:py-48">
                <h1 class="text-4xl md:text-6xl font-bold mb-6">"Hi, I'm Sreemannarayana."</h1>
                <p class="text-xl text-slate-500 mb-10">"Full Stack Developer & Aerospace Engineer."</p>
                <p class="text-lg text-slate-600 mb-12">
                    "I'm driven by curiosity and a love for solving complex problems. My journey from aerospace to software taught me that the best solutions come from thinking deeply and building deliberately. "
                    <a href="/about" class="text-blue-600 hover:text-blue-800">"More about my story →"</a>
                </p>
                <div class="flex space-x-6">
                    <a href="#projects" class="font-semibold border-b-2 border-slate-900">"View Work"</a>
                    <a href="/contact" class="text-slate-500">"Contact Me"</a>
                </div>
            </section>
            <section id="projects" class="py-24 bg-white">
                <div class="max-w-7xl mx-auto px-6">
                    <div class="flex justify-between items-end mb-16">
                        <div>
                            <h2 class="text-3xl font-bold mb-4">"Selected Work"</h2>
                            <p class="text-slate-500">"Projects exploring web and aerospace."</p>
                        </div>
                        <a href="/projects" class="hidden md:block text-blue-600">"View All →"</a>
                    </div>
                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                        {projects.into_iter().take(3).map(|p| view! { <ProjectCard project=p/> }).collect::<Vec<_>>()}
                    </div>
                </div>
            </section>
        </div>
    }
}
