#![allow(unused)]
use axum::{body::Body, routing::get};
use errors::AppError;

use http_body_util::StreamBody;
use tokio::{io::simplex, net::TcpListener};

use futures::{SinkExt, TryStreamExt};
use std::pin::pin;
use tokio_util::{
    codec::{FramedRead, FramedWrite},
    io::{ReaderStream, StreamReader},
};

mod codec;
mod errors;
mod slow_api;

use codec::JsonLinesCodec;
use slow_api::PrTitle;

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

async fn req_handler() -> Result<Body, AppError> {
    println!("received http request");

    let prs_resp = slow_api::get_prs().await?;

    let stream = prs_resp
        .into_body()
        // converts Body into Stream<Item = Result<Bytes>>
        .into_data_stream()
        .map_err(std::io::Error::other);

    // convert Stream into AsyncRead
    let reader = StreamReader::new(stream);

    // A channel but via AsyncRead/Write interface
    let (rx, tx) = simplex(1024);

    tokio::spawn(async move {
        // convert AsyncRead into Stream<Item = Result<PrTitle>>
        let mut framderead = pin!(FramedRead::new(reader, JsonLinesCodec::<PrTitle>::new()));

        // convert AsyncWrite into Sink<String>
        let mut tx = FramedWrite::new(tx, JsonLinesCodec::<String>::new());

        // recv a PrTitle
        while let Some(msg) = framderead.as_mut().try_next().await.unwrap() {
            let PrTitle { id, title } = msg;
            let out_msg = format!("{id}: {title}");
            // send our string
            tx.send(out_msg).await.unwrap();
        }

        // close our writer
        tx.close().await.unwrap();
    });

    // convert an AsyncRead into a Stream<Item = Result<Bytes>>
    let rx = ReaderStream::new(rx);

    // convert a Stream<Item = Result<Bytes>> into a Body
    Ok(Body::from_stream(rx))
}
