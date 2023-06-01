//! Get GitHub repository information given an `owner` and `repo`.
//!
//! ## Example
//!
//! ```toml
//! [dependencies]
//! gh-repo-info = "0.1"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ```rust,no_run
//! #[tokio::main]
//! async fn main() {
//!     let repo = gh_repo_info::get("rust-lang", "rust").await.unwrap();
//!     println!("{:#?}", repo);
//! }
//! ```
//!
//! ## Blocking
//!
//! ```toml
//! [dependencies]
//! gh-repo-info = { version = "0.1", features = ["blocking"] }
//! ```
//!
//! ```rust,no_run
//! fn main() {
//!     let repo = gh_repo_info::blocking::get("rust-lang", "rust").unwrap();
//!     println!("{:#?}", repo);
//! }
//! ```
//!
//! ## Output
//!
//! ```text
//! GhRepoInfo {
//!     name: "rust",
//!     full_name: "rust-lang/rust",
//!     url: "https://github.com/rust-lang/rust",
//!     owner: GhRepoOwnerInfo {
//!         name: "rust-lang",
//!         url: "https://github.com/rust-lang",
//!         avatar_url: "https://avatars.githubusercontent.com/u/5430905?v=4",
//!         kind: Organization,
//!     },
//!     stargazers_count: 82127,
//!     subscribers_count: 1489,
//!     forks_count: 10830,
//!     open_issues_count: 9549,
//!     is_fork: false,
//!     is_archived: false,
//!     default_branch: "master",
//!     homepage: "https://www.rust-lang.org",
//!     description: "Empowering everyone to build reliable and efficient software.",
//!     license: GhRepoLicenseInfo {
//!         key: "other",
//!         name: "Other",
//!     },
//!     language: "Rust",
//!     topics: [
//!         "compiler",
//!         "hacktoberfest",
//!         "language",
//!         "rust",
//!     ],
//! }
//! ```

#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

use std::error;
use std::fmt;

use reqwest::StatusCode;
use serde::Deserialize;
use urlencoding::encode;

#[derive(Deserialize, Clone, Debug)]
pub struct GhRepoInfo {
    pub name: String,
    pub full_name: String,

    #[serde(rename = "html_url")]
    pub url: String,

    pub owner: GhRepoOwnerInfo,

    pub stargazers_count: usize,
    pub subscribers_count: usize,
    pub forks_count: usize,

    /// Open Issues + Open PRs
    pub open_issues_count: usize,

    #[serde(rename = "fork")]
    pub is_fork: bool,
    #[serde(rename = "archived")]
    pub is_archived: bool,

    pub default_branch: String,

    pub homepage: String,
    pub description: String,
    pub license: GhRepoLicenseInfo,

    pub language: String,
    pub topics: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GhRepoOwnerInfo {
    #[serde(rename = "login")]
    pub name: String,
    #[serde(rename = "html_url")]
    pub url: String,
    pub avatar_url: String,
    #[serde(rename = "type")]
    pub kind: GhRepoOwnerKind,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub enum GhRepoOwnerKind {
    User,
    Organization,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GhRepoLicenseInfo {
    pub key: String,
    pub name: String,
}

/// Get GitHub repository information given an `owner` and `repo`.
pub async fn get(
    owner: impl AsRef<str>,
    repo: impl AsRef<str>,
) -> Result<GhRepoInfo, GhRepoInfoError> {
    let (owner, repo) = (owner.as_ref(), repo.as_ref());
    let url = api_url(owner, repo);

    let resp = reqwest::Client::new()
        .get(url)
        .header("User-Agent", env!("CARGO_PKG_NAME"))
        .send()
        .await
        .map_err(GhRepoInfoError::SendRequest)?;

    let status = resp.status();
    if !status.is_success() {
        return Err(GhRepoInfoError::ResponseNonSuccess(status));
    }

    let repo = resp
        .json::<GhRepoInfo>()
        .await
        .map_err(GhRepoInfoError::DeserializeFailed)?;
    Ok(repo)
}

/// The functionality in `gh_repo_info::blocking` must not be executed
/// within an async runtime, or it will panic when attempting to block.
#[cfg(feature = "blocking")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "blocking")))]
pub mod blocking {
    use super::{api_url, GhRepoInfo, GhRepoInfoError};

    /// Get GitHub repository information given an `owner` and `repo`.
    pub fn get(
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> Result<GhRepoInfo, GhRepoInfoError> {
        let (owner, repo) = (owner.as_ref(), repo.as_ref());
        let url = api_url(owner, repo);

        let resp = reqwest::blocking::Client::new()
            .get(url)
            .header("User-Agent", env!("CARGO_PKG_NAME"))
            .send()
            .map_err(GhRepoInfoError::SendRequest)?;

        let status = resp.status();
        if !status.is_success() {
            return Err(GhRepoInfoError::ResponseNonSuccess(status));
        }

        let repo = resp
            .json::<GhRepoInfo>()
            .map_err(GhRepoInfoError::DeserializeFailed)?;
        Ok(repo)
    }
}

fn api_url(owner: impl AsRef<str>, repo: impl AsRef<str>) -> String {
    let (owner, repo) = (owner.as_ref(), repo.as_ref());
    let owner = encode(owner);
    let repo = encode(repo);
    format!("https://api.github.com/repos/{owner}/{repo}")
}

#[derive(Debug)]
pub enum GhRepoInfoError {
    SendRequest(reqwest::Error),
    ResponseNonSuccess(StatusCode),
    DeserializeFailed(reqwest::Error),
}

impl error::Error for GhRepoInfoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::SendRequest(err) => Some(err),
            Self::ResponseNonSuccess(_code) => None,
            Self::DeserializeFailed(err) => Some(err),
        }
    }
}

impl fmt::Display for GhRepoInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SendRequest(err) => write!(f, "send request failed: {err}"),
            Self::ResponseNonSuccess(code) => write!(f, "response non-successful: {code}"),
            Self::DeserializeFailed(err) => write!(f, "deserialization failed: {err}"),
        }
    }
}
