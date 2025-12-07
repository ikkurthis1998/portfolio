use leptos::prelude::*;
use crate::data::{get_portfolio_data, Project};

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let data = get_portfolio_data();
    let projects = data.projects;
    
    view! {
        <div class="min-h-screen bg-gray-50 pt-28 pb-16">
            <div class="max-w-6xl mx-auto px-4">
                <h1 class="text-4xl font-bold text-center mb-12">"All Projects"</h1>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {projects.into_iter().map(|p| view! { <ProjectCard project=p/> }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <a href={project.url} target="_blank" class="group block bg-white border rounded-2xl p-8 hover:shadow-xl transition-all">
            <div class="flex justify-between mb-6">
                <span class="px-3 py-1 rounded-full text-xs bg-slate-50">{project.language}</span>
                <span class="text-slate-400 text-sm flex items-center gap-1">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-amber-400" fill="currentColor" viewBox="0 0 20 20">
                        <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                    </svg>
                    {project.stars}
                </span>
            </div>
            <h3 class="text-xl font-bold mb-3 group-hover:text-blue-600">{project.name}</h3>
            <p class="text-slate-500 text-sm mb-6">{project.description}</p>
            <div class="flex flex-wrap gap-2">
                {project.topics.into_iter().take(3).map(|t| view! { <span class="text-xs text-slate-400">"#" {t}</span> }).collect::<Vec<_>>()}
            </div>
        </a>
    }
}
