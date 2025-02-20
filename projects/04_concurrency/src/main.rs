use axum::routing::get;
use errors::AppError;
use tokio::{net::TcpListener, task::JoinSet};

mod errors;
mod slow_api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = tokio::spawn(http_server());

    server.await??;

    Ok(())
}

async fn http_server() -> anyhow::Result<()> {
    let router = axum::Router::new().route("/", get(req_handler));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn req_handler() -> Result<String, AppError> {
    println!("received http request");

    let pulls = slow_api::list_pulls().await?;

    let mut js = JoinSet::<Result<_, anyhow::Error>>::new();

    for (i, pull) in pulls.into_iter().enumerate() {
        js.spawn(async move {
            let title = slow_api::get_title(pull).await?;
            Ok((i, format!("{pull}: {title}\n")))
        });
    }

    let mut results = vec![];
    while let Some(res) = js.join_next().await {
        match res.unwrap() {
            Ok(r) => results.push(r),
            Err(e) => {
                js.abort_all();
                return Err(e.into());
            }
        }
    }
    results.sort_by_key(|(id, _)| *id);
    let results = results.into_iter().map(|(_, s)| s).collect::<Vec<_>>();

    Ok(results.join(""))
}
