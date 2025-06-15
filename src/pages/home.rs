use crate::components::latest_blog_posts::LatestBlogPosts;
use crate::components::tech_cards::TechnologyCards;
use crate::constants::social::{GITHUB_URL, LINKEDIN_URL};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn Home() -> impl IntoView {
    view! {
        // Set the page title
        <Title text="Mikko Leppänen - Software Engineer & Rust Developer" />

        // Hero Section
        <section class="bg-gradient-to-b from-blue-50 to-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16 md:py-24">
                <div class="lg:grid lg:grid-cols-12 lg:gap-8 items-center">
                    <div class="lg:col-span-7">
                        <div class="inline-flex items-center justify-center px-4 py-2 bg-blue-100 rounded-full mb-6">
                            <span class="text-blue-800 font-medium text-sm">
                                "Precision Engineering: Crafting Software with Rust and Python"
                            </span>
                        </div>
                        <h1 class="text-4xl font-extrabold tracking-tight text-gray-900 sm:text-5xl md:text-6xl">
                            <span class="block mb-2">"Hi, I'm Mikko Leppänen"</span>
                            <span class="block bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                                "Crafting Digital Experiences"
                            </span>
                        </h1>
                        <p class="mt-6 text-xl text-gray-500 max-w-3xl leading-relaxed">
                            "Crafting robust software with Rust and Python. Lifelong learner, clean code advocate, open source contributor. Always exploring what's next in tech."
                        </p>
                        <div class="mt-8 flex flex-wrap gap-4">
                            <A
                                href="/blog"
                                attr:class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 transition-colors duration-200"
                            >
                                "Read My Blog"
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-5 w-5 ml-2"
                                    viewBox="0 0 20 20"
                                    fill="currentColor"
                                >
                                    <path
                                        fill-rule="evenodd"
                                        d="M10.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L12.586 11H5a1 1 0 110-2h7.586l-2.293-2.293a1 1 0 010-1.414z"
                                        clip-rule="evenodd"
                                    />
                                </svg>
                            </A>
                            <A
                                href="/about"
                                attr:class="inline-flex items-center px-6 py-3 border border-gray-300 text-base font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 transition-colors duration-200"
                            >
                                "About Me"
                            </A>
                        </div>
                    </div>
                    <div class="mt-12 lg:mt-0 lg:col-span-5">
                        <div class="bg-gradient-to-r from-blue-500 to-purple-600 p-1 rounded-2xl shadow-xl">
                            <div class="bg-white p-4 rounded-xl">
                                <div class="aspect-w-5 aspect-h-4 rounded-lg overflow-hidden">
                                    <div class="bg-gradient-to-br from-blue-100 to-purple-100 h-full w-full flex items-center justify-center">
                                        <svg
                                            class="h-32 w-32 text-blue-500"
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke="currentColor"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="1"
                                                d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
                                            />
                                        </svg>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <LatestBlogPosts />

        <TechnologyCards />

        // Connect Section
        <section class="bg-white py-16">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="bg-gradient-to-r from-blue-600 to-purple-600 rounded-2xl shadow-xl overflow-hidden">
                    <div class="px-6 py-12 sm:px-12 lg:py-16 lg:pr-0 xl:py-20 xl:px-20">
                        <div class="lg:self-center">
                            <h2 class="text-3xl font-extrabold text-white sm:text-4xl">
                                <span class="block">"Let's connect"</span>
                            </h2>
                            <p class="mt-4 text-lg leading-6 text-blue-100">
                                "Follow me on GitHub and LinkedIn to see my latest projects and connect professionally."
                            </p>
                            <div class="mt-8 flex gap-4">
                                <a
                                    href=GITHUB_URL
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-blue-600 bg-white hover:bg-blue-50 transition-colors duration-200"
                                >
                                    <svg
                                        class="h-5 w-5 mr-2"
                                        fill="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                    "GitHub"
                                </a>
                                <a
                                    href=LINKEDIN_URL
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-blue-600 bg-white hover:bg-blue-50 transition-colors duration-200"
                                >
                                    <svg
                                        class="h-5 w-5 mr-2"
                                        fill="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452z" />
                                    </svg>
                                    "LinkedIn"
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
