use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;

    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn main() {
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/index");

    let commit = match git(&["rev-parse", "--short=12", "HEAD"]) {
        Some(commit)
            if git(&["status", "--porcelain"]).is_some_and(|status| !status.is_empty()) =>
        {
            format!("{commit}-dirty")
        }
        Some(commit) => commit,
        None => "unknown".to_string(),
    };

    println!("cargo:rustc-env=QUICKSHARE_COMMIT={commit}");
}
