// gh-repo-info = { version = "...", features = ["blocking"] }

fn main() {
    let repo = gh_repo_info::blocking::get("rust-lang", "rust").unwrap();
    println!("{:#?}", repo);
}
