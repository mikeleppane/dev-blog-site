// filepath: /home/mikko/dev/mike-dev-blog/src/pages/blog.rs
use crate::components::blog_list::BlogList;
use leptos::prelude::*;
use leptos_meta::Title;
#[component]
#[allow(clippy::must_use_candidate)]
pub fn BlogPage() -> impl IntoView {
    view! {
        <Title text="Blog - Mike's Dev Blog" />

        <div class="bg-gradient-to-b from-blue-50 to-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                <div class="text-center">
                    <h1 class="text-4xl font-extrabold tracking-tight text-gray-900 sm:text-5xl md:text-6xl">
                        <span class="block">"My Blog"</span>
                        <span class="block bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                            "Thoughts on Software Development"
                        </span>
                    </h1>
                    <p class="max-w-xl mt-5 mx-auto text-xl text-gray-500">
                        "Exploring ideas, technologies, and best practices in modern software development."
                    </p>
                </div>
            </div>
        </div>

        <BlogList />
    }
}
