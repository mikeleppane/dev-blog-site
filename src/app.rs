use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Script, Style, Stylesheet, Title};
use leptos_router::components::A;
use leptos_router::path;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::blog_detail::BlogDetail;
use crate::components::footer::Footer;
use crate::components::navigation::Navigation;
use crate::pages::about::AboutPage;
use crate::pages::blog::BlogPage;
use crate::pages::book::BooksPage;
use crate::pages::home::Home;

#[must_use]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/mike-dev-blog.css" />

        <Title text="Mikko Leppänen - Software Engineer" />
        <Meta charset="utf-8" />
        <Meta
            name="description"
            content="Personal portfolio and tech blog of Mikko Leppänen, Full Stack Developer specializing in Rust, web development, and modern technologies."
        />
        <Meta
            name="keywords"
            content="Mikko Leppänen, Software Developer, Rust, Web Development, Tech Blog"
        />
        <Meta name="author" content="Mikko Leppänen" />

        <Link rel="icon" type_="image/png" sizes="64x64" href="/images/dev-logo-64x64.png" />
        // <Link rel="stylesheet" href="/style/main.css" />
        <Link rel="stylesheet" href="/assets/main.css" />
        <Stylesheet href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" />

        // Prism.js for code highlighting (optional)
        <Stylesheet href="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/themes/prism-tomorrow.min.css" />

        // Prism.js Core
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js" />

        // Prism.js Language Support
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js" />

        // Prism.js Plugins for better experience
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/copy-to-clipboard/prism-copy-to-clipboard.min.js" />
        <Link
            rel="stylesheet"
            href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/copy-to-clipboard/prism-copy-to-clipboard.min.css"
        />

        // Custom styles and JavaScript
        <Style>
            r#"
            /* Ensure headings display properly */
            .blog-content h1,
            .blog-content .heading-1 {
            @apply text-4xl font-bold mb-8 mt-12 text-gray-900 border-b border-gray-200 pb-4;
            display: block !important;
            line-height: 1.2;
            }
            
            .blog-content h2,
            .blog-content .heading-2 {
            @apply text-3xl font-bold mb-6 mt-10 text-gray-900 border-b border-gray-100 pb-3;
            display: block !important;
            line-height: 1.3;
            }
            
            .blog-content h3,
            .blog-content .heading-3 {
            @apply text-2xl font-bold mb-4 mt-8 text-gray-900;
            display: block !important;
            line-height: 1.4;
            }
            
            .blog-content h4,
            .blog-content .heading-4 {
            @apply text-xl font-bold mb-3 mt-6 text-gray-900;
            display: block !important;
            line-height: 1.4;
            }
            
            .blog-content h5,
            .blog-content .heading-5 {
            @apply text-lg font-bold mb-2 mt-4 text-gray-900;
            display: block !important;
            line-height: 1.5;
            }
            
            .blog-content h6,
            .blog-content .heading-6 {
            @apply text-base font-bold mb-2 mt-4 text-gray-900;
            display: block !important;
            line-height: 1.5;
            }
            
            /* Ensure paragraphs have proper spacing */
            .blog-content p {
            @apply text-lg text-gray-700 leading-relaxed mb-6;
            display: block !important;
            }
            
            /* Override any conflicting styles */
            .prose h1, .prose h2, .prose h3, .prose h4, .prose h5, .prose h6 {
            display: block !important;
            margin-top: revert !important;
            margin-bottom: revert !important;
            font-weight: bold !important;
            color: rgb(17 24 39) !important; /* gray-900 */
            }
            
            /* Code block improvements */
            .code-block-wrapper {
            @apply rounded-lg overflow-hidden shadow-lg border border-gray-200 my-8;
            }
            
            .code-block-header {
            font-family: ui-monospace, SFMono-Regular, "SF Mono", Monaco, Inconsolata, 
                         "Roboto Mono", monospace;
            }
            
            .prose pre[class*="language-"] {
            @apply !bg-gray-900 !text-white !p-4 !m-0 !border-0;
            font-family: ui-monospace, SFMono-Regular, "SF Mono", Monaco, Inconsolata, 
                         "Roboto Mono", monospace;
            font-size: 0.875rem;
            line-height: 1.6;
            }
            
            /* Inline code styling */
            .prose :not(pre) > code {
            @apply bg-gray-100 text-gray-800 px-2 py-1 rounded text-sm font-medium;
            font-family: ui-monospace, SFMono-Regular, "SF Mono", Monaco, Inconsolata, 
                         "Roboto Mono", monospace;
            }
            
            /* List styling */
            .blog-content ul {
            @apply list-disc pl-6 mb-6 space-y-2;
            }
            
            .blog-content ol {
            @apply list-decimal pl-6 mb-6 space-y-2;
            }
            
            .blog-content li {
            @apply mb-2 text-gray-700 leading-relaxed;
            }
            
            /* Link styling */
            .blog-content a {
            @apply text-blue-600 hover:text-blue-800 underline transition-colors;
            }
            
            /* Blockquote styling */
            .blog-content blockquote {
            @apply border-l-4 border-blue-500 pl-6 italic bg-blue-50 py-4 rounded-r-lg mb-6;
            }
            
            /* Table styling */
            .blog-content table {
            @apply table-auto w-full mb-6 border-collapse;
            }
            
            .blog-content thead {
            @apply bg-gray-50;
            }
            
            .blog-content th {
            @apply px-4 py-2 text-left font-semibold text-gray-900 border-b border-gray-200;
            }
            
            .blog-content td {
            @apply px-4 py-2 border-b border-gray-100;
            }
            "#
        </Style>
        <Style>
            r#"
            /* Code block styling */
            .code-block-wrapper {
            @apply rounded-lg overflow-hidden shadow-lg;
            }
            
            .code-block-header {
            font-family: ui-monospace, SFMono-Regular, "SF Mono", Monaco, Inconsolata, "Roboto Mono", monospace;
            }
            
            .prose pre[class*="language-"] {
            @apply !bg-gray-900 !text-white !p-4 !m-0;
            font-family: ui-monospace, SFMono-Regular, "SF Mono", Monaco, Inconsolata, "Roboto Mono", monospace;
            font-size: 0.875rem;
            line-height: 1.6;
            border-radius: 0 0 0.5rem 0.5rem;
            }
            
            .prose code[class*="language-"] {
            color: inherit;
            background: none;
            font-family: inherit;
            font-size: inherit;
            text-shadow: none;
            }
            
            .prose :not(pre) > code {
            @apply bg-gray-100 text-gray-800 px-2 py-1 rounded text-sm;
            font-family: ui-monospace, SFMono-Regular, "SF Mono", Monaco, Inconsolata, "Roboto Mono", monospace;
            }
            
            /* Copy button styling */
            .copy-button {
            transition: all 0.2s;
            }
            
            .copy-button:hover {
            transform: scale(1.1);
            }
            
            /* Language label styling */
            .language-label {
            text-transform: uppercase;
            letter-spacing: 0.05em;
            font-weight: 600;
            }
            
            /* Scrollbar styling for code blocks */
            .prose pre::-webkit-scrollbar {
            height: 8px;
            }
            
            .prose pre::-webkit-scrollbar-track {
            background: rgba(255, 255, 255, 0.1);
            border-radius: 4px;
            }
            
            .prose pre::-webkit-scrollbar-thumb {
            background: rgba(255, 255, 255, 0.3);
            border-radius: 4px;
            }
            
            .prose pre::-webkit-scrollbar-thumb:hover {
            background: rgba(255, 255, 255, 0.5);
            }
            "#
        </Style>

        // Copy to clipboard functionality
        <Script>
            r#"
            function copyCode(button) {
            const codeBlock = button.closest('.code-block-wrapper').querySelector('code');
            const text = codeBlock.textContent;
            
            navigator.clipboard.writeText(text).then(function() {
                // Visual feedback
                const originalHTML = button.innerHTML;
                button.innerHTML = '<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>';
                button.classList.add('text-green-400');
                
                setTimeout(() => {
                    button.innerHTML = originalHTML;
                    button.classList.remove('text-green-400');
                }, 2000);
            }).catch(function() {
                console.error('Failed to copy code to clipboard');
            });
            }
            
            // Ensure Prism highlighting runs after page loads
            document.addEventListener('DOMContentLoaded', function() {
            if (typeof Prism !== 'undefined') {
                Prism.highlightAll();
            }
            });
            "#
        </Script>

        <AppRoutes />
    }
}

