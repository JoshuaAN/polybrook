use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

const IR_DIR: &str = "tests/ir";

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

// Recursively collects '.ir' files as paths relative to the crate root.
fn collect(rel: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(root().join(rel))? {
        let entry = entry?;
        let path = rel.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            collect(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "ir") {
            out.push(path);
        }
    }
    Ok(())
}

fn ir_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect(Path::new(IR_DIR), &mut files)
        .unwrap_or_else(|e| panic!("reading {IR_DIR}: {e}"));
    if let Ok(filter) = env::var("IR_FILTER") {
        files.retain(|p| p.to_string_lossy().contains(&filter));
    }
    files.sort();
    assert!(!files.is_empty(), "no .ir files found under {IR_DIR}");
    files
}

#[test]
fn one_invocation() {
    for file in ir_files() {
        println!("Test: {}", file.display());
    }
}
