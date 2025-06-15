use crate::api::blogs::get_blog_server;
use crate::models::blog::BlogPost;
use js_sys::wasm_bindgen;
use leptos::prelude::*;
use leptos::web_sys;
use leptos_router::hooks::use_navigate;
use leptos_router::hooks::use_params_map;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightAll();

    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightAllUnder(element: web_sys::Element);
}

#[component]
pub fn BlogEditor() -> impl IntoView {
    let navigate = use_navigate();
    let params = use_params_map();

    // Check if we're editing an existing post or creating a new one
    let blog_id = move || params.with(|params| params.get("id").unwrap_or_default().to_string());
    let is_editing = move || !blog_id().is_empty();

    // Form state with larger initial values for better UX
    let (title, set_title) = signal(String::new());
    let (content, set_content) = signal(String::new());
    let (tags, set_tags) = signal(String::new());
    let (image_url, set_image_url) = signal(String::new());
    let (saving, set_saving) = signal(false);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal::<Option<String>>(None);
    let (success_message, set_success_message) = signal::<Option<String>>(None);

    // Preview mode state
    let (preview_mode, set_preview_mode) = signal(false);
    let preview_ref = NodeRef::new();

    // Load existing blog post if editing
    let load_blog_action = Action::new(|id: &String| {
        let id = id.clone();
        async move { get_blog_server(id).await }
    });

    // Effect to load blog post when editing
    Effect::new(move |_| {
            if !blog_id().is_empty() && blog_id() != "new" {
                set_loading.set(true);
                load_blog_action.dispatch(blog_id());
            }
    });

    // Effect to handle loaded blog post
    Effect::new(move |_| {
        if let Some(result) = load_blog_action.value().get() {
            set_loading.set(false);
            match result {
                Ok(blog_post) => {
                    set_title.set(blog_post.title);
                    set_content.set(blog_post.content);
                    set_tags.set(blog_post.tags.join(", "));
                    set_image_url.set(blog_post.image_url.unwrap_or_default());
                    set_error.set(None);
                }
                Err(err) => {
                    set_error.set(Some(format!("Failed to load blog post: {}", err)));
                }
            }
        }
    });

    // Effect to highlight syntax when in preview mode
    create_effect(move |_| {
        if preview_mode.get() {
            set_timeout(
                move || {
                    if let Some(element) = preview_ref.get() {
                        highlightAllUnder(element.into());
                    }
                },
                std::time::Duration::from_millis(100),
            );
        }
    });

    // Form submission handler
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        set_saving.set(true);
        set_error.set(None);
        set_success_message.set(None);

        let title_val = title.get().trim().to_string();
        let content_val = content.get().trim().to_string();
        let tags_val = tags.get().trim().to_string();
        let image_url_val = image_url.get().trim().to_string();

        // Validation
        if title_val.is_empty() {
            set_error.set(Some("Title is required".to_string()));
            set_saving.set(false);
            return;
        }

        if content_val.is_empty() {
            set_error.set(Some("Content is required".to_string()));
            set_saving.set(false);
            return;
        }

        // Process tags
        let tag_list = if tags_val.is_empty() {
            vec![]
        } else {
            tags_val
                .split(',')
                .map(|tag| tag.trim().to_string())
                .filter(|tag| !tag.is_empty())
                .collect::<Vec<String>>()
        };

        let image_url_opt = if image_url_val.is_empty() {
            None
        } else {
            Some(image_url_val)
        };

        let navigate = navigate.clone();
        spawn_local(async move {
            let result = if let Some(existing_id) = blog_id() {
                if !existing_id.is_empty() && existing_id != "new" {
                    // Update existing blog post
                    let mut post = BlogPost::new(
                        title_val,
                        content_val,
                        "Mike".to_string(), // Your author name
                        tag_list,
                        image_url_opt,
                    );
                    post.id = existing_id;
                    update_blog_server(post).await
                } else {
                    // Create new blog post
                    let post = BlogPost::new(
                        title_val,
                        content_val,
                        "Mike".to_string(),
                        tag_list,
                        image_url_opt,
                    );
                    create_blog_server(post).await
                }
            } else {
                // Create new blog post
                let post = BlogPost::new(
                    title_val,
                    content_val,
                    "Mike".to_string(),
                    tag_list,
                    image_url_opt,
                );
                create_blog_server(post).await
            };

            match result {
                Ok(_) => {
                    set_success_message.set(Some("Blog post saved successfully!".to_string()));
                    set_saving.set(false);
                    // Navigate after a short delay to show success message
                    set_timeout(
                        move || {
                            navigate("/blog", NavigateOptions::default());
                        },
                        std::time::Duration::from_millis(1500),
                    );
                }
                Err(err) => {
                    set_error.set(Some(format!("Error saving post: {}", err)));
                    set_saving.set(false);
                }
            }
        });
    };

    // Helper functions
    let toggle_preview = move |_| set_preview_mode.update(|prev| *prev = !*prev);

    let insert_code_block = move |language: &str| {
        let current_content = content.get();
        let cursor_pos = current_content.len();
        let code_template = format!(
            "\n\n```{}\n// Your {} code here\nfn main() {{\n    println!(\"Hello, world!\");\n}}\n```\n\n",
            language, language
        );
        let new_content = format!("{}{}", current_content, code_template);
        set_content.set(new_content);
    };

    let clear_messages = move |_| {
        set_error.set(None);
        set_success_message.set(None);
    };

    view! {
        <div class="min-h-screen bg-gray-50 py-8">
            <div class="max-w-7xl mx-auto px-6 sm:px-8 lg:px-12">
                // Header Section
                <div class="mb-12 bg-white rounded-xl shadow-sm border border-gray-200 p-8">
                    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
                        <div class="mb-4 sm:mb-0">
                            <h1 class="text-4xl font-bold text-gray-900 mb-2">
                                {move || {
                                    if is_editing() {
                                        "Edit Blog Post"
                                    } else {
                                        "Create New Blog Post"
                                    }
                                }}
                            </h1>
                            <p class="text-lg text-gray-600">
                                {move || {
                                    if is_editing() {
                                        "Update your existing blog post with new content and insights"
                                    } else {
                                        "Share your knowledge and insights with the world"
                                    }
                                }}
                            </p>
                        </div>
                        <div class="flex items-center space-x-4">
                            <button
                                type="button"
                                class="px-6 py-3 text-sm font-medium text-gray-700 hover:text-gray-900 bg-white border border-gray-300 rounded-lg shadow-sm hover:bg-gray-50 transition-colors"
                                on:click=toggle_preview
                            >
                                {move || {
                                    if preview_mode.get() {
                                        view! {
                                            <>
                                                <svg
                                                    class="w-4 h-4 mr-2 inline"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    viewBox="0 0 24 24"
                                                >
                                                    <path
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        stroke-width="2"
                                                        d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                                                    />
                                                </svg>
                                                "Edit Mode"
                                            </>
                                        }
                                    } else {
                                        view! {
                                            <>
                                                <svg
                                                    class="w-4 h-4 mr-2 inline"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    viewBox="0 0 24 24"
                                                >
                                                    <path
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        stroke-width="2"
                                                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                                    />
                                                    <path
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        stroke-width="2"
                                                        d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                                                    />
                                                </svg>
                                                "Preview Mode"
                                            </>
                                        }
                                    }
                                }}
                            </button>
                            <A
                                href="/blog"
                                class="px-6 py-3 text-sm font-medium text-gray-700 hover:text-gray-900 bg-white border border-gray-300 rounded-lg shadow-sm hover:bg-gray-50 transition-colors"
                            >
                                "Cancel"
                            </A>
                        </div>
                    </div>
                </div>

                // Loading State
                {move || {
                    if loading.get() {
                        view! {
                            <div class="flex justify-center items-center py-16">
                                <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-8">
                                    <div class="flex items-center">
                                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mr-4"></div>
                                        <span class="text-lg text-gray-600">
                                            "Loading blog post..."
                                        </span>
                                    </div>
                                </div>
                            </div>
                        }
                            .into_view()
                    } else {
                        view! {
                            <>
                                // Success/Error Messages
                                {move || {
                                    if let Some(success) = success_message.get() {
                                        view! {
                                            <div class="mb-8 bg-green-50 border-l-4 border-green-400 rounded-lg p-6 shadow-sm">
                                                <div class="flex items-center">
                                                    <svg
                                                        class="h-6 w-6 text-green-400 mr-3"
                                                        fill="none"
                                                        viewBox="0 0 24 24"
                                                        stroke="currentColor"
                                                    >
                                                        <path
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            stroke-width="2"
                                                            d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                                        />
                                                    </svg>
                                                    <div>
                                                        <p class="text-sm font-medium text-green-800">{success}</p>
                                                        <p class="text-sm text-green-700 mt-1">
                                                            "Redirecting to blog list..."
                                                        </p>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                            .into_view()
                                    } else if let Some(err) = error.get() {
                                        view! {
                                            <div class="mb-8 bg-red-50 border-l-4 border-red-400 rounded-lg p-6 shadow-sm">
                                                <div class="flex items-center justify-between">
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
                                                                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                                            />
                                                        </svg>
                                                        <div>
                                                            <p class="text-sm font-medium text-red-800">"Error"</p>
                                                            <p class="text-sm text-red-700">{err}</p>
                                                        </div>
                                                    </div>
                                                    <button
                                                        class="text-red-400 hover:text-red-600 transition-colors"
                                                        on:click=clear_messages
                                                    >
                                                        <svg
                                                            class="h-5 w-5"
                                                            fill="none"
                                                            stroke="currentColor"
                                                            viewBox="0 0 24 24"
                                                        >
                                                            <path
                                                                stroke-linecap="round"
                                                                stroke-linejoin="round"
                                                                stroke-width="2"
                                                                d="M6 18L18 6M6 6l12 12"
                                                            />
                                                        </svg>
                                                    </button>
                                                </div>
                                            </div>
                                        }
                                            .into_view()
                                    } else {
                                        view! { <></> }.into_view()
                                    }
                                }}
                                {move || {
                                    if preview_mode.get() {
                                        // Preview Mode
                                        view! {
                                            <div class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
                                                <div class="px-8 py-6 border-b border-gray-200 bg-gray-50">
                                                    <h2 class="text-xl font-semibold text-gray-900">
                                                        "Live Preview"
                                                    </h2>
                                                    <p class="text-sm text-gray-600 mt-1">
                                                        "This is how your blog post will appear to readers"
                                                    </p>
                                                </div>
                                                <div class="p-8" node_ref=preview_ref>
                                                    {move || {
                                                        if !title.get().is_empty() {
                                                            view! {
                                                                <h1 class="text-4xl font-bold text-gray-900 mb-8 leading-tight">
                                                                    {title.get()}
                                                                </h1>
                                                            }
                                                                .into_view()
                                                        } else {
                                                            view! {
                                                                <div class="text-gray-400 italic mb-8">
                                                                    "Enter a title to see it here"
                                                                </div>
                                                            }
                                                                .into_view()
                                                        }
                                                    }}

                                                    {move || {
                                                        if !image_url.get().is_empty() {
                                                            view! {
                                                                <div class="mb-8">
                                                                    <img
                                                                        src=image_url.get()
                                                                        alt="Cover"
                                                                        class="rounded-xl shadow-lg max-h-96 w-full object-cover"
                                                                        on:error=move |_| {
                                                                            set_error
                                                                                .set(
                                                                                    Some(
                                                                                        "Failed to load cover image. Please check the URL."
                                                                                            .to_string(),
                                                                                    ),
                                                                                );
                                                                        }
                                                                    />
                                                                </div>
                                                            }
                                                                .into_view()
                                                        } else {
                                                            view! { <></> }.into_view()
                                                        }
                                                    }}

                                                    {move || {
                                                        if !tags.get().is_empty() {
                                                            view! {
                                                                <div class="flex flex-wrap gap-3 mb-8">
                                                                    {tags
                                                                        .get()
                                                                        .split(",")
                                                                        .map(|tag| {
                                                                            let tag = tag.trim();
                                                                            if !tag.is_empty() {
                                                                                view! {
                                                                                    <span class="inline-flex items-center px-4 py-2 rounded-full text-sm font-medium bg-blue-100 text-blue-800">
                                                                                        {tag}
                                                                                    </span>
                                                                                }
                                                                            } else {
                                                                                view! { <></> }
                                                                            }
                                                                        })
                                                                        .collect_view()}
                                                                </div>
                                                            }
                                                                .into_view()
                                                        } else {
                                                            view! { <></> }.into_view()
                                                        }
                                                    }}

                                                    <div class="prose prose-lg max-w-none">
                                                        {move || {
                                                            if !content.get().is_empty() {
                                                                view! {
                                                                    <div inner_html=convert_markdown_to_html(
                                                                        &content.get(),
                                                                    )></div>
                                                                }
                                                                    .into_view()
                                                            } else {
                                                                view! {
                                                                    <div class="text-gray-400 italic">
                                                                        "Enter content to see the preview"
                                                                    </div>
                                                                }
                                                                    .into_view()
                                                            }
                                                        }}
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                            .into_view()
                                    } else {
                                        // Editor Mode
                                        view! {
                                            <form on:submit=on_submit class="space-y-8">
                                                // Title Input
                                                <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-8">
                                                    <label
                                                        for="title"
                                                        class="block text-lg font-semibold text-gray-900 mb-4"
                                                    >
                                                        "Blog Title"
                                                        <span class="text-red-500 ml-1">"*"</span>
                                                    </label>
                                                    <input
                                                        type="text"
                                                        name="title"
                                                        id="title"
                                                        class="block w-full text-xl border-gray-300 rounded-lg shadow-sm focus:ring-blue-500 focus:border-blue-500 px-6 py-4 placeholder-gray-400"
                                                        placeholder="Enter an engaging title for your blog post..."
                                                        prop:value=title
                                                        on:input=move |ev| {
                                                            set_title.set(event_target_value(&ev));
                                                        }
                                                        required
                                                    />
                                                    <p class="mt-3 text-sm text-gray-500">
                                                        "A great title is clear, specific, and captures the reader's attention. Aim for 50-60 characters."
                                                    </p>
                                                </div>

                                                // Cover Image URL Input
                                                <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-8">
                                                    <label
                                                        for="image_url"
                                                        class="block text-lg font-semibold text-gray-900 mb-4"
                                                    >
                                                        "Cover Image URL"
                                                        <span class="text-gray-500 font-normal ml-2">
                                                            "(optional)"
                                                        </span>
                                                    </label>
                                                    <input
                                                        type="url"
                                                        name="image_url"
                                                        id="image_url"
                                                        class="block w-full text-lg border-gray-300 rounded-lg shadow-sm focus:ring-blue-500 focus:border-blue-500 px-6 py-4 placeholder-gray-400"
                                                        placeholder="https://example.com/your-cover-image.jpg"
                                                        prop:value=image_url
                                                        on:input=move |ev| {
                                                            set_image_url.set(event_target_value(&ev));
                                                        }
                                                    />
                                                    <p class="mt-3 text-sm text-gray-500">
                                                        "Add a compelling cover image to make your post more visually appealing. Use high-quality images from Unsplash, Pexels, or your own collection."
                                                    </p>
                                                    {move || {
                                                        if !image_url.get().is_empty() {
                                                            view! {
                                                                <div class="mt-4 p-4 bg-gray-50 rounded-lg">
                                                                    <p class="text-sm font-medium text-gray-700 mb-2">
                                                                        "Preview:"
                                                                    </p>
                                                                    <img
                                                                        src=image_url.get()
                                                                        alt="Cover preview"
                                                                        class="rounded-lg max-h-32 object-cover"
                                                                        on:error=move |_| {
                                                                            set_error
                                                                                .set(
                                                                                    Some(
                                                                                        "Invalid image URL. Please check the URL and try again."
                                                                                            .to_string(),
                                                                                    ),
                                                                                );
                                                                        }
                                                                    />
                                                                </div>
                                                            }
                                                                .into_view()
                                                        } else {
                                                            view! { <></> }.into_view()
                                                        }
                                                    }}
                                                </div>

                                                // Tags Input
                                                <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-8">
                                                    <label
                                                        for="tags"
                                                        class="block text-lg font-semibold text-gray-900 mb-4"
                                                    >
                                                        "Tags"
                                                        <span class="text-gray-500 font-normal ml-2">
                                                            "(comma separated)"
                                                        </span>
                                                    </label>
                                                    <input
                                                        type="text"
                                                        name="tags"
                                                        id="tags"
                                                        class="block w-full text-lg border-gray-300 rounded-lg shadow-sm focus:ring-blue-500 focus:border-blue-500 px-6 py-4 placeholder-gray-400"
                                                        placeholder="rust, web development, tutorial, programming, leptos"
                                                        prop:value=tags
                                                        on:input=move |ev| {
                                                            set_tags.set(event_target_value(&ev));
                                                        }
                                                    />
                                                    <p class="mt-3 text-sm text-gray-500">
                                                        "Tags help readers find your content. Use relevant keywords separated by commas. Examples: rust, programming, tutorial, web development"
                                                    </p>
                                                    {move || {
                                                        if !tags.get().is_empty() {
                                                            view! {
                                                                <div class="mt-4 p-4 bg-gray-50 rounded-lg">
                                                                    <p class="text-sm font-medium text-gray-700 mb-2">
                                                                        "Tag Preview:"
                                                                    </p>
                                                                    <div class="flex flex-wrap gap-2">
                                                                        {tags
                                                                            .get()
                                                                            .split(",")
                                                                            .map(|tag| {
                                                                                let tag = tag.trim();
                                                                                if !tag.is_empty() {
                                                                                    view! {
                                                                                        <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800">
                                                                                            {tag}
                                                                                        </span>
                                                                                    }
                                                                                } else {
                                                                                    view! { <></> }
                                                                                }
                                                                            })
                                                                            .collect_view()}
                                                                    </div>
                                                                </div>
                                                            }
                                                                .into_view()
                                                        } else {
                                                            view! { <></> }.into_view()
                                                        }
                                                    }}
                                                </div>

                                                // Code Block Quick Insert
                                                <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-8">
                                                    <h3 class="text-lg font-semibold text-gray-900 mb-4">
                                                        "Quick Insert Code Blocks"
                                                    </h3>
                                                    <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-3">
                                                        <button
                                                            type="button"
                                                            class="px-4 py-3 text-sm font-medium bg-orange-100 text-orange-800 rounded-lg hover:bg-orange-200 transition-colors flex items-center justify-center"
                                                            on:click=move |_| insert_code_block("rust")
                                                        >
                                                            <svg
                                                                class="w-4 h-4 mr-2"
                                                                fill="currentColor"
                                                                viewBox="0 0 24 24"
                                                            >
                                                                <path d="M23.8 12.2l-2.6-1.4c.3-.9.3-1.9 0-2.8l2.6-1.4c.3-.2.4-.6.2-.9l-2.4-4.2c-.2-.3-.6-.4-.9-.2l-2.6 1.4c-.7-.5-1.5-.9-2.4-1.1V.8c0-.4-.3-.8-.8-.8h-4.8c-.4 0-.8.3-.8.8v2.8c-.9.2-1.7.6-2.4 1.1L4.3 3.3c-.3-.2-.7-.1-.9.2L.9 7.7c-.2.3-.1.7.2.9l2.6 1.4c-.3.9-.3 1.9 0 2.8l-2.6 1.4c-.3.2-.4.6-.2.9l2.4 4.2c.2.3.6.4.9.2l2.6-1.4c.7.5 1.5.9 2.4 1.1v2.8c0 .4.3.8.8.8h4.8c.4 0 .8-.3.8-.8v-2.8c.9-.2 1.7-.6 2.4-1.1l2.6 1.4c.3.2.7.1.9-.2l2.4-4.2c.2-.3.1-.7-.2-.9zM12 16.5c-2.5 0-4.5-2-4.5-4.5s2-4.5 4.5-4.5 4.5 2 4.5 4.5-2 4.5-4.5 4.5zm0-7c-1.4 0-2.5 1.1-2.5 2.5s1.1 2.5 2.5 2.5 2.5-1.1 2.5-2.5-1.1-2.5-2.5-2.5z" />
                                                            </svg>
                                                            "Rust"
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="px-4 py-3 text-sm font-medium bg-blue-100 text-blue-800 rounded-lg hover:bg-blue-200 transition-colors"
                                                            on:click=move |_| insert_code_block("python")
                                                        >
                                                            "Python"
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="px-4 py-3 text-sm font-medium bg-yellow-100 text-yellow-800 rounded-lg hover:bg-yellow-200 transition-colors"
                                                            on:click=move |_| insert_code_block("javascript")
                                                        >
                                                            "JavaScript"
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="px-4 py-3 text-sm font-medium bg-green-100 text-green-800 rounded-lg hover:bg-green-200 transition-colors"
                                                            on:click=move |_| insert_code_block("bash")
                                                        >
                                                            "Bash"
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class="px-4 py-3 text-sm font-medium bg-purple-100 text-purple-800 rounded-lg hover:bg-purple-200 transition-colors"
                                                            on:click=move |_| insert_code_block("sql")
                                                        >
                                                            "SQL"
                                                        </button>
                                                    </div>
                                                </div>

                                                // Content Editor
                                                <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-8">
                                                    <label
                                                        for="content"
                                                        class="block text-lg font-semibold text-gray-900 mb-4"
                                                    >
                                                        "Blog Content"
                                                        <span class="text-red-500 ml-1">"*"</span>
                                                    </label>
                                                    <div class="relative">
                                                        <textarea
                                                            name="content"
                                                            id="content"
                                                            rows="25"
                                                            class="block w-full border border-gray-300 rounded-lg shadow-sm focus:ring-blue-500 focus:border-blue-500 text-base font-mono px-6 py-4 placeholder-gray-400 leading-relaxed"
                                                            placeholder="Write your blog content here...
                                                            
                                                            Tips for great content:
                                                            - Use markdown for formatting
                                                            - Add code blocks with ```language
                                                            - Include examples and explanations
                                                            - Break up text with headers and lists
                                                            
                                                            Example code block:
                                                            ```rust
                                                            fn main() {
                                                            println!(\"Hello, world!\");
                                                            }
                                                            ```"
                                                            prop:value=content
                                                            on:input=move |ev| {
                                                                set_content.set(event_target_value(&ev));
                                                            }
                                                            style="resize: vertical; min-height: 600px;"
                                                            required
                                                        ></textarea>
                                                    </div>
                                                    <div class="mt-4 p-4 bg-blue-50 rounded-lg">
                                                        <h4 class="text-sm font-medium text-blue-900 mb-2">
                                                            "Content Writing Tips:"
                                                        </h4>
                                                        <ul class="text-sm text-blue-800 space-y-1">
                                                            <li>
                                                                "• Use ```language for syntax-highlighted code blocks"
                                                            </li>
                                                            <li>"• HTML tags are supported for rich formatting"</li>
                                                            <li>
                                                                "• Use the quick insert buttons above for common code blocks"
                                                            </li>
                                                            <li>
                                                                "• Break up long content with subheadings and lists"
                                                            </li>
                                                            <li>"• Include practical examples and explanations"</li>
                                                        </ul>
                                                    </div>
                                                </div>

                                                // Submit Button
                                                <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-8">
                                                    <div class="flex flex-col sm:flex-row sm:justify-between sm:items-center">
                                                        <div class="mb-4 sm:mb-0">
                                                            <p class="text-sm text-gray-600">
                                                                "Ready to publish? Make sure you've filled in the title and content."
                                                            </p>
                                                        </div>
                                                        <button
                                                            type="submit"
                                                            class="inline-flex items-center justify-center px-8 py-4 border border-transparent text-lg font-semibold rounded-lg text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-sm hover:shadow-md"
                                                            disabled=saving
                                                        >
                                                            {move || {
                                                                if saving.get() {
                                                                    view! {
                                                                        <>
                                                                            <svg
                                                                                class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
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
                                                                            "Saving..."
                                                                        </>
                                                                    }
                                                                } else if is_editing() {
                                                                    view! {
                                                                        <>
                                                                            <svg
                                                                                class="w-5 h-5 mr-2"
                                                                                fill="none"
                                                                                stroke="currentColor"
                                                                                viewBox="0 0 24 24"
                                                                            >
                                                                                <path
                                                                                    stroke-linecap="round"
                                                                                    stroke-linejoin="round"
                                                                                    stroke-width="2"
                                                                                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
                                                                                />
                                                                            </svg>
                                                                            "Update Post"
                                                                        </>
                                                                    }
                                                                } else {
                                                                    view! {
                                                                        <>
                                                                            <svg
                                                                                class="w-5 h-5 mr-2"
                                                                                fill="none"
                                                                                stroke="currentColor"
                                                                                viewBox="0 0 24 24"
                                                                            >
                                                                                <path
                                                                                    stroke-linecap="round"
                                                                                    stroke-linejoin="round"
                                                                                    stroke-width="2"
                                                                                    d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"
                                                                                />
                                                                            </svg>
                                                                            "Publish Post"
                                                                        </>
                                                                    }
                                                                }
                                                            }}
                                                        </button>
                                                    </div>
                                                </div>
                                            </form>
                                        }
                                            .into_view()
                                    }
                                }}
                            </>
                        }
                            .into_view()
                    }
                }}
            </div>
        </div>
    }
}

// Helper function to convert markdown code blocks to HTML with Prism classes
fn convert_markdown_to_html(content: &str) -> String {
    use regex::Regex;

    let mut html = content.to_string();

    // Convert markdown code blocks to HTML with Prism classes
    if let Ok(re) = Regex::new(r"```(\w+)\n([\s\S]*?)\n```") {
        html = re
            .replace_all(&html, |caps: &regex::Captures| {
                let language = &caps[1];
                let code = &caps[2];
                format!(
                    "<pre><code class=\"language-{}\">{}</code></pre>",
                    language,
                    html_escape::encode_text(code)
                )
            })
            .to_string();
    }

    // Convert basic markdown elements
    html = html.replace("**", "<strong>").replace("**", "</strong>");
    html = html.replace("*", "<em>").replace("*", "</em>");
    html = html.replace("\n\n", "</p><p>");
    html = format!("<p>{}</p>", html);
    html = html.replace("<p></p>", "");

    html
}
