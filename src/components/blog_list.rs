#![allow(clippy::manual_div_ceil)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::cast_precision_loss)]
use leptos::{logging, prelude::*};

use crate::{api::blogs::get_blogs_server, models::blog::BlogPost};
use leptos_router::components::A;
#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn BlogList() -> impl IntoView {
    // Create an Action for fetching blogs instead of Resource
    let fetch_blogs_action = Action::new(|(): &()| async move { get_blogs_server().await });

    // Create an Action for refreshing blogs
    let refresh_blogs_action = Action::new(|(): &()| async move { get_blogs_server().await });

    // Trigger initial fetch when component mounts
    Effect::new(move |_| {
        fetch_blogs_action.dispatch(());
    });

    // Handle refresh action results
    Effect::new(move |_| {
        if let Some(result) = refresh_blogs_action.value().get() {
            if result.is_ok() {
                // Optionally show success message or handle refresh completion
                logging::log!("Blog list refreshed successfully");
            }
        }
    });

    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            // Header with refresh button
            <div class="flex justify-between items-center mb-12">
                <h2 class="text-3xl font-extrabold tracking-tight text-gray-900 sm:text-4xl">
                    <span class="block bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                        "Latest Blog Posts"
                    </span>
                </h2>
                <div class="flex items-center space-x-4">
                    <button
                        type="button"
                        class="inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        on:click=move |_| {
                            refresh_blogs_action.dispatch(());
                        }
                        disabled=move || {
                            refresh_blogs_action.pending().get()
                                || fetch_blogs_action.pending().get()
                        }
                    >
                        {move || {
                            if refresh_blogs_action.pending().get() {
                                view! {
                                    <svg
                                        class="animate-spin -ml-1 mr-2 h-4 w-4 text-gray-700"
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
                                    "Refreshing..."
                                }
                            } else {
                                view! {
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="h-4 w-4 mr-2"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke="currentColor"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                                        />
                                    </svg>
                                    "Refresh"
                                }
                            }
                        }}
                    </button>

                // www.w3.org/2000/svg"
                </div>
            </div>

            // Content area
            <div class="space-y-8">
                {move || {
                    let current_result = if let Some(refresh_result) = refresh_blogs_action
                        .value()
                        .get()
                    {
                        Some(refresh_result)
                    } else {
                        fetch_blogs_action.value().get()
                    };
                    if fetch_blogs_action.pending().get() && current_result.is_none() {
                        // Get the most recent result from either action

                        // Show loading state
                        view! {
                            <div class="text-center py-12">
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
                                    <span class="text-xl text-gray-600">
                                        "Loading blog posts..."
                                    </span>
                                </div>
                            </div>
                        }
                            .into_any()
                    } else if let Some(result) = current_result {
                        match result {
                            Ok(posts) => {
                                if posts.is_empty() {
                                    view! {
                                        <div class="text-center py-16">
                                            <svg
                                                class="mx-auto h-12 w-12 text-gray-400"
                                                fill="none"
                                                viewBox="0 0 24 24"
                                                stroke="currentColor"
                                                aria-hidden="true"
                                            >
                                                <path
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    stroke-width="2"
                                                    d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                                />
                                            </svg>
                                            <h3 class="mt-2 text-2xl font-medium text-gray-900">
                                                "No blog posts yet"
                                            </h3>
                                            <p class="mt-1 text-lg text-gray-500">
                                                "Get started by writing your first blog post."
                                            </p>
                                            <div class="mt-6">
                                                <A
                                                    href="/blog-editor"
                                                    attr:class="inline-flex items-center px-6 py-3 border border-transparent shadow-sm text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                                                >
                                                    <svg
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        class="h-5 w-5 mr-2"
                                                        fill="none"
                                                        viewBox="0 0 24 24"
                                                        stroke="currentColor"
                                                    >
                                                        <path
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            stroke-width="2"
                                                            d="M12 4v16m8-8H4"
                                                        />
                                                    </svg>
                                                    "Write your first post"
                                                </A>
                                            </div>
                                        </div>
                                    }
                                        .into_any()
                                } else if refresh_blogs_action.pending().get() {
                                    view! {
                                        <div class="space-y-8">
                                            <div class="bg-blue-50 border border-blue-200 rounded-md p-4">
                                                <div class="flex items-center">
                                                    <svg
                                                        class="animate-spin h-5 w-5 text-blue-600 mr-3"
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
                                                    <span class="text-sm text-blue-800">
                                                        "Refreshing blog posts..."
                                                    </span>
                                                </div>
                                            </div>
                                            <div class="grid gap-8 md:grid-cols-2 lg:grid-cols-3">
                                                {posts
                                                    .into_iter()
                                                    .map(|post| {
                                                        view! { <BlogPostCard post=post /> }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <div class="grid gap-8 md:grid-cols-2 lg:grid-cols-3">
                                            {posts
                                                .into_iter()
                                                .map(|post| {
                                                    view! { <BlogPostCard post=post /> }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any()
                                }
                            }
                            Err(err) => {
                                view! {
                                    <div class="text-center py-12">
                                        <div class="bg-red-50 border-l-4 border-red-400 p-6 rounded-md max-w-lg mx-auto">
                                            <div class="flex">
                                                <div class="flex-shrink-0">
                                                    <svg
                                                        class="h-5 w-5 text-red-400"
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        viewBox="0 0 20 20"
                                                        fill="currentColor"
                                                    >
                                                        <path
                                                            fill-rule="evenodd"
                                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                                                            clip-rule="evenodd"
                                                        />
                                                    </svg>
                                                </div>
                                                <div class="ml-3">
                                                    <h3 class="text-sm font-medium text-red-800">
                                                        "Error Loading Blog Posts"
                                                    </h3>
                                                    <p class="mt-2 text-sm text-red-700">
                                                        {format!("Failed to load blog posts: {err}")}
                                                    </p>
                                                    <div class="mt-4">
                                                        <button
                                                            type="button"
                                                            class="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-red-700 bg-red-100 hover:bg-red-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                                                            on:click=move |_| {
                                                                fetch_blogs_action.dispatch(());
                                                            }
                                                        >
                                                            "Try Again"
                                                        </button>
                                                    </div>
                                                </div>
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
                                <p class="text-xl text-gray-600">"Initializing..."</p>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn BlogPostCard(post: BlogPost) -> impl IntoView {
    let formatted_date = post.created_at.format("%B %d, %Y").to_string();
    let summary = if post.content.len() > 200 {
        let mut truncated = post.content.chars().take(200).collect::<String>();
        // Try to break at word boundary
        if let Some(last_space) = truncated.rfind(' ') {
            truncated.truncate(last_space);
        }
        format!("{truncated}...")
    } else {
        post.content.clone()
    };

    view! {
        <article class="group flex flex-col overflow-hidden rounded-lg shadow-lg hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1 bg-white">
            <div class="flex-shrink-0">
                {post
                    .image_url
                    .map(|url| {
                        view! {
                            <div class="aspect-w-16 aspect-h-9">
                                <img
                                    class="h-48 w-full object-cover group-hover:scale-105 transition-transform duration-300"
                                    src=url
                                    alt=&post.title
                                    loading="lazy"
                                />
                            </div>
                        }
                    })
                    .unwrap_or_else(|| {
                        view! {
                            <div class="h-48 w-full bg-gradient-to-br from-blue-400 via-purple-500 to-pink-500 flex items-center justify-center group-hover:from-blue-500 group-hover:via-purple-600 group-hover:to-pink-600 transition-all duration-300">
                                <span class="text-white text-4xl font-bold drop-shadow-lg">
                                    {post.title.chars().next().unwrap_or('B').to_string()}
                                </span>
                            </div>
                        }
                    })}
            </div>

            <div class="flex flex-1 flex-col justify-between p-6">
                <div class="flex-1">
                    // Tags
                    <div class="flex items-center gap-2 mb-3">
                        {post
                            .tags
                            .iter()
                            .take(3)
                            .map(|tag| {
                                view! {
                                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 hover:bg-blue-200 transition-colors">
                                        {tag.clone()}
                                    </span>
                                }
                            })
                            .collect_view()}
                        {if post.tags.len() > 3 {
                            view! {
                                <span class="text-xs text-gray-500">
                                    {format!("+{} more", post.tags.len() - 3)}
                                </span>
                            }
                                .into_any()
                        } else {
                            view! { <></> }
                            ().into_any()
                        }}
                    </div>

                    // Title and content
                    <A href=format!("/blog/{}", post.id) attr:class="block">
                        <h3 class="text-xl font-semibold text-gray-900 group-hover:text-blue-600 transition-colors duration-300 line-clamp-2">
                            {post.title.clone()}
                        </h3>
                        <p class="mt-3 text-base text-gray-500 line-clamp-3" inner_html=summary></p>
                    </A>
                </div>

                // Author and date
                <div class="mt-6 flex items-center">
                    <div class="flex-shrink-0">
                        <span class="sr-only">{post.author.clone()}</span>
                        <div class="h-10 w-10 rounded-full bg-gradient-to-r from-blue-400 to-purple-500 flex items-center justify-center ring-2 ring-white">
                            <span class="text-white font-medium text-sm">
                                {post.author.chars().next().unwrap_or('A').to_string()}
                            </span>
                        </div>
                    </div>
                    <div class="ml-3">
                        <p class="text-sm font-medium text-gray-900">{post.author.clone()}</p>
                        <div class="flex space-x-1 text-sm text-gray-500">
                            <time datetime=post.created_at.to_rfc3339()>{formatted_date}</time>
                            <span aria-hidden="true">"·"</span>
                            <span>
                                {format!(
                                    "{} min read",
                                    (post.content.split_whitespace().count() as f32 / 200.0).ceil()
                                        as u32,
                                )}
                            </span>
                        </div>
                    </div>
                </div>
            </div>
        </article>
    }
}
