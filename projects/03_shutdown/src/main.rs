use std::{future::Future, pin::pin, time::Duration};

use anyhow::anyhow;
use hyper::{Request, Response, body::Incoming, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = tokio::spawn(http_server());

    server.await??;

    Ok(())
}

async fn http_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _addr) = listener.accept().await?;
        tokio::spawn(conn_handler(stream));
    }
}

async fn conn_handler(stream: TcpStream) -> anyhow::Result<()> {
    let builder = hyper_util::server::conn::auto::Builder::new(TaskExecutor {});
    let conn = pin!(builder.serve_connection(
        TokioIo::new(stream),
        service_fn(|req| async { req_handler(req).await }),
    ));

    conn.await.map_err(|e| anyhow!(e))
}

async fn req_handler(req: Request<Incoming>) -> anyhow::Result<Response<String>> {
    println!("received http request at {}", req.uri());

    tokio::time::sleep(Duration::from_secs(5)).await;

    anyhow::Ok(Response::new("hello world\n".to_string()))
}

#[derive(Clone)]
struct TaskExecutor {}

impl<Fut> hyper::rt::Executor<Fut> for TaskExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send,
{
    fn execute(&self, fut: Fut) {
        tokio::spawn(fut);
    }
}
