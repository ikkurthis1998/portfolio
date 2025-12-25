use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="fixed w-full top-0 z-50 print:hidden">
            // Checkbox must be at top level for peer to work
            <input type="checkbox" id="menu-toggle" class="hidden peer"/>
            
            <div class="backdrop-blur-md bg-white/80 border-b border-slate-200/50">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between h-20 items-center">
                        <a href="/" class="flex items-center group">
                            <img src="/assets/logo.png" alt="Sree's Logo" class="h-[76px] w-[76px] object-contain group-hover:scale-105 transition-transform duration-300"/>
                        </a>
                        <div class="hidden md:flex items-center space-x-10">
                            <a href="/" class="text-sm font-medium text-slate-600 hover:text-blue-600">"Home"</a>
                            <a href="/projects" class="text-sm font-medium text-slate-600 hover:text-blue-600">"Projects"</a>
                            <a href="/about" class="text-sm font-medium text-slate-600 hover:text-blue-600">"About"</a>
                            <a href="/resume" class="text-sm font-medium text-slate-600 hover:text-blue-600">"Resume"</a>
                        </div>
                        <label for="menu-toggle" class="md:hidden p-2 cursor-pointer">
                            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </label>
                    </div>
                </div>
            </div>
            
            // Mobile menu - sibling of input, so peer-checked works
            <div class="md:hidden hidden peer-checked:block absolute w-full bg-white shadow-lg">
                <div class="px-4 py-4 space-y-2">
                    <a href="/" class="block px-3 py-2 text-slate-900 hover:bg-slate-50">"Home"</a>
                    <a href="/projects" class="block px-3 py-2 text-slate-600 hover:bg-slate-50">"Projects"</a>
                    <a href="/about" class="block px-3 py-2 text-slate-600 hover:bg-slate-50">"About"</a>
                    <a href="/resume" class="block px-3 py-2 text-slate-600 hover:bg-slate-50">"Resume"</a>
                </div>
            </div>
        </nav>
    }
}
