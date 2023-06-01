# gh-repo-info

[![Latest Version](https://img.shields.io/crates/v/gh-repo-info.svg)](https://crates.io/crates/gh-repo-info)
[![Docs](https://docs.rs/gh-repo-info/badge.svg)](https://docs.rs/gh-repo-info)
[![License](https://img.shields.io/github/license/vallentin/gh-repo-info.svg)](https://github.com/vallentin/gh-repo-info)

Get GitHub repository information given an `owner` and `repo`.

## Example

```toml
[dependencies]
gh-repo-info = "0.1"
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() {
    let repo = gh_repo_info::get("rust-lang", "rust").await.unwrap();
    println!("{:#?}", repo);
}
```

## Blocking

```toml
[dependencies]
gh-repo-info = { version = "0.1", features = ["blocking"] }
```

```rust
fn main() {
    let repo = gh_repo_info::blocking::get("rust-lang", "rust").unwrap();
    println!("{:#?}", repo);
}
```

## Output

```text
GhRepoInfo {
    name: "rust",
    full_name: "rust-lang/rust",
    url: "https://github.com/rust-lang/rust",
    owner: GhRepoOwnerInfo {
        name: "rust-lang",
        url: "https://github.com/rust-lang",
        avatar_url: "https://avatars.githubusercontent.com/u/5430905?v=4",
        kind: Organization,
    },
    stargazers_count: 82127,
    subscribers_count: 1489,
    forks_count: 10830,
    open_issues_count: 9549,
    is_fork: false,
    is_archived: false,
    default_branch: "master",
    homepage: "https://www.rust-lang.org",
    description: "Empowering everyone to build reliable and efficient software.",
    license: GhRepoLicenseInfo {
        key: "other",
        name: "Other",
    },
    language: "Rust",
    topics: [
        "compiler",
        "hacktoberfest",
        "language",
        "rust",
    ],
}
```
