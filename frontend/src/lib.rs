use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Blog {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    body: String,
}

#[function_component]
fn Blogkomponentti() -> HtmlResult {
    async fn fetch_blog() -> Blog {
        let blogs: Blog = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        blogs
    }

    let blog = use_prepared_state!(async move |_| -> Blog { fetch_blog().await }, ())?.unwrap();

    Ok(html! {
        <div>
            <h2>
            {"Author "} {blog.id}  {" - 19.02.23"}
            </h2>

            <p>
            {&blog.title}

            {&blog.body}
            </p>

            <p>
            {&blog.title}

            {&blog.body}
            </p>
        </div>
    })
}

#[function_component(App)]
pub fn app() -> Html {
    let fallback = html! { <div>{"Loading..."}</div> };
    return html! {
        <>
        <div>
            <h1>
                {"Blog post title "}
            </h1>
            <Suspense {fallback}>
                <Blogkomponentti />
            </Suspense>

        </div>
        </>
    };
}
