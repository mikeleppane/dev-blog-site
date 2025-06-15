use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct TechCard {
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub color_class: &'static str,
    pub url: &'static str,
}

pub const TECH_STACK: &[TechCard] = &[
    TechCard {
        name: "Rust",
        description: "Systems programming language focused on safety, speed, and concurrency",
        icon: "🦀",
        color_class: "bg-orange-100 text-orange-800 border-orange-200",
        url: "https://www.rust-lang.org/",
    },
    TechCard {
        name: "Python",
        description: "Versatile programming language for web development, data science, and automation",
        icon: "🐍",
        color_class: "bg-blue-100 text-blue-800 border-blue-200",
        url: "https://python.org/",
    },
    TechCard {
        name: "Mojo",
        description: "High-performance programming language for AI and systems programming",
        icon: "🔥",
        color_class: "bg-red-100 text-red-800 border-red-200",
        url: "https://www.modular.com/mojo",
    },
    TechCard {
        name: "Azure",
        description: "Microsoft's cloud computing platform for building, deploying, and managing applications",
        icon: "☁️",
        color_class: "bg-cyan-100 text-cyan-800 border-cyan-200",
        url: "https://azure.microsoft.com/",
    },
    TechCard {
        name: "Cosmos DB",
        description: "Globally distributed, multi-model database service for modern applications",
        icon: "🌍",
        color_class: "bg-purple-100 text-purple-800 border-purple-200",
        url: "https://azure.microsoft.com/en-us/products/cosmos-db/",
    },
    TechCard {
        name: "SurrealDB",
        description: "Multi-model database with SQL querying, real-time subscriptions, and advanced security",
        icon: "🗄️",
        color_class: "bg-gray-100 text-gray-800 border-gray-200",
        url: "https://surrealdb.com/",
    },
    TechCard {
        name: "Docker",
        description: "Containerization platform for building, shipping, and running applications",
        icon: "🐳",
        color_class: "bg-blue-100 text-blue-800 border-blue-200",
        url: "https://www.docker.com/",
    },
    TechCard {
        name: "FastAPI",
        description: "Modern, fast web framework for building APIs with Python based on standard type hints",
        icon: "⚡",
        color_class: "bg-green-100 text-green-800 border-green-200",
        url: "https://fastapi.tiangolo.com/",
    },
    TechCard {
        name: "Leptos",
        description: "Full-stack web framework for Rust with fine-grained reactivity and server-side rendering",
        icon: "🌟",
        color_class: "bg-yellow-100 text-yellow-800 border-yellow-200",
        url: "https://leptos.dev/",
    },
];

#[component]
#[allow(clippy::must_use_candidate)]
pub fn TechnologyCards() -> impl IntoView {
    view! {
        <section class="py-16 bg-gray-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center mb-12">
                    <h2 class="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
                        "Technology Stack"
                    </h2>
                    <p class="text-xl text-gray-600 max-w-3xl mx-auto">
                        "The tools and technologies I use to build modern, scalable, and performant applications"
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {TECH_STACK
                        .iter()
                        .map(|tech| {
                            view! { <TechCardComponent tech=tech.clone() /> }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}

#[component]
fn TechCardComponent(tech: TechCard) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 hover:shadow-lg transition-all duration-300 transform hover:-translate-y-1 group">
            <div class="flex items-start space-x-4">
                <div class=format!(
                    "flex-shrink-0 w-12 h-12 rounded-lg flex items-center justify-center text-2xl border-2 {}",
                    tech.color_class,
                )>{tech.icon}</div>
                <div class="flex-1 min-w-0">
                    <h3 class="text-lg font-semibold text-gray-900 mb-2 group-hover:text-blue-600 transition-colors">
                        {tech.name}
                    </h3>
                    <p class="text-sm text-gray-600 leading-relaxed mb-4">{tech.description}</p>
                    <a
                        href=tech.url
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-flex items-center text-sm font-medium text-blue-600 hover:text-blue-800 transition-colors"
                    >
                        "Learn more"
                        <svg
                            class="ml-1 w-4 h-4 group-hover:translate-x-1 transition-transform"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                            />
                        </svg>
                    </a>
                </div>
            </div>
        </div>
    }
}
