use leptos::prelude::*;
use crate::data::get_portfolio_data;
use super::projects::ProjectCard;

#[component]
pub fn HomePage() -> impl IntoView {
    let data = get_portfolio_data();
    let projects = data.projects;
    
    view! {
        <div class="min-h-screen">
            // Hero Section
            <section class="relative bg-gradient-to-br from-slate-50 via-white to-blue-50">
                <div class="max-w-6xl mx-auto px-6 py-28 md:py-40">
                    <div class="flex flex-col md:flex-row items-center gap-12 md:gap-20">
                        <div class="flex-1 text-center md:text-left">
                            <p class="text-blue-600 font-medium mb-4 tracking-wide uppercase text-sm">"Software Engineer • Aerospace Background"</p>
                            <h1 class="text-5xl md:text-6xl lg:text-7xl font-bold mb-6 text-slate-900 leading-tight">
                                "Hi, I'm "<span class="bg-gradient-to-r from-blue-600 to-indigo-600 bg-clip-text text-transparent">"Sreemannarayana"</span>"."
                            </h1>
                            <p class="text-lg md:text-xl text-slate-600 mb-3 max-w-lg leading-relaxed">
                                "I build things for the web with a background in systems that demand precision. Clean code, thoughtful design, real impact."
                            </p>
                            <a href="/about" class="inline-block text-blue-600 hover:text-blue-800 mb-10 font-medium">"More about me →"</a>
                            <div class="flex flex-wrap justify-center md:justify-start gap-4">
                                <a href="#projects" class="px-8 py-4 bg-slate-900 text-white rounded-full hover:bg-slate-800 transition-all font-medium shadow-lg hover:shadow-xl">
                                    "View My Work"
                                </a>
                                <a href="#contact" class="px-8 py-4 border-2 border-slate-200 text-slate-700 rounded-full hover:border-slate-300 transition-all font-medium">
                                    "Get In Touch"
                                </a>
                            </div>
                        </div>
                        <div class="relative flex-shrink-0">
                            <div class="absolute inset-0 bg-gradient-to-br from-blue-400 to-indigo-500 rounded-full blur-2xl opacity-20 scale-110"></div>
                            <img src="/assets/IMG_4872.JPG" alt="Sreemannarayana" class="relative w-72 h-72 md:w-96 md:h-96 rounded-full object-cover object-top ring-4 ring-white shadow-2xl"/>
                        </div>
                    </div>
                </div>
            </section>
            
            // Projects Section
            <section id="projects" class="py-28 bg-white">
                <div class="max-w-7xl mx-auto px-6">
                    <div class="flex flex-col md:flex-row justify-between items-start md:items-end gap-4 mb-16">
                        <div>
                            <p class="text-blue-600 font-medium mb-2 text-sm uppercase tracking-wide">"Portfolio"</p>
                            <h2 class="text-4xl font-bold text-slate-900">"Selected Work"</h2>
                        </div>
                        <a href="/projects" class="text-slate-500 hover:text-slate-900">"View all →"</a>
                    </div>
                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                        {projects.into_iter().take(3).map(|p| view! { <ProjectCard project=p/> }).collect::<Vec<_>>()}
                    </div>
                </div>
            </section>
            
            // Contact Section
            <section id="contact" class="py-28 bg-slate-50">
                <div class="max-w-2xl mx-auto px-6 text-center">
                    <p class="text-blue-600 font-medium mb-2 text-sm uppercase tracking-wide">"Contact"</p>
                    <h2 class="text-4xl font-bold text-slate-900 mb-6">"Let's Connect"</h2>
                    <p class="text-lg text-slate-600 mb-10">"Open to new opportunities and collaborations."</p>
                    <div class="flex justify-center gap-6">
                        <a href="https://github.com/ikkurthis1998" target="_blank" class="p-4 bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7 text-slate-700 hover:text-slate-900" fill="currentColor" viewBox="0 0 24 24">
                                <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd"/>
                            </svg>
                        </a>
                        <a href="https://www.linkedin.com/in/ikkurthis1998/" target="_blank" class="p-4 bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7 text-blue-600" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/>
                            </svg>
                        </a>
                        <a href="https://twitter.com/isree1998" target="_blank" class="p-4 bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7 text-sky-500" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.84 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/>
                            </svg>
                        </a>
                    </div>
                </div>
            </section>
        </div>
    }
}
