use std::sync::Arc;

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use warp::Filter;

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

#[tokio::main]
async fn main() {
    let mut hb = Handlebars::new();
    hb.register_template_file("index.html", "templates/index.html")
        .unwrap();

    let hb = Arc::new(hb);

    let handlebars = move |with_template| render(with_template, hb.clone());

    let routes = warp::get()
        .and(warp::path::end())
        .map(|| WithTemplate {
            name: "index.html",
            value: json!({"value": "templated"}),
        })
        .map(handlebars);

    println!("Starting server at: 0.0.0.0:3131");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3131))
        .await;
}
