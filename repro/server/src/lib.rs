use app::*;
use futures::StreamExt;
use futures_util::TryStreamExt;
use js_sys::Uint8Array;
use leptos::{ssr::render_to_stream, *};
use wasm_bindgen::prelude::*;
use gloo_net::http;

#[wasm_bindgen]
pub async fn main(req: web_sys::Request) -> Result<web_sys::Response, JsError> {
    let req: http::Request = req.into();

    let pkg_path = "/lib/client";
    let head = format!(
        r#"<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <link rel="modulepreload" href="{pkg_path}.generated.js">
                    <link rel="preload" href="{pkg_path}_bg.wasm" as="fetch" type="application/wasm" crossorigin="">
                    <script type="module">
                        import {{ instantiate }} from "{pkg_path}.generated.js"; 
                        const {{ hydrate }} = await instantiate(); 
                        hydrate();
                    </script>
                </head>
                <body>"#
    );

    let tail = "</body></html>";

    type Result<T> = std::result::Result<T, JsValue>;

    let stream = futures::stream::once(async move { head.clone() })
        .chain(render_to_stream(|cx| view! { cx, <App /> }.into_view(cx)))
        .chain(futures::stream::once(async { tail.to_string() }))
        .inspect(|html| println!("{html}"))
        .map(|html| Result::Ok(html.into_bytes()))
        .map_ok(|chunk| {
            let array = Uint8Array::new_with_length(chunk.len() as _);
            array.copy_from(&chunk);

            array.into()
        });

    let js_stream: web_sys::ReadableStream = wasm_streams::ReadableStream::from_stream(stream)
        .into_raw()
        .unchecked_into();

    http::Response::builder()
        .header("Content-Type", "text/html")
        .body(Some(http::ResponseBody::Stream(js_stream)))
        .map_err(|e| JsError::new(e.to_string().as_str()))
        .map(web_sys::Response::from)
}
