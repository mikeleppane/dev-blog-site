use crate::constants::social::GITHUB_URL;
use crate::constants::social::LINKEDIN_URL;
use leptos::leptos_dom::logging;
use leptos::prelude::*;
use leptos_meta::Title;
#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn AboutPage() -> impl IntoView {
    on_cleanup(|| {
        logging::console_log("AboutPage component cleaned up");
    });
    view! {
        // Set the page title
        <Title text="About Me - Mike's Dev Blog" />

        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <div class="max-w-3xl mx-auto">
                <div class="bg-gradient-to-r from-blue-500 to-purple-600 p-1 rounded-2xl shadow-xl">
                    <div class="bg-white p-8 sm:p-12 rounded-xl">
                        <div class="text-center mb-12">
                            <div class="inline-flex items-center justify-center p-2 bg-blue-100 rounded-full mb-6">
                                <span class="text-blue-800 font-medium px-4 py-2">"About Me"</span>
                            </div>
                            <h1 class="text-4xl font-bold text-gray-900 sm:text-5xl">
                                "Hi, I'm Mike Leppänen"
                            </h1>
                            <p class="mt-4 text-xl text-gray-600">
                                "Software Engineer & Code Craftsman & Dad 👨‍💻👨‍👧‍👦"
                            </p>
                        </div>

                        <div class="prose prose-lg max-w-none">
                            <p>
                                "I am deeply passionate about solving challenging problems and committed to lifelong learning, staying current with emerging technologies. Since 2008, Python has been my primary programming language, extensively applied in software development and test automation. In recent years, I've also embraced Rust, enhancing my expertise in memory management, concurrency, and system-level programming. Currently, I'm exploring Mojo, leveraging its innovative features to tackle complex programming tasks."
                            </p>
                            <br />
                            <p>
                                "I advocate for Behavior-Driven Development (BDD) and Domain-Driven Design (DDD), methodologies that enable me to deliver software closely aligned with user needs and business objectives. Clean, elegant, and maintainable code is integral to my approach, promoting productivity and sustainable success."
                            </p>
                            <br />
                            <p>
                                "Driven by curiosity, I'm always eager to embrace new challenges and continually innovate."
                            </p>

                            <h2>"What I'm Working On"</h2>
                            <p>
                                "Currently, I'm focused on building high-performance web applications using Rust and Leptos.
                                I'm also exploring how to leverage Azure services for scalable backend solutions."
                            </p>

                            <h2>"Connect With Me"</h2>
                            <p>
                                "I'm always open to interesting conversations, collaboration opportunities, or just chatting about tech.
                                Feel free to reach out to me on GitHub or LinkedIn!"
                            </p>

                            // Social Links with updated layout
                            <div class="flex flex-col items-center justify-center gap-6 mb-12">
                                // Top row: GitHub and LinkedIn
                                <div class="flex flex-col sm:flex-row items-center justify-center gap-4">
                                    <a
                                        href=GITHUB_URL
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="inline-flex items-center px-6 py-3 border border-gray-300 text-base font-medium rounded-lg text-gray-700 bg-white hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md group"
                                    >
                                        <svg
                                            class="w-5 h-5 mr-3 group-hover:scale-110 transition-transform"
                                            fill="currentColor"
                                            viewBox="0 0 20 20"
                                        >
                                            <path
                                                fill-rule="evenodd"
                                                d="M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z"
                                                clip-rule="evenodd"
                                            />
                                        </svg>
                                        "GitHub"
                                    </a>

                                    <a
                                        href=LINKEDIN_URL
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="inline-flex items-center px-6 py-3 border border-gray-300 text-base font-medium rounded-lg text-gray-700 bg-white hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md group"
                                    >
                                        <svg
                                            class="w-5 h-5 mr-3 group-hover:scale-110 transition-transform"
                                            fill="currentColor"
                                            viewBox="0 0 20 20"
                                        >
                                            <path d="M16.338 16.338H13.67V12.16c0-.995-.017-2.277-1.387-2.277-1.39 0-1.601 1.086-1.601 2.207v4.248H8.014v-8.59h2.559v1.174h.037c.356-.675 1.227-1.387 2.526-1.387 2.703 0 3.203 1.778 3.203 4.092v4.711zM5.005 6.575a1.548 1.548 0 11-.003-3.096 1.548 1.548 0 01.003 3.096zm-1.337 9.763H6.34v-8.59H3.667v8.59zM17.668 1H2.328C1.595 1 1 1.581 1 2.298v15.403C1 18.418 1.595 19 2.328 19h15.34c.734 0 1.332-.582 1.332-1.299V2.298C19 1.581 18.402 1 17.668 1z" />
                                        </svg>
                                        "LinkedIn"
                                    </a>
                                </div>

                                // Bottom row: Resume button
                                <div class="flex justify-center">
                                    <a
                                        href="https://mikkoleppanenresume.net"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="inline-flex items-center px-8 py-4 border border-transparent text-lg font-semibold rounded-lg text-white bg-blue-600 hover:bg-blue-700 transition-all duration-200 shadow-lg hover:shadow-xl group transform hover:scale-105"
                                    >
                                        <svg
                                            class="w-6 h-6 mr-3 group-hover:scale-110 transition-transform"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                            />
                                        </svg>
                                        "View My Resume"
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