#[component]
#[allow(clippy::must_use_candidate)]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <div class="route-container min-h-screen bg-gray-50 flex flex-col">
                <Navigation />
                <main>
                    <ErrorBoundary fallback=|errors| {
                        view! {
                            <div class="max-w-4xl mx-auto px-6 py-16 text-center">
                                <h1 class="text-2xl font-bold text-red-600 mb-4">
                                    "Something went wrong"
                                </h1>
                                <div class="text-left bg-red-50 p-4 rounded">
                                    <ul>
                                        {errors
                                            .get()
                                            .into_iter()
                                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                            .collect_view()}
                                    </ul>
                                </div>
                            </div>
                        }
                    }>
                        <Routes fallback=NotFound>
                            // Home page route
                            <Route
                                path=StaticSegment("")
                                view=|| {
                                    view! { <Home /> }
                                }
                            />
                            <Route
                                path=StaticSegment("blog")
                                view=|| {
                                    view! { <BlogPage /> }
                                }
                            />

                            <Route
                                path=path!("/blog/:id")
                                view=|| {
                                    view! { <BlogDetail /> }
                                }
                            />

                            <Route
                                path=StaticSegment("books")
                                view=|| {
                                    view! { <BooksPage /> }
                                }
                            />
                            // About page
                            <Route
                                path=StaticSegment("about")
                                view=|| {
                                    view! { <AboutPage /> }
                                }
                            />
                        </Routes>
                    </ErrorBoundary>
                </main>
                <Footer />
            </div>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    // Set a 404 status code
    #[cfg(feature = "ssr")]
    {
        use http::StatusCode;
        use leptos::prelude::*;
        let response = expect_context::<leptos_axum::ResponseOptions>();
        response.set_status(StatusCode::NOT_FOUND);
    }

    view! {
        <Title text="Page Not Found - 404" />
        <div class="flex items-center justify-center min-h-[60vh] py-12 px-4 sm:px-6 lg:px-8">
            <div class="text-center">
                <h1 class="text-6xl font-extrabold text-blue-600 mb-6">404</h1>
                <h2 class="text-3xl font-bold text-gray-900 mb-4">Page not found</h2>
                <p class="text-xl text-gray-600 mb-8">
                    "Sorry, we couldn't find the page you're looking for."
                </p>
                <div>
                    <A
                        href="/"
                        attr:class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 transition-colors duration-200"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5 mr-2"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M9.707 14.707a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 1.414L7.414 9H15a1 1 0 110 2H7.414l2.293 2.293a1 1 0 010 1.414z"
                                clip-rule="evenodd"
                            />
                        </svg>
                        "Back to Home"
                    </A>
                </div>
            </div>
        </div>
    }
}
