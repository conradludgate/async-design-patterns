//! Don't touch this file please :)

use std::time::Duration;

use anyhow::bail;

#[allow(dead_code)]
pub async fn list_pulls() -> anyhow::Result<Vec<u32>> {
    tokio::time::sleep(Duration::from_millis(340)).await;
    Ok(vec![
        136927, 136926, 136924, 136923, 136922, 136921, 136918, 136916, 136915, 136914,
    ])
}

#[allow(dead_code)]
pub async fn get_title(pull: u32) -> anyhow::Result<String> {
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

// github API too agressive with rate limitting :(

// async fn list_pulls() -> anyhow::Result<Vec<u32>> {
//     #[derive(Deserialize)]
//     struct Entry {
//         number: u32,
//     }

//     let client = Client::builder()
//         .user_agent("reqwest-rust-nation-uk-2025")
//         .build()?;
//     let resp = client
//         .get("https://api.github.com/repos/rust-lang/rust/pulls?per_page=10")
//         .header("accept", "application/vnd.github+json")
//         .header("x-github-api-version", "2022-11-28")
//         .send()
//         .await?
//         .error_for_status()?
//         .json::<Vec<Entry>>()
//         .await?;

//     Ok(resp.into_iter().map(|x| x.number).collect())
// }

// async fn get_title(pull: u32) -> anyhow::Result<String> {
//     #[derive(Deserialize)]
//     struct Entry {
//         title: String,
//     }

//     let client = Client::builder()
//         .user_agent("reqwest-rust-nation-uk-2025")
//         .build()?;
//     let resp = client
//         .get(format!(
//             "https://api.github.com/repos/rust-lang/rust/pulls/{pull}"
//         ))
//         .header("accept", "application/vnd.github+json")
//         .header("x-github-api-version", "2022-11-28")
//         .send()
//         .await?
//         .error_for_status()?
//         .json::<Entry>()
//         .await?;

//     Ok(resp.title)
// }
