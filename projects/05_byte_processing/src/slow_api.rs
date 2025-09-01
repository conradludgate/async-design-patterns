use std::time::Duration;

use anyhow::bail;
use axum::{
    BoxError,
    body::{Body, Bytes},
    http::Response,
};
use futures::{StreamExt, stream};
use serde::{Deserialize, Serialize};

async fn list_pulls() -> anyhow::Result<Vec<u32>> {
    tokio::time::sleep(Duration::from_millis(340)).await;
    Ok(vec![
        136927, 136926, 136924, 136923, 136922, 136921, 136918, 136916, 136915, 136914,
    ])
}

async fn get_title(pull: u32) -> anyhow::Result<String> {
    tokio::time::sleep(Duration::from_millis(180)).await;
    match pull {
        136927 => Ok("Correctly escape hashtags when running invalid_rust_codeblocks lint".into()),
        136926 => Ok("Stabilize -Zdwarf-version as -Cdwarf-version".into()),
        136924 => Ok("Add profiling of bootstrap commands using Chrome events".into()),
        136923 => Ok("Lint #[must_use] attributes applied to methods in trait impls".into()),
        136922 => Ok(
            "Pattern types: Avoid having to handle an Option for range ends in the type system"
                .into(),
        ),
        136921 => Ok("Build GCC on CI".into()),
        136918 => Ok("Rollup of 8 pull requests".into()),
        136916 => Ok("use cc archiver as default in cc2ar".into()),
        136915 => Ok("documentation fix: f16 and f128 are not double-precision".into()),
        136914 => Ok("ci: use ubuntu 24 for free arm runner".into()),
        _ => bail!("invalid pull request"),
    }
}

#[allow(dead_code)]
pub async fn get_prs() -> anyhow::Result<Response<axum::body::Body>> {
    let pulls = list_pulls().await?;

    let stream = stream::iter(pulls).then(|id| async move {
        let title = get_title(id).await?;

        let mut data = serde_json::to_vec(&PrTitle { title, id }).unwrap();
        data.push(b'\n');

        Result::<Bytes, BoxError>::Ok(Bytes::from(data))
    });
    Ok(Response::new(Body::from_stream(stream)))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrTitle {
    pub id: u32,
    pub title: String,
}
