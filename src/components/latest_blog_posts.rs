#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]

use leptos::prelude::*;
use leptos_router::components::A;

use crate::api::blogs::get_latest_blogs_server;
use crate::models::blog::BlogPost;
use crate::utils::date_formatter::format_date_readable;

#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn LatestBlogPosts() -> impl IntoView {
    // Action to fetch latest blog posts
    let fetch_latest_blogs_action =
        Action::new(|(): &()| async move { get_latest_blogs_server(3).await });

    // Trigger initial fetch
    Effect::new(move |_| {
        fetch_latest_blogs_action.dispatch(());
    });

    view! {
        <section class="py-16 bg-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center mb-12">
                    <h2 class="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
                        "Latest Articles"
                    </h2>
                    <p class="text-xl text-gray-600 max-w-2xl mx-auto">
                        "Fresh insights on modern software development, architecture patterns, and emerging technologies"
                    </p>
                </div>

                // Loading, Error, and Success States
                {move || {
                    if fetch_latest_blogs_action.pending().get() {
                        view! {
                            <div class="flex justify-center items-center py-12">
                                <div class="inline-flex items-center">
                                    <svg
                                        class="animate-spin -ml-1 mr-3 h-8 w-8 text-blue-600"
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                    >
                                        <circle
                                            class="opacity-25"
                                            cx="12"
                                            cy="12"
                                            r="10"
                                            stroke="currentColor"
                                            stroke-width="4"
                                        ></circle>
                                        <path
                                            class="opacity-75"
                                            fill="currentColor"
                                            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                        ></path>
                                    </svg>
                                    <span class="text-lg text-gray-600">
                                        "Loading latest articles..."
                                    </span>
                                </div>
                            </div>
                        }
                            .into_any()
                    } else if let Some(result) = fetch_latest_blogs_action.value().get() {
                        match result {
                            Ok(blogs) => {
                                if blogs.is_empty() {
                                    view! {
                                        <div class="text-center py-12">
                                            <div class="max-w-md mx-auto">
                                                <svg
                                                    class="mx-auto h-12 w-12 text-gray-400"
                                                    fill="none"
                                                    viewBox="0 0 24 24"
                                                    stroke="currentColor"
                                                >
                                                    <path
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        stroke-width="2"
                                                        d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                                    />
                                                </svg>
                                                <h3 class="mt-2 text-xl font-medium text-gray-900">
                                                    "No articles yet"
                                                </h3>
                                                <p class="mt-1 text-gray-500">
                                                    "Stay tuned for upcoming content!"
                                                </p>
                                            </div>
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <div class="grid gap-8 md:grid-cols-2 lg:grid-cols-3">
                                            {blogs
                                                .into_iter()
                                                .map(|blog| {
                                                    view! { <BlogPostCard blog=blog /> }
                                                })
                                                .collect_view()}
                                        </div>

                                        // View All Articles Button
                                        <div class="text-center mt-12">
                                            <A href="/blog">
                                                <span class="inline-flex items-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors duration-200 shadow-sm hover:shadow-md transform hover:-translate-y-0.5">
                                                    "View All Articles"
                                                    <svg
                                                        class="ml-2 -mr-1 w-5 h-5"
                                                        fill="currentColor"
                                                        viewBox="0 0 20 20"
                                                    >
                                                        <path
                                                            fill-rule="evenodd"
                                                            d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
                                                            clip-rule="evenodd"
                                                        />
                                                    </svg>
                                                </span>
                                            </A>
                                        </div>
                                    }
                                        .into_any()
                                }
                            }
                            Err(err) => {
                                view! {
                                    <div class="text-center py-12">
                                        <div class="bg-red-50 border border-red-200 rounded-lg p-6 max-w-md mx-auto">
                                            <div class="flex items-center">
                                                <svg
                                                    class="h-6 w-6 text-red-400 mr-3"
                                                    fill="none"
                                                    viewBox="0 0 24 24"
                                                    stroke="currentColor"
                                                >
                                                    <path
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        stroke-width="2"
                                                        d="M12 8v4m0 4v.01M12 4a8 8 0 100 16 8 8 0 000-16z"
                                                    />
                                                </svg>
                                                <div class="text-left">
                                                    <h3 class="text-sm font-medium text-red-800">
                                                        "Unable to load articles"
                                                    </h3>
                                                    <p class="mt-1 text-sm text-red-600">
                                                        {format!("Error: {err}")}
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="mt-4">
                                                <button
                                                    class="text-sm font-medium text-red-600 hover:text-red-500 transition-colors"
                                                    on:click=move |_| {
                                                        fetch_latest_blogs_action.dispatch(());
                                                    }
                                                >
                                                    "Try again"
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                }
                                    .into_any()
                            }
                        }
                    } else {
                        view! {
                            <div class="text-center py-12">
                                <p class="text-lg text-gray-600">"Initializing..."</p>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
        </section>
    }
}

#[component]
fn BlogPostCard(blog: BlogPost) -> impl IntoView {
    let blog_clone = blog.clone();
    let formatted_date = format_date_readable(blog.created_at);
    let reading_time = estimate_reading_time(&blog.content);

    view! {
        <article class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden hover:shadow-lg transition-all duration-300 transform hover:-translate-y-1 group">
            // Blog Image
            {blog_clone
                .image_url
                .clone()
                .map(|url| {
                    view! {
                        <div class="aspect-w-16 aspect-h-9 bg-gray-200">
                            <img
                                src=url.clone()
                                alt=format!("Cover image for {}", blog_clone.title.clone())
                                class="w-full h-48 object-cover group-hover:scale-105 transition-transform duration-300"
                                loading="lazy"
                            />
                        </div>
                    }
                })
                .unwrap_or_else(|| {
                    view! {
                        <div class="h-48 bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center">
                            <div class="text-center text-white">
                                <svg
                                    class="mx-auto h-12 w-12 mb-2"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                    />
                                </svg>
                                <p class="text-sm font-medium">"Tech Article"</p>
                            </div>
                        </div>
                    }
                })} <div class="p-6">
                // Blog metadata
                <div class="flex items-center text-sm text-gray-500 mb-3">
                    <time datetime=blog_clone
                        .created_at
                        .clone()
                        .to_rfc3339()>{formatted_date}</time>
                    <span class="mx-2">"•"</span>
                    <span>{reading_time}" min read"</span>
                </div>

                // Blog title
                <h3 class="text-xl font-semibold text-gray-900 mb-3 group-hover:text-blue-600 transition-colors line-clamp-2">
                    <A href=format!("/blog/{}", blog_clone.id.clone())>
                        <span class="hover:no-underline">{blog_clone.title.clone()}</span>
                    </A>
                </h3>

                // Blog excerpt
                <p class="text-gray-600 mb-4 line-clamp-3 leading-relaxed">
                    {extract_excerpt(&blog_clone.content, 150)}
                </p>

                // Tags
                {if blog_clone.tags.is_empty() {
                    view! { <></> }
                    ().into_any()
                } else {
                    view! {
                        <div class="flex flex-wrap gap-2 mb-4">
                            {blog_clone
                                .tags
                                .iter()
                                .take(3)
                                .map(|tag| {
                                    view! {
                                        <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                                            {tag.clone()}
                                        </span>
                                    }
                                })
                                .collect_view()}
                            {if blog_clone.tags.len() > 3 {
                                view! {
                                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-600">
                                        {format!("+{}", blog_clone.tags.len() - 3)}
                                    </span>
                                }
                                    .into_any()
                            } else {
                                view! { <></> }
                                ().into_any()
                            }}
                        </div>
                    }
                        .into_any()
                }}

                // Read more link
                <div class="flex items-center justify-between">
                    <A href=format!("/blog/{}", blog_clone.id.clone())>
                        <span class="inline-flex items-center text-blue-600 hover:text-blue-800 font-medium transition-colors group">
                            "Read more"
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
                                    d="M9 5l7 7-7 7"
                                />
                            </svg>
                        </span>
                    </A>

                    <div class="text-sm text-gray-500">"by " {blog.author.clone()}</div>
                </div>
            </div>
        </article>
    }
}

// Helper function to estimate reading time
fn estimate_reading_time(content: &str) -> u32 {
    // Average reading speed is about 200-250 words per minute
    // We'll use 225 words per minute as average
    let word_count = content.split_whitespace().count();
    let reading_time = (word_count as f32 / 225.0).ceil() as u32;
    std::cmp::max(1, reading_time) // Minimum 1 minute
}

// Helper function to extract excerpt from HTML content
fn extract_excerpt(html_content: &str, max_length: usize) -> String {
    // Simple HTML tag removal and excerpt extraction
    let text_content = html_content
        .replace("<p>", "")
        .replace("</p>", " ")
        .replace("<br>", " ")
        .replace("<br/>", " ")
        .replace("<h1>", "")
        .replace("</h1>", " ")
        .replace("<h2>", "")
        .replace("</h2>", " ")
        .replace("<h3>", "")
        .replace("</h3>", " ")
        .replace("<strong>", "")
        .replace("</strong>", "")
        .replace("<em>", "")
        .replace("</em>", "")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<li>", "")
        .replace("</li>", " ")
        .replace("<ol>", "")
        .replace("</ol>", "");

    let cleaned_text: String = text_content
        .chars()
        .filter(|c| !c.is_control() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    if cleaned_text.len() <= max_length {
        cleaned_text
    } else {
        let truncated = &cleaned_text[..max_length];
        if let Some(last_space) = truncated.rfind(' ') {
            format!("{}...", &truncated[..last_space])
        } else {
            format!("{truncated}...")
        }
    }
}
