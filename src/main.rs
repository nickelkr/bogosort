use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::path::end().map(|| "Bogosort home");

    println!("Starting server at: 0.0.0.0:3121");
    warp::serve(index)
        .run(([0, 0, 0, 0], 3131))
        .await;
}
