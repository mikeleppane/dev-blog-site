#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::cast_precision_loss)]
#![allow(unused_unsafe)]
use js_sys::wasm_bindgen;
use leptos::server::Resource;
use leptos::{html, prelude::*};
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params_map;

use crate::api::blogs::get_blog_server;
use crate::utils::date_formatter::format_date_readable;
use leptos::suspense::Suspense;
use leptos::web_sys;
use leptos_router::components::A;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightAll();

    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightAllUnder(element: web_sys::Element);
}

#[component]
#[allow(clippy::too_many_lines)]
#[allow(clippy::must_use_candidate)]
pub fn BlogDetail() -> impl IntoView {
    let params = use_params_map();
    let content_ref = NodeRef::<html::Div>::new();
    let blog_id = move || params.with(|params| params.get("id").unwrap_or_default());

    // Resource to fetch the blog post
    let blog_resource = Resource::new(blog_id, |id| async move {
        if id.is_empty() {
            return Err("Blog ID not found".to_string());
        }
        get_blog_server(id).await.map_err(|e| e.to_string())
    });

    Effect::new(move |_| {
        if let Some(Ok(_)) = blog_resource.get() {
            // Small delay to ensure DOM is updated
            set_timeout(
                move || {
                    if let Some(element) = content_ref.get() {
                        unsafe { highlightAllUnder(element.into()) };
                    } else {
                        // Fallback to highlighting all if element ref fails
                        unsafe { highlightAll() };
                    }
                },
                std::time::Duration::from_millis(100),
            );
        }
    });

    view! {
        <Suspense fallback=move || {
            view! {
                <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                    <div class="animate-pulse">
                        <div class="h-8 bg-gray-200 rounded mb-8"></div>
                        <div class="h-64 bg-gray-200 rounded mb-8"></div>
                        <div class="space-y-4">
                            <div class="h-4 bg-gray-200 rounded w-3/4"></div>
                            <div class="h-4 bg-gray-200 rounded w-1/2"></div>
                            <div class="h-4 bg-gray-200 rounded w-5/6"></div>
                        </div>
                    </div>
                </div>
            }
        }>
            {move || match blog_resource.get() {
                Some(Ok(blog_post)) => {
                    view! {
                        <Title text=format!("{} - Mike's Dev Blog", blog_post.title.clone()) />
                        <Meta
                            name="description"
                            content=extract_meta_description(&blog_post.content.clone())
                        />

                        <article class="max-w-4xl mx-auto px-6 sm:px-8 lg:px-12 py-16">
                            // Blog Header with increased spacing
                            <header class="mb-12 pb-8 border-b border-gray-200">
                                // Title with increased margin
                                <h1 class="text-4xl md:text-5xl font-bold text-gray-900 leading-tight mb-8">
                                    {blog_post.title.clone()}
                                </h1>

                                // Meta information with increased spacing
                                <div class="flex items-center text-gray-600 mb-8">
                                    <div class="flex items-center">
                                        <span class="text-sm font-medium">"by "</span>
                                        <span class="text-sm font-semibold text-gray-900 ml-1">
                                            {blog_post.author.clone()}
                                        </span>
                                    </div>
                                    <span class="mx-3 text-gray-400">"•"</span>
                                    <time
                                        datetime=blog_post.created_at.clone().to_rfc3339()
                                        class="text-sm"
                                    >
                                        {format_date_readable(blog_post.created_at)}
                                    </time>
                                    <span class="mx-3 text-gray-400">"•"</span>
                                    <span class="text-sm">
                                        {estimate_reading_time(&blog_post.content)} " min read"
                                    </span>
                                </div>

                                // Tags with increased spacing
                                {if blog_post.tags.is_empty() {
                                    view! { <></> }
                                    ().into_any()
                                } else {
                                    view! {
                                        <div class="flex flex-wrap gap-3 mb-8">
                                            {blog_post
                                                .tags
                                                .clone()
                                                .iter()
                                                .map(|tag| {
                                                    view! {
                                                        <span class="inline-flex items-center px-4 py-2 rounded-full text-sm font-medium bg-blue-100 text-blue-800 hover:bg-blue-200 transition-colors">
                                                            {tag.clone()}
                                                        </span>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any()
                                }}
                            </header>

                            // Cover Image with increased spacing
                            {blog_post
                                .image_url
                                .as_ref()
                                .map(|url| {
                                    view! {
                                        <div class="mb-12">
                                            <figure class="rounded-xl overflow-hidden shadow-lg">
                                                <img
                                                    src=url.clone()
                                                    alt=format!("Cover image for {}", blog_post.title.clone())
                                                    class="w-full h-auto object-cover"
                                                    loading="eager"
                                                />
                                            </figure>
                                        </div>
                                    }
                                        .into_any()
                                })
                                .unwrap_or_else(|| { view! { <div></div> }.into_any() })}

                            // Blog Content with increased spacing and padding
                            <div class="prose prose-lg prose-blue max-w-none">
                                <div
                                    class="blog-content px-4 py-8 leading-relaxed"
                                    inner_html=blog_post.content.clone()
                                ></div>
                            </div>

                            // Footer section with increased spacing
                            <footer class="mt-16 pt-8 border-t border-gray-200">
                                <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
                                    <div class="mb-4 sm:mb-0">
                                        <p class="text-sm text-gray-600">
                                            "Published on "
                                            <time
                                                datetime=blog_post.created_at.to_rfc3339()
                                                class="font-medium"
                                            >
                                                {format_date_readable(blog_post.created_at)}
                                            </time>
                                        </p>
                                        {if blog_post.updated_at == blog_post.created_at {
                                            view! { <></> }
                                            ().into_any()
                                        } else {
                                            view! {
                                                <p class="text-sm text-gray-500 mt-1">
                                                    "Last updated on "
                                                    <time datetime=blog_post
                                                        .updated_at
                                                        .to_rfc3339()>
                                                        {format_date_readable(blog_post.updated_at)}
                                                    </time>
                                                </p>
                                            }
                                                .into_any()
                                        }}
                                    </div>

                                    <div class="flex space-x-4">
                                        <A href="/blog">
                                            <span class="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 transition-colors">
                                                <svg
                                                    class="mr-2 h-4 w-4"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    viewBox="0 0 24 24"
                                                >
                                                    <path
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        stroke-width="2"
                                                        d="M7 16l-4-4m0 0l4-4m-4 4h18"
                                                    />
                                                </svg>
                                                "Back to Blog"
                                            </span>
                                        </A>

                                        <button
                                            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors"
                                            on:click=move |_| {
                                                let clipboard = window().navigator().clipboard();
                                                let url = window().location().href().unwrap_or_default();
                                                let _ = clipboard.write_text(&url);
                                            }
                                        >
                                            <svg
                                                class="mr-2 h-4 w-4"
                                                fill="none"
                                                stroke="currentColor"
                                                viewBox="0 0 24 24"
                                            >
                                                <path
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    stroke-width="2"
                                                    d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
                                                />
                                            </svg>
                                            "Share"
                                        </button>
                                    </div>
                                </div>
                            </footer>
                        </article>
                    }
                        .into_any()
                }
                Some(Err(error)) => {
                    view! {
                        <div class="max-w-4xl mx-auto px-6 sm:px-8 lg:px-12 py-16">
                            <div class="text-center">
                                <div class="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-red-100 mb-6">
                                    <svg
                                        class="h-8 w-8 text-red-600"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke="currentColor"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"
                                        />
                                    </svg>
                                </div>
                                <h1 class="text-2xl font-bold text-gray-900 mb-4">
                                    "Blog Post Not Found"
                                </h1>
                                <p class="text-gray-600 mb-8">{error}</p>
                                <A href="/blog">
                                    <span class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors">
                                        "Back to Blog"
                                    </span>
                                </A>
                            </div>
                        </div>
                    }
                        .into_any()
                }
                None => {
                    view! { <></> }
                    ().into_any()
                }
            }}
        </Suspense>
    }
}

// Helper function to estimate reading time
fn estimate_reading_time(content: &str) -> u32 {
    let word_count = content.split_whitespace().count();
    let reading_time = (word_count as f32 / 225.0).ceil() as u32;
    std::cmp::max(1, reading_time)
}

// Helper function to extract meta description from content
fn extract_meta_description(content: &str) -> String {
    let text_content = content
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
        .replace("</em>", "");

    let cleaned_text: String = text_content
        .chars()
        .filter(|c| !c.is_control() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    if cleaned_text.len() <= 160 {
        cleaned_text
    } else {
        let truncated = &cleaned_text[..160];
        if let Some(last_space) = truncated.rfind(' ') {
            format!("{}...", &truncated[..last_space])
        } else {
            format!("{truncated}...")
        }
    }
}
