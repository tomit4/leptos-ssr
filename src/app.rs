use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use serde::{Serialize, Deserialize};

use crate::todo::BusyButton;

#[component]
pub fn App() -> impl IntoView {
    // logging::log!("where do I run?"); // Answer: twice on server, once on client

    /*
    let data = if cfg!(target_arch = "wasm32") {
        vec![0, 1, 2]
    } else {
        vec![]
    };

    data.into_iter()
        .map(|value| view! { <span>{value}</span> })
        .collect_view()
    */
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-ssr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    /*
                     * Example of Opting into different SSR Mode
                     * See https://book.leptos.dev/ssr/23_ssr_modes.html
                    <Route
                        path="/post/:id"
                        view=Post
                        ssr=SsrMode::Async
                    />
                    */
                    <Route path="/blog" view=BlogPost/>
                    <Route
                        path="/blog/:id"
                        view=BlogPost
                        ssr=SsrMode::PartiallyBlocked
                    />
                    <Route
                        path="/busy"
                        view=BusyButton
                    />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

// Following the example from https://book.leptos.dev/ssr/23_ssr_modes.html
// Along with help from ChatGPT

#[derive(Serialize, Deserialize, Clone)]
struct BlogPost {
    title: String,
    content: String,
    excerpt: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Comment {
    id: u32,
    author: String,
    content: String,
}

// fn post_id_source() -> u32 {
    // 1
// }

fn comments_source() -> u32 {
    1
}

async fn fetch_post(post_id: u32) -> BlogPost {
    // Simulate fetching different posts based on the `post_id`
    match post_id {
        1 => BlogPost {
            title: "First Blog Post".to_string(),
            content: "This is the content of the first blog post.".to_string(),
            excerpt: "A short excerpt of the first blog post.".to_string(),
        },
        2 => BlogPost {
            title: "Second Blog Post".to_string(),
            content: "This is the content of the second blog post.".to_string(),
            excerpt: "A short excerpt of the second blog post.".to_string(),
        },
        _ => BlogPost {
            title: "Unknown Blog Post".to_string(),
            content: "This blog post does not exist.".to_string(),
            excerpt: "A short excerpt of an unknown blog post.".to_string(),
        },
    }
}

async fn fetch_comments(_post_id: u32) -> Vec<Comment> {
    // Simulated fetch operation
    vec![
        Comment {
            id: 1,
            author: "Alice".to_string(),
            content: "Great post! Thanks for sharing.".to_string(),
        },
        Comment {
            id: 2,
            author: "Bob".to_string(),
            content: "I disagree with some points, but overall good read".to_string(),
        },
        Comment {
            id: 3,
            author: "Charlie".to_string(),
            content: "Very informative. Looking forward to your next post".to_string(),
        },

    ]
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let id = params.with(|params| params.get("id").cloned().unwrap_or_else(|| "0".to_string()));
    let post_id: u32 = id.parse().unwrap_or(0);

    let post_data = create_blocking_resource(move || post_id, fetch_post);
    let comments_data = create_resource(comments_source, fetch_comments);

    view! {
        <Suspense fallback=|| ()>
            {move || {
                         post_data.with(|data| {
                             if let Some(data) = data {
                                 view! {
                                    <Title text=data.title.clone()/>
                                    <Meta name="description" content=data.excerpt.clone()/>
                                    <article>
                                        {data.content.clone()}
                                    </article>
                                 }
                             } else {
                                 view! { <>"Loading..."</> }
                             }
                         })
                     }}
        </Suspense>
        <Suspense fallback=|| ()>
            {move || {
                comments_data.with(|comments| {
                    if let Some(comments) = comments {
                        view! {
                            <section>
                                <h2>"Comments"</h2>
                                {comments.iter().map(|comment| {
                                   view! {
                                       <div>
                                            <strong>{&comment.author}</strong>
                                            <p>{&comment.content}</p>
                                       </div>
                                   }
                                }).collect::<Vec<_>>()}
                            </section>
                        }
                    } else {
                        view! {<section>"Loading comments..."</section>}
                    }
                })
            }}
        </Suspense>
    }
}
