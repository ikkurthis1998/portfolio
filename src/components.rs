use crate::data::*;
use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="bg-white shadow-lg sticky top-0 z-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between h-16">
                    <div class="flex items-center">
                        <a href="/" 
                           class="text-2xl font-bold text-blue-600 hover:text-blue-800 transition-colors">
                            "Sreemannarayana"
                        </a>
                    </div>
                    <div class="flex items-center space-x-8">
                        <a href="/" 
                           class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
                            "Home"
                        </a>
                        <a href="/projects" 
                           class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
                            "Projects"
                        </a>
                        <a href="/about" 
                           class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
                            "About"
                        </a>
                        <a href="/contact" 
                           class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
                            "Contact"
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let (portfolio_resource, set_portfolio_resource) = signal(Option::<Result<PortfolioData, String>>::None);
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            let res = fetch_portfolio_data().await;
            set_portfolio_resource.set(Some(res));
        });
    });
    
    view! {
        <Suspense fallback=move || view! { <div class="min-h-screen flex justify-center items-center">"Loading..."</div> }>
            {move || {
                match portfolio_resource.get() {
                    Some(Ok(data)) => {
                        let projects = data.projects;
                        view! {
                            <div class="min-h-screen bg-slate-50 font-sans text-slate-900">
            // Hero Section
            <section class="max-w-4xl mx-auto px-6 py-32 md:py-48">
                <div class="max-w-2xl">
                    <h1 class="text-5xl md:text-6xl font-bold tracking-tight mb-6 text-slate-900">
                        "Hi, I'm Sreemannarayana."
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-500 font-light mb-10 leading-relaxed">
                        "Full Stack Developer & Aerospace Engineer."
                    </p>
                    <p class="text-lg text-slate-600 mb-12 max-w-xl leading-relaxed">
                        "Passionate about building innovative solutions using modern technologies. 
                        I bridge the gap between complex engineering systems and elegant web applications."
                    </p>
                    
                    <div class="flex items-center space-x-6">
                        <a href="#projects" 
                           class="group flex items-center text-slate-900 font-semibold border-b-2 border-slate-900 pb-1 hover:text-blue-600 hover:border-blue-600 transition-all">
                            "View Work"
                            <span class="ml-2 group-hover:translate-x-1 transition-transform">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd" />
                                </svg>
                            </span>
                        </a>
                        
                        <a href="/contact" 
                           class="text-slate-500 hover:text-slate-900 font-medium transition-colors">
                            "Contact Me"
                        </a>
                    </div>
                </div>
            </section>

            // Featured Projects Section
            <section id="projects" class="py-24 bg-white">
                <div class="max-w-7xl mx-auto px-6">
                    <div class="flex justify-between items-end mb-16">
                        <div>
                            <h2 class="text-3xl font-bold text-slate-900 mb-4">"Selected Work"</h2>
                            <p class="text-slate-500">"A collection of projects exploring web and aerospace."</p>
                        </div>
                        <a href="/projects" 
                           class="hidden md:inline-flex items-center text-sm font-semibold text-blue-600 hover:text-blue-800 transition-colors">
                            "View All Projects →"
                        </a>
                    </div>
                    
                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                        {projects.into_iter().take(3).map(|project| view! {
                            <ProjectCard project=project/>
                        }).collect::<Vec<_>>()}
                    </div>
                    
                    <div class="mt-12 md:hidden text-center">
                         <a href="/projects" class="text-blue-600 font-semibold">"View all projects →"</a>
                    </div>
                </div>
            </section>


                            </div>
                        }.into_any()
                    },
                    Some(Err(e)) => view! { 
                        <div class="min-h-screen flex justify-center items-center text-red-600">
                            {format!("Error loading data: {}. Make sure data.json is accessible.", e)}
                        </div> 
                    }.into_any(),
                    None => view! { <div class="min-h-screen flex justify-center items-center">"Loading..."</div> }.into_any(),
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <a href={project.url} target="_blank"
           class="group block h-full bg-white border border-slate-100 rounded-2xl p-8 hover:shadow-xl hover:-translate-y-1 transition-all duration-300">
           
            <div class="flex justify-between items-start mb-6">
                <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-slate-50 text-slate-600 group-hover:bg-blue-50 group-hover:text-blue-600 transition-colors">
                    {project.language}
                </span>
                <span class="text-slate-400 text-sm flex items-center gap-1 group-hover:text-yellow-500 transition-colors">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 fill-current" viewBox="0 0 20 20" fill="currentColor">
                        <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                    </svg>
                    {project.stars}
                </span>
            </div>
            
            <h3 class="text-xl font-bold text-slate-900 mb-3 group-hover:text-blue-600 transition-colors">
                {project.name}
            </h3>
            
            <p class="text-slate-500 text-sm leading-relaxed mb-6">
                {project.description}
            </p>
            
            <div class="flex flex-wrap gap-2 mt-auto">
                {project.topics.into_iter().take(3).map(|topic| view! {
                    <span class="text-xs text-slate-400">"#" {topic}</span>
                }).collect::<Vec<_>>()}
            </div>
        </a>
    }
}



#[component]
pub fn ProjectsPage() -> impl IntoView {
    let (portfolio_resource, set_portfolio_resource) = signal(Option::<Result<PortfolioData, String>>::None);
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            let res = fetch_portfolio_data().await;
            set_portfolio_resource.set(Some(res));
        });
    });

    view! {
        <Suspense fallback=move || view! { <div class="min-h-screen flex justify-center items-center">"Loading..."</div> }>
            {move || {
                match portfolio_resource.get() {
                    Some(Ok(data)) => {
                        let projects = data.projects;
                        view! {
                            <div class="min-h-screen bg-gray-50">
                                <div class="py-16">
                                    <div class="max-w-6xl mx-auto px-4">
                                        <h1 class="text-4xl font-bold text-center mb-12 text-gray-800">
                                            "All Projects"
                                        </h1>
                                        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                                            {projects.into_iter().map(|project| view! {
                                                <ProjectCard project=project/>
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    },
                    Some(Err(e)) => view! { <div class="text-red-600 text-center py-20">{format!("Error: {}", e)}</div> }.into_any(),
                    None => view! { <div class="text-center py-20">"Loading..."</div> }.into_any(),
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn AboutPage() -> impl IntoView {
    let (portfolio_resource, set_portfolio_resource) = signal(Option::<Result<PortfolioData, String>>::None);
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            let res = fetch_portfolio_data().await;
            set_portfolio_resource.set(Some(res));
        });
    });

    view! {
        <Suspense fallback=move || view! { <div class="min-h-screen flex justify-center items-center">"Loading..."</div> }>
            {move || {
                match portfolio_resource.get() {
                    Some(Ok(data)) => {
                        let experiences = data.experiences;
                        view! {
                            <div class="min-h-screen bg-white">
                                <div class="py-16">
                                    <div class="max-w-4xl mx-auto px-4">
                                        <h1 class="text-4xl font-bold text-center mb-12 text-gray-800">
                                            "About Me"
                                        </h1>

                                        <div class="mb-12">
                                            <p class="text-lg text-gray-700 leading-relaxed mb-6">
                                                "I am a passionate full-stack developer with a strong background in aerospace engineering.
                                                My journey began with numerical analysis and optimization algorithms in aerospace, and has
                                                evolved to include modern web development technologies."
                                            </p>
                                            <p class="text-lg text-gray-700 leading-relaxed">
                                                "I specialize in building high-performance web applications using Rust, TypeScript, and Python.
                                                My aerospace background gives me a unique perspective on problem-solving and systems thinking."
                                            </p>
                                        </div>

                                        <div class="mb-12">
                                            <h2 class="text-2xl font-bold mb-6 text-gray-800">"Experience"</h2>
                                            <div class="border-l-2 border-slate-200 ml-3 space-y-0">
                                                {experiences.into_iter().map(|exp| view! {
                                                    <ExperienceCard experience=exp/>
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>

                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    },
                    Some(Err(e)) => view! { <div class="text-red-600 text-center py-20">{format!("Error: {}", e)}</div> }.into_any(),
                    None => view! { <div class="text-center py-20">"Loading..."</div> }.into_any(),
                }
            }}
        </Suspense>
    }
}

#[component]
pub fn ExperienceCard(experience: Experience) -> impl IntoView {
    view! {
        <div class="relative pl-8 pb-12 last:pb-0 group">
            // Timeline Dot
            <div class="absolute -left-[9px] top-0 w-5 h-5 rounded-full bg-blue-600 border-4 border-white shadow-sm transition-transform group-hover:scale-125"></div>
            
            // Content Card
            <div class="bg-slate-50 rounded-2xl p-6 hover:bg-white hover:shadow-lg transition-all border border-transparent hover:border-slate-100">
                <div class="flex flex-col md:flex-row md:justify-between md:items-start mb-4 gap-2">
                    <div>
                        <h3 class="text-xl font-bold text-slate-800">{experience.title}</h3>
                        <p class="text-blue-600 font-medium text-lg">{experience.company}</p>
                    </div>
                    <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-50 text-blue-700 whitespace-nowrap">
                        {experience.duration}
                    </span>
                </div>
                
                <p class="text-slate-600 leading-relaxed mb-6">
                    {experience.description}
                </p>
                
                <div class="flex flex-wrap gap-2">
                    {experience.technologies.into_iter().map(|tech| view! {
                        <span class="px-3 py-1 bg-white text-slate-600 text-xs font-semibold rounded-full border border-slate-200 shadow-sm">
                            {tech}
                        </span>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <div class="py-16">
                <div class="max-w-4xl mx-auto px-4">
                    <h1 class="text-4xl font-bold text-center mb-12 text-gray-800">
                        "Get In Touch"
                    </h1>

                    <div class="bg-white rounded-lg shadow-md p-8">
                        <div class="grid md:grid-cols-2 gap-8">
                            <div>
                                <h2 class="text-2xl font-bold mb-6 text-gray-800">"Contact Information"</h2>
                                <div class="space-y-4">
                                    <div class="flex items-center">
                                        <span class="text-slate-600 mr-3">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
                                                <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd" />
                                            </svg>
                                        </span>
                                        <a href="https://github.com/ikkurthis1998" 
                                           class="text-blue-600 hover:text-blue-800">
                                            "github.com/ikkurthis1998"
                                        </a>
                                    </div>
                                    <div class="flex items-center">
                                        <span class="text-slate-600 mr-3">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.84 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/>
                                            </svg>
                                        </span>
                                        <a href="https://twitter.com/isree1998" 
                                           class="text-blue-600 hover:text-blue-800">
                                            "@isree1998"
                                        </a>
                                    </div>
                                </div>
                            </div>

                            <div>
                                <h2 class="text-2xl font-bold mb-6 text-gray-800">"Let's Connect"</h2>
                                <p class="text-gray-700 mb-6">
                                    "I'm always interested in new opportunities and collaborations.
                                    Feel free to reach out if you'd like to discuss a project or just say hello!"
                                </p>
                                <div class="flex space-x-4">
                                    <a href="https://github.com/ikkurthis1998" target="_blank"
                                       class="border border-blue-600 text-blue-600 px-6 py-3 rounded-lg font-semibold hover:bg-blue-600 hover:text-white transition-colors">
                                        "Follow on GitHub"
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
