// gh-repo-info = "..."

#[tokio::main]
async fn main() {
    let repo = gh_repo_info::get("rust-lang", "rust").await.unwrap();
    println!("{:#?}", repo);
}
