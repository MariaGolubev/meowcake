use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=PROFILE");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let source_dir = manifest_dir.join("templates");
    let output_dir = manifest_dir
        .join("target")
        .join("release")
        .join("templates");

    emit_rerun_if_changed(&source_dir);

    if env::var("PROFILE").ok().as_deref() == Some("release") {
        fs::create_dir_all(&output_dir)
            .expect("build.rs: failed to create target/release/templates");

        minify_templates(&source_dir, &output_dir).expect("build.rs: failed to minify templates");
    }
}

fn emit_rerun_if_changed(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                emit_rerun_if_changed(&path);
            } else if let Some(path_str) = path.to_str() {
                println!("cargo:rerun-if-changed={}", path_str);
            }
        }
    }
}

fn minify_templates(source_dir: &Path, output_dir: &Path) -> io::Result<()> {
    minify_templates_inner(source_dir, source_dir, output_dir)
}

fn minify_templates_inner(
    root_dir: &Path,
    current_dir: &Path,
    output_dir: &Path,
) -> io::Result<()> {
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let rel_path = path
                .strip_prefix(root_dir)
                .expect("build.rs: failed to strip prefix");
            let output_path = output_dir.join(rel_path);

            if path.is_dir() {
                fs::create_dir_all(&output_path)?;
                minify_templates_inner(root_dir, &path, output_dir)?;
                continue;
            }

            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            if path.extension().and_then(|ext| ext.to_str()) == Some("html") {
                let content = fs::read_to_string(&path)?;
                let minified = html_minifier::minify(content)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                fs::write(&output_path, minified)?;
            } else {
                fs::copy(&path, &output_path)?;
            }
        }
    }

    Ok(())
}
