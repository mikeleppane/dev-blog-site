use leptos::prelude::*;
use regex::Regex;

#[component]
#[allow(clippy::must_use_candidate)]
pub fn BlogContent(content: String) -> impl IntoView {
    let processed_content = Memo::new(move |_| process_blog_content(&content));

    // Trigger Prism.js highlighting after content is rendered
    Effect::new(move |_| {
        let _ = processed_content.get(); // Subscribe to changes

        // Trigger Prism.js highlighting on the client side
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = Prism)]
                fn highlightAll();
            }

            // Small delay to ensure DOM is updated
            set_timeout(
                || {
                    highlightAll();
                },
                std::time::Duration::from_millis(100),
            );
        }
    });

    view! {
        <div
            class="prose prose-lg prose-blue max-w-none
            prose-headings:font-bold prose-headings:tracking-tight
            prose-h1:text-4xl prose-h1:mb-8 prose-h1:mt-12 prose-h1:text-gray-900 
            prose-h1:border-b prose-h1:border-gray-200 prose-h1:pb-4
            prose-h2:text-3xl prose-h2:mb-6 prose-h2:mt-10 prose-h2:text-gray-900 
            prose-h2:border-b prose-h2:border-gray-100 prose-h2:pb-3
            prose-h3:text-2xl prose-h3:mb-4 prose-h3:mt-8 prose-h3:text-gray-900
            prose-h4:text-xl prose-h4:mb-3 prose-h4:mt-6 prose-h4:text-gray-900
            prose-p:text-gray-700 prose-p:leading-relaxed prose-p:mb-6 prose-p:text-lg
            prose-a:text-blue-600 prose-a:hover:text-blue-800 prose-a:transition-colors 
            prose-a:underline
            prose-strong:text-gray-900 prose-strong:font-semibold
            prose-em:text-gray-800 prose-em:italic
            prose-blockquote:border-l-4 prose-blockquote:border-blue-500 
            prose-blockquote:pl-6 prose-blockquote:italic prose-blockquote:bg-blue-50 
            prose-blockquote:py-4 prose-blockquote:rounded-r-lg
            prose-ul:list-disc prose-ul:pl-6 prose-ul:mb-6 prose-ul:space-y-2
            prose-ol:list-decimal prose-ol:pl-6 prose-ol:mb-6 prose-ol:space-y-2
            prose-li:mb-2 prose-li:text-gray-700
            prose-table:table-auto prose-table:w-full prose-table:mb-6
            prose-thead:bg-gray-50
            prose-th:px-4 prose-th:py-2 prose-th:text-left prose-th:font-semibold 
            prose-th:text-gray-900
            prose-td:px-4 prose-td:py-2 prose-td:border-t prose-td:border-gray-200"
            inner_html=move || processed_content.get()
        />
    }
}

fn process_blog_content(content: &str) -> String {
    // Enhanced code block processing for Prism.js
    let enhanced_content = enhance_code_blocks_for_prism(content);

    // Process as HTML (since your blog posts are stored as HTML)
    sanitize_html(&enhanced_content)
}

#[allow(clippy::expect_used)]
fn enhance_code_blocks_for_prism(content: &str) -> String {
    // Pattern to match existing code blocks and enhance them for Prism.js
    let code_block_regex = Regex::new(r#"<pre><code class="language-(\w+)">(.*?)</code></pre>"#)
        .unwrap_or_else(|_| Regex::new(r"a^").expect("Simple regex should never fail"));

    code_block_regex.replace_all(content, |caps: &regex::Captures| {
    let language = caps.get(1).map_or("", |m| m.as_str());
    let code = caps.get(2).map_or("", |m| m.as_str());
    // Decode HTML entities in code
    let decoded_code = html_escape::decode_html_entities(code);
    // Create Prism.js compatible code block
    format!(
         r#"<div class="code-block-wrapper my-8 rounded-lg overflow-hidden shadow-lg border border-gray-200">
                <div class="code-block-header bg-gray-800 text-gray-300 px-4 py-3 text-sm font-medium flex items-center justify-between">
                    <span class="language-label font-semibold">{}</span>
                    <button class="copy-button text-gray-400 hover:text-white transition-colors p-1 rounded hover:bg-gray-700" 
                            onclick="copyCode(this)" title="Copy to clipboard">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                  d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z">
                            </path>
                        </svg>
                    </button>
                </div>
                <pre class="language-{} !mt-0 !rounded-t-none !border-0">
                    <code class="language-{}">{}</code>
                </pre>
            </div>"#,
        get_language_display_name(language),
        language,
        language,
        html_escape::encode_text(&decoded_code)
    )
    }).to_string()
}

fn get_language_display_name(lang: &str) -> &str {
    match lang {
        "rust" => "Rust",
        "python" => "Python",
        "javascript" => "JavaScript",
        "typescript" => "TypeScript",
        "json" => "JSON",
        "yaml" => "YAML",
        "toml" => "TOML",
        "bash" | "shell" => "Shell",
        "html" => "HTML",
        "css" => "CSS",
        "sql" => "SQL",
        _ => "Code",
    }
}

fn sanitize_html(html: &str) -> String {
    // Use ammonia to sanitize HTML while keeping safe tags
    #[cfg(feature = "ssr")]
    {
        use ammonia::Builder;

        Builder::default()
            .add_tags(&["div", "span", "button", "svg", "path"])
            .add_generic_attributes(&[
                "class",
                "onclick",
                "stroke",
                "fill",
                "viewBox",
                "stroke-linecap",
                "stroke-linejoin",
                "stroke-width",
            ])
            .clean(html)
            .to_string()
    }

    #[cfg(not(feature = "ssr"))]
    {
        html.to_string()
    }
}
