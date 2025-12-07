use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let (is_menu_open, set_is_menu_open) = signal(false);

    view! {
        <nav class="fixed w-full top-0 z-50 backdrop-blur-md bg-white/80 border-b border-slate-200/50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between h-20 items-center">
                    <a href="/" class="text-xl font-bold text-slate-900 hover:text-blue-600">"isree.dev"</a>
                    <div class="hidden md:flex items-center space-x-10">
                        <a href="/" class="text-sm font-medium text-slate-600 hover:text-blue-600">"Home"</a>
                        <a href="/projects" class="text-sm font-medium text-slate-600 hover:text-blue-600">"Projects"</a>
                        <a href="/about" class="text-sm font-medium text-slate-600 hover:text-blue-600">"About"</a>
                        <a href="/contact" class="text-sm font-medium text-slate-600 hover:text-blue-600">"Contact"</a>
                    </div>
                    <button on:click=move |_| set_is_menu_open.update(|o| *o = !*o) class="md:hidden p-2">
                        <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            {move || if is_menu_open.get() {
                                view! { <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /> }
                            } else {
                                view! { <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" /> }
                            }}
                        </svg>
                    </button>
                </div>
            </div>
            <div class={move || format!("md:hidden absolute w-full bg-white/95 backdrop-blur-xl shadow-lg {}", if is_menu_open.get() { "block" } else { "hidden" })}>
                <div class="px-4 py-4 space-y-2">
                    <a href="/" class="block px-3 py-2 text-slate-900">"Home"</a>
                    <a href="/projects" class="block px-3 py-2 text-slate-600">"Projects"</a>
                    <a href="/about" class="block px-3 py-2 text-slate-600">"About"</a>
                    <a href="/contact" class="block px-3 py-2 text-slate-600">"Contact"</a>
                </div>
            </div>
        </nav>
    }
}
