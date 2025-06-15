#![allow(non_upper_case_globals)]
use crate::constants::social::GITHUB_URL;
use crate::constants::social::LINKEDIN_URL;
use js_sys::wasm_bindgen;
use leptos::prelude::*;
use leptos::web_sys;
use leptos_router::components::A;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2)]
    static location: web_sys::Location;
}

#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
#[allow(unused_unsafe)]
#[allow(clippy::no_effect_underscore_binding)]
pub fn Navigation() -> impl IntoView {
    let (mobile_menu_open, _set_mobile_menu_open) = signal(false);

    let (_is_mobile_menu_open, set_mobile_menu_open) = signal(false);

    let _toggle_mobile_menu = move |_: leptos::ev::MouseEvent| {
        set_mobile_menu_open.update(|open| *open = !*open);
    };

    let handle_books_click = move |_| {
        // Use web_sys Location API directly
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href("/books");
        }
    };

    view! {
        <header class="bg-white shadow-sm border-b border-gray-200">
            <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" aria-label="Top">
                <div class="flex items-center justify-between h-16">
                    // Logo/Brand
                    <div class="flex items-center">
                        <A href="/" attr:class="flex items-center">
                            <span class="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                                "Mike's Dev Blog"
                            </span>
                        </A>
                    </div>

                    // Desktop Navigation
                    <div class="hidden md:block">
                        <div class="ml-10 flex items-center space-x-8">
                            <A
                                href="/"
                                attr:class="text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors duration-200"
                                exact=true
                            >
                                "Home"
                            </A>
                            <A
                                href="/blog"
                                attr:class="text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors duration-200"
                            >
                                "Blog"
                            </A>
                            <button
                                class="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium transition-colors"
                                on:click=handle_books_click
                            >
                                "Books"
                            </button>
                            <A
                                href="/about"
                                attr:class="text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors duration-200"
                            >
                                "About"
                            </A>

                            // Social Links
                            <div class="flex items-center space-x-4">
                                <a
                                    href=GITHUB_URL
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="text-gray-400 hover:text-gray-600 transition-colors duration-200"
                                    aria-label="GitHub"
                                >
                                    <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
                                        <path
                                            fill-rule="evenodd"
                                            d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                </a>
                                <a
                                    href=LINKEDIN_URL
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="text-gray-400 hover:text-gray-600 transition-colors duration-200"
                                    aria-label="LinkedIn"
                                >
                                    <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452z" />
                                    </svg>
                                </a>
                            </div>
                        </div>
                    </div>

                    // Mobile menu button
                    <div class="md:hidden">
                        <button
                            type="button"
                            class="bg-white p-2 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-blue-500"
                            on:click=move |_| set_mobile_menu_open.update(|open| *open = !*open)
                        >
                            <svg
                                class="h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 6h16M4 12h16M4 18h16"
                                />
                            </svg>
                        </button>
                    </div>
                </div>

                // Mobile menu
                <div class=move || if mobile_menu_open.get() { "md:hidden" } else { "hidden" }>
                    <div class="px-2 pt-2 pb-3 space-y-1 border-t border-gray-200">
                        <A
                            href="/"
                            attr:class="text-gray-700 hover:text-blue-600 block px-3 py-2 text-base font-medium transition-colors duration-200"
                            exact=true
                            on:click=move |_| set_mobile_menu_open.set(false)
                        >
                            "Home"
                        </A>
                        <A
                            href="/blog"
                            attr:class="text-gray-700 hover:text-blue-600 block px-3 py-2 text-base font-medium transition-colors duration-200"
                            on:click=move |_| set_mobile_menu_open.set(false)
                        >
                            "Blog"
                        </A>
                        <a
                            href="/books"
                            class="text-gray-700 hover:text-blue-600 block px-3 py-2 text-base font-medium transition-colors duration-200"
                            on:click=move |e| {
                                set_mobile_menu_open.set(false);
                                handle_books_click(e);
                            }
                        >
                            // on:click=move |_| set_mobile_menu_open.set(false)
                            "Books"
                        </a>
                        <A
                            href="/about"
                            attr:class="text-gray-700 hover:text-blue-600 block px-3 py-2 text-base font-medium transition-colors duration-200"
                            on:click=move |_| set_mobile_menu_open.set(false)
                        >
                            "About"
                        </A>
                    </div>
                </div>
            </nav>
        </header>
    }
}
