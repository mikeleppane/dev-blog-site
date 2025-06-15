#![allow(clippy::map_unwrap_or)]
use crate::{
    api::books::get_books_server,
    models::books::{Book, BookCategory},
};
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use std::collections::HashMap;

#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn BooksPage() -> impl IntoView {
    let (selected_category, set_selected_category) = signal(None::<BookCategory>);
    let (search_query, set_search_query) = signal(String::new());

    // Action to fetch books
    let fetch_books_action = Action::new(|(): &()| async move { get_books_server().await });

    // Trigger initial fetch
    Effect::new(move |_| {
        fetch_books_action.dispatch(());
    });

    // Filtered books based on category and search
    let filtered_books = Memo::new(move |_| {
        if let Some(Ok(books)) = fetch_books_action.value().get() {
            let mut filtered: Vec<Book> = books;

            // Filter by category
            if let Some(category) = selected_category.get() {
                filtered.retain(|book| book.category == category);
            }

            // Filter by search query
            let query = search_query.get().to_lowercase();
            if !query.is_empty() {
                filtered.retain(|book| {
                    book.title.to_lowercase().contains(&query)
                        || book.author.to_lowercase().contains(&query)
                        || book.description.to_lowercase().contains(&query)
                        || book
                            .tags
                            .iter()
                            .any(|tag| tag.to_lowercase().contains(&query))
                });
            }

            // Group by category for better display
            let mut grouped: HashMap<BookCategory, Vec<Book>> = HashMap::new();
            for book in filtered {
                grouped.entry(book.category.clone()).or_default().push(book);
            }

            // Sort books within each category by rating (highest first)
            for books in grouped.values_mut() {
                books.sort_by(|a, b| {
                    b.rating
                        .partial_cmp(&a.rating)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }

            grouped
        } else {
            HashMap::new()
        }
    });

    view! {
        <Title text="Book Recommendations - Mike's Dev Blog" />
        <Meta
            name="description"
            content="Curated book recommendations for software developers, covering Rust, Python, system design, and more technical topics"
        />

        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            // Header
            <div class="text-center mb-12">
                <h1 class="text-4xl md:text-5xl font-bold text-gray-900 mb-6">
                    <span class="bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                        "Book Recommendations"
                    </span>
                </h1>
                <p class="text-xl text-gray-600 max-w-3xl mx-auto leading-relaxed">
                    "A curated collection of books that have shaped my understanding of software development,
                    system design, and technology leadership. Each recommendation includes my personal insights 
                    and key takeaways."
                </p>
            </div>

            // Search and Filter Controls
            <div class="mb-8 bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                <div class="flex flex-col lg:flex-row gap-4 items-center">
                    // Search Input
                    <div class="flex-1 w-full lg:w-auto">
                        <div class="relative">
                            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                <svg
                                    class="h-5 w-5 text-gray-400"
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                                    />
                                </svg>
                            </div>
                            <input
                                type="text"
                                class="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors"
                                placeholder="Search books, authors, or topics..."
                                prop:value=move || search_query.get()
                                on:input=move |ev| {
                                    set_search_query.set(event_target_value(&ev));
                                }
                            />
                        </div>
                    </div>

                    // Category Filter
                    <div class="flex flex-wrap gap-2">
                        <button
                            class=move || {
                                format!(
                                    "px-4 py-2 rounded-full text-sm font-medium transition-colors {}",
                                    if selected_category.get().is_none() {
                                        "bg-blue-600 text-white"
                                    } else {
                                        "bg-gray-100 text-gray-700 hover:bg-gray-200"
                                    },
                                )
                            }
                            on:click=move |_| set_selected_category.set(None)
                        >
                            "All Categories"
                        </button>
                        {BookCategory::all_categories()
                            .into_iter()
                            .map(|category| {
                                let category_clone_for_selected_category = category.clone();
                                let category_clone_for_set_selected_category = category.clone();
                                view! {
                                    <button
                                        class=move || {
                                            let category_for_comparison = category_clone_for_selected_category
                                                .clone();
                                            format!(
                                                "px-4 py-2 rounded-full text-sm font-medium transition-colors {}",
                                                if selected_category.get() == Some(category_for_comparison)
                                                {
                                                    "bg-blue-600 text-white"
                                                } else {
                                                    "bg-gray-100 text-gray-700 hover:bg-gray-200"
                                                },
                                            )
                                        }
                                        on:click=move |_| {
                                            set_selected_category
                                                .set(
                                                    Some(category_clone_for_set_selected_category.clone()),
                                                );
                                        }
                                    >
                                        {category.display_name()}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            </div>

            // Loading State
            <Show when=move || fetch_books_action.pending().get()>
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
                        <span class="text-xl text-gray-600">"Loading book recommendations..."</span>
                    </div>
                </div>
            </Show>

            <Show when=move || {
                !fetch_books_action.pending().get() && fetch_books_action.value().get().is_some()
            }>
                {move || {
                    match fetch_books_action.value().get() {
                        Some(Ok(_)) => {
                            let grouped_books = filtered_books.get();
                            if grouped_books.is_empty() {
                                view! {
                                    <div class="text-center py-16">
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
                                                d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.746 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
                                            />
                                        </svg>
                                        <h3 class="mt-2 text-2xl font-medium text-gray-900">
                                            "No books found"
                                        </h3>
                                        <p class="mt-1 text-lg text-gray-500">
                                            "Try adjusting your search or category filter."
                                        </p>
                                    </div>
                                }
                            } else {
                                view! {
                                    <div class="space-y-12">
                                        {grouped_books
                                            .into_iter()
                                            .map(|(category, books)| {
                                                view! {
                                                    <BookCategorySection category=category books=books />
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                }
                            }
                        }
                        Some(Err(err)) => {
                            view! {
                                <div class="text-center py-12">
                                    <div class="bg-red-50 border-l-4 border-red-400 p-6 rounded-md max-w-lg mx-auto">
                                        <h3 class="text-lg font-medium text-red-800">
                                            "Error Loading Books"
                                        </h3>
                                        <p class="mt-2 text-sm text-red-700">
                                            {format!("Failed to load book recommendations: {err}")}
                                        </p>
                                        <button
                                            class="mt-4 inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-red-700 bg-red-100 hover:bg-red-200"
                                            on:click=move |_| {
                                                fetch_books_action.dispatch(());
                                            }
                                        >
                                            "Try Again"
                                        </button>
                                    </div>
                                </div>
                            }
                        }
                        None => {
                            view! {
                                <div class="text-center py-12">
                                    <p class="text-xl text-gray-600">"Initializing..."</p>
                                </div>
                            }
                        }
                    }
                }}
            </Show>
        </div>
    }
}

#[component]
fn BookCategorySection(category: BookCategory, books: Vec<Book>) -> impl IntoView {
    view! {
        <section class="space-y-6">
            <div class="flex items-center space-x-3">
                <h2 class="text-2xl font-bold text-gray-900">{category.display_name()}</h2>
                <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-gray-100 text-gray-800">
                    {books.len()} {if books.len() == 1 { "book" } else { "books" }}
                </span>
            </div>

            <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                {books
                    .into_iter()
                    .map(|book| {
                        view! { <BookCard book=book /> }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

#[component]
fn BookCard(book: Book) -> impl IntoView {
    let rating_stars = (0..5).map(|i| {
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let rating_floor = book.rating.floor().max(0.0) as usize;
        let filled = i < rating_floor;
        let half_filled = i == rating_floor && book.rating.fract() >= 0.5;
        if filled {
            view! {
                <svg class="w-4 h-4 text-yellow-400 fill-current" viewBox="0 0 20 20">
                    <path d="M10 15l-5.878 3.09 1.123-6.545L.489 6.91l6.572-.955L10 0l2.939 5.955 6.572.955-4.756 4.635 1.123 6.545z" />
                </svg>
            }
        } else if half_filled {
            view! {
                <svg class="w-4 h-4 text-yellow-400" viewBox="0 0 20 20">
                    <defs>
                        <linearGradient id="half-fill">
                            <stop offset="50%" stop-color="currentColor" />
                            <stop offset="50%" stop-color="transparent" />
                        </linearGradient>
                    </defs>
                    <path
                        fill="url(#half-fill)"
                        d="M10 15l-5.878 3.09 1.123-6.545L.489 6.91l6.572-.955L10 0l2.939 5.955 6.572.955-4.756 4.635 1.123 6.545z"
                    />
                </svg>
            }
        } else {
            view! {
                <svg class="w-4 h-4 text-gray-300" viewBox="0 0 20 20">
                    <path
                        fill="currentColor"
                        d="M10 15l-5.878 3.09 1.123-6.545L.489 6.91l6.572-.955L10 0l2.939 5.955 6.572.955-4.756 4.635 1.123 6.545z"
                    />
                </svg>
            }
        }
    }).collect_view();

    view! {
        <article class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden hover:shadow-lg transition-all duration-300 transform hover:-translate-y-1">
            <div class="p-6">
                // Book cover and basic info
                <div class="flex space-x-4 mb-4">

                    <div class="flex-1 min-w-0">
                        <h3 class="text-lg font-semibold text-gray-900 mb-1 line-clamp-2">
                            {book.title.clone()}
                        </h3>
                        <p class="text-sm text-gray-600 mb-2">"by " {book.author.clone()}</p>

                        // Rating
                        <div class="flex items-center space-x-2">
                            <div class="flex space-x-1">{rating_stars}</div>
                            <span class="text-sm text-gray-500">
                                {format!("{:.1}", book.rating)}
                            </span>
                        </div>
                    </div>
                </div>

                // Category badge
                <div class="mb-3">
                    <span class=format!(
                        "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {}",
                        book.category.color_class(),
                    )>{book.category.display_name()}</span>
                </div>

                // Description
                <p class="text-sm text-gray-600 mb-4 line-clamp-3">{book.description.clone()}</p>

                // Key takeaways (if any)
                {if book.key_takeaways.is_empty() {
                    view! { <></> }
                    ().into_any()
                } else {
                    view! {
                        <div class="mb-4">
                            <h4 class="text-xs font-medium text-gray-900 uppercase tracking-wide mb-2">
                                "Key Takeaways"
                            </h4>
                            <ul class="text-xs text-gray-600 space-y-1">
                                {book
                                    .key_takeaways
                                    .iter()
                                    .take(3)
                                    .map(|takeaway| {
                                        view! {
                                            <li class="flex items-start">
                                                <span class="text-blue-500 mr-1">"•"</span>
                                                <span class="line-clamp-2">{takeaway.clone()}</span>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        </div>
                    }
                        .into_any()
                }}

                // Tags
                {if book.tags.is_empty() {
                    view! { <></> }
                    ().into_any()
                } else {
                    view! {
                        <div class="mb-4">
                            <div class="flex flex-wrap gap-1">
                                {book
                                    .tags
                                    .iter()
                                    .take(4)
                                    .map(|tag| {
                                        view! {
                                            <span class="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-gray-100 text-gray-800">
                                                {tag.clone()}
                                            </span>
                                        }
                                    })
                                    .collect_view()}
                                {if book.tags.len() > 4 {
                                    view! {
                                        <span class="text-xs text-gray-500">
                                            {format!("+{} more", book.tags.len() - 4)}
                                        </span>
                                    }
                                        .into_any()
                                } else {
                                    view! { <></> }
                                    ().into_any()
                                }}
                            </div>
                        </div>
                    }
                        .into_any()
                }}

                // External links
                <div class="flex items-center space-x-3 text-sm">
                    {book
                        .amazon_url
                        .as_ref()
                        .map(|url| {
                            view! {
                                <a
                                    href=url.clone()
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="text-blue-600 hover:text-blue-800 font-medium transition-colors"
                                >
                                    "Amazon"
                                </a>
                            }
                        })}
                    {book
                        .goodreads_url
                        .as_ref()
                        .map(|url| {
                            view! {
                                <a
                                    href=url.clone()
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="text-blue-600 hover:text-blue-800 font-medium transition-colors"
                                >
                                    "Goodreads"
                                </a>
                            }
                        })}
                </div>
            </div>
        </article>
    }
}
