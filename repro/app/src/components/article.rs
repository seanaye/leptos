use leptos::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Story {
    pub id: usize,
    pub title: String,
    pub points: Option<i32>,
    pub user: Option<String>,
    pub time: usize,
    pub time_ago: String,
    #[serde(alias = "type")]
    pub story_type: String,
    pub url: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub comments: Option<Vec<Comment>>,
    pub comments_count: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Comment {
    pub id: usize,
    pub level: usize,
    pub user: Option<String>,
    pub time: usize,
    pub time_ago: String,
    pub content: Option<String>,
    pub comments: Vec<Comment>,
}

pub async fn fetch_api<T>(cx: Scope, path: &str) -> Option<T>
where
    T: Serializable,
{
    let abort_controller = web_sys::AbortController::new().ok();
    // let abort_signal = abort_controller.as_ref().map(|a| a.signal());
    gloo_console::log!("inside fetch call");

    let json = gloo_net::http::Request::get(path)
        // .abort_signal(abort_signal.as_ref())
        .send()
        .await
        .map_err(|e| gloo_console::log!("{e:?}"))
        .map(|v| { gloo_console::log!("{v:?}"); v })
        .ok()?
        .text()
        .await
        .ok()?;

    gloo_console::log!("after fetch call {json:?}");

    // abort in-flight requests if the Scope is disposed
    // i.e., if we've navigated away from this page
    leptos::on_cleanup(cx, move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });
    T::from_json(&json).ok()
}

pub fn story(path: &str) -> String {
    format!("https://node-hnapi.herokuapp.com/{path}")
}

#[component]
pub fn Article(cx: Scope) -> impl IntoView {
    println!("got here");
    let stories = create_resource(
        cx,
        move || (),
        move |()| async move {
            let path = "1";
            fetch_api::<Vec<Story>>(cx, &story(path)).await
        },
    );

    view! { cx,
        <div>
            {move || match stories.read(cx) {
                None => None,
                Some(None) => Some(view! { cx,  <p>"Error loading stories."</p> }.into_any()),
                Some(Some(stories)) => {
                    Some(view! { cx,
                        <ul>
                            <For
                                each=move || stories.clone()
                                key=|story| story.id
                                view=move |cx, story: Story| {
                                    view! { cx,
                                        <div>{story.title}</div>
                                    }
                                }
                            />
                        </ul>
                    }.into_any())
                }
            }}
        </div>
    }
}
