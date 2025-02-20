use std::{future::Future, pin::pin, process::exit, time::Duration};

use anyhow::anyhow;
use hyper::{body::Incoming, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::{
    net::{TcpListener, TcpStream},
    signal::ctrl_c,
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = CancellationToken::new();
    tokio::spawn({
        let token = token.clone();
        async move {
            let _guard = token.drop_guard();
            ctrl_c().await.unwrap();
        }
    });

    let server = tokio::spawn(http_server(token));

    server.await??;

    Ok(())
}

async fn http_server(token: CancellationToken) -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let tracker = TaskTracker::new();

    loop {
        let Some(res) = token.run_until_cancelled(listener.accept()).await else {
            tracker.close();
            break;
        };
        let (stream, _addr) = res?;
        tracker.spawn(conn_handler(stream, token.child_token()));
    }

    drop(listener);

    tracker.wait().await;

    Ok(())
}

async fn conn_handler(stream: TcpStream, token: CancellationToken) -> anyhow::Result<()> {
    let builder = hyper_util::server::conn::auto::Builder::new(TaskExecutor {});
    let mut conn = pin!(builder.serve_connection(
        TokioIo::new(stream),
        service_fn(|req| async { req_handler(req).await }),
    ));

    match token.run_until_cancelled(conn.as_mut()).await {
        Some(res) => res.map_err(|e| anyhow!(e)),
        None => {
            conn.as_mut().graceful_shutdown();
            conn.await.map_err(|e| anyhow!(e))
        }
    }
}

async fn req_handler(req: Request<Incoming>) -> anyhow::Result<Response<String>> {
    println!("received http request at {}", req.uri());

    tokio::time::sleep(Duration::from_secs(5)).await;

    anyhow::Ok(Response::new("hello world\n".to_string()))
}

#[derive(Clone)]
struct TaskExecutor {

}

impl<Fut> hyper::rt::Executor<Fut> for TaskExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send,
{
    fn execute(&self, fut: Fut) {
        tokio::spawn(fut);
    }
}
