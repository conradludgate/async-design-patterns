#![allow(unused)]
use anyhow::Context;
use axum::{body::Body, routing::get};
use errors::AppError;

use http_body_util::BodyExt;
use serde_json::Deserializer;
use tokio::net::TcpListener;

use futures::{SinkExt, TryStreamExt};
use std::{io::BufRead, pin::pin};
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

    // TODO:
    // This currently waits until the full response is loaded from slow_api::get_prs.
    // Can we stream out the responses instead?

    let data = prs_resp.into_body().collect().await?.to_bytes();

    let mut output = String::new();
    for pr_title in data.lines() {
        let pr_title = pr_title.context("missing full json line")?;
        let PrTitle { id, title } = serde_json::from_str(&pr_title)?;

        output += &format!("{id}: {title}\n");
    }

    Ok(Body::new(output))
}
