use std::{error::Error, fs, path::Path, process::Command};
fn run(cmd: &mut Command) -> Result<(), Box<dyn Error>> {
    if !cmd.status()?.success() {
        return Err(format!("Failed: {cmd:?}").into());
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    match std::env::args().nth(1).as_deref() {
        Some("setup") => {}
        _ => {
            println!("cargo xtask setup");
            return Ok(());
        }
    }
    let config = fs::read_to_string(root.join("upstream.toml"))?;
    let value = |key: &str| -> Result<&str, Box<dyn Error>> {
        config
            .lines()
            .find_map(|line| line.strip_prefix(&format!("{key} = \"")))
            .and_then(|line| line.strip_suffix('"'))
            .ok_or_else(|| format!("Missing {key}").into())
    };
    let path = root.join("vendor/JoltPhysics");
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap())?;
        fs::create_dir(&path)?;
        let setup = (|| {
            run(Command::new("git").arg("init").arg(&path))?;
            run(Command::new("git").arg("-C").arg(&path).args([
                "remote",
                "add",
                "origin",
                value("jolt_repository")?,
            ]))?;
            run(Command::new("git").arg("-C").arg(&path).args([
                "fetch",
                "--depth",
                "1",
                "origin",
                value("jolt_revision")?,
            ]))?;
            run(Command::new("git").arg("-C").arg(&path).args([
                "checkout",
                "--detach",
                value("jolt_revision")?,
            ]))
        })();
        if let Err(error) = setup {
            // This invocation created the directory; leave setup retryable.
            fs::remove_dir_all(&path)?;
            return Err(error);
        }
    }
    let rev = Command::new("git")
        .arg("-C")
        .arg(&path)
        .args(["rev-parse", "HEAD"])
        .output()?;
    let status = Command::new("git")
        .arg("-C")
        .arg(&path)
        .args(["status", "--porcelain"])
        .output()?;
    if !rev.status.success()
        || String::from_utf8(rev.stdout)?.trim() != value("jolt_revision")?
        || !status.status.success()
        || !status.stdout.is_empty()
    {
        return Err(
            "Jolt checkout is dirty or not at the pinned revision; setup will not overwrite it"
                .into(),
        );
    }
    println!("Jolt is ready. Run cargo test --workspace.");
    Ok(())
}
