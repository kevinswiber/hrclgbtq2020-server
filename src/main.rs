pub(crate) mod generated;
pub(crate) mod schema;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_std::task;
use schema::{HrcLgbtq2020, QueryRoot};
use std::env;
use tide::{http::mime, security::CorsMiddleware, Body, Redirect, Response, StatusCode};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
struct AppState {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

fn main() -> Result<()> {
    task::block_on(run())
}

async fn run() -> Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(HrcLgbtq2020::new())
        .limit_depth(15)
        .finish();

    let mut app = tide::new();

    app.with(CorsMiddleware::new());

    //app.at("/").serve_file("public/index.html")?;
    //app.at("/images/*").serve_dir("public/images/")?;
    //app.at("/js/*").serve_dir("public/js/")?;
    //app.at("/css/*").serve_dir("public/css/")?;

    app.at("/api-explorer").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(playground_source(
            GraphQLPlaygroundConfig::new("/api"),
        )));
        resp.set_content_type(mime::HTML);
        Ok(resp)
    });

    // Moving from /graphql to /api permanently.
    app.at("/graphql").all(Redirect::permanent("/api"));
    app.at("/api").post(async_graphql_tide::endpoint(schema));

    let listen_addr = match env::var("LISTEN_ADDR") {
        Ok(addr) => addr,
        _ => match env::var("PORT") {
            Ok(port) => format!("0.0.0.0:{}", port),
            _ => "localhost:8000".to_string(),
        },
    };

    println!("Playground: http://{}", listen_addr);

    app.listen(listen_addr).await?;

    Ok(())
}
