/// Helper that provides common code used in build.rs service files.
use std::fs::{self, DirBuilder, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

// Look for all the *.rs file and subdirectories in this directory
//  and add them to the local mod.rs
fn generate_mod(dir: &str) -> Result<(), ::std::io::Error> {
    let path = Path::new(dir);
    if !path.is_dir() {
        return Ok(());
    }
    let mut items = vec![];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(p) = path.file_name() {
                items.push(p.to_str().unwrap().to_owned());
            }
        }

        if let Some(ext) = path.extension() {
            if ext == "rs" && !path.ends_with("mod.rs") && !path.ends_with("gecko_client.rs") {
                if let Some(p) = path.file_stem() {
                    items.push(p.to_str().unwrap().to_owned());
                }
            }
        }
    }

    let mut mod_file = File::create(path.join("mod.rs"))?;
    for item in items {
        mod_file.write_fmt(format_args!("pub mod {};\n", item))?;
    }

    Ok(())
}

/// Builds a service whose SIDL description is at $service/src/$sidl_name.sidl
/// Allows callers to pass a custom codegen configuration.
pub fn build_service_with_config(sidl_name: &str) {
    // Pick up changes in the codegen.
    println!("cargo:rerun-if-changed=../../codegen/src");
    println!("cargo:rerun-if-changed=../../codegen/src/templates");

    let sidl_path = format!("src/{}.sidl", sidl_name);
    println!("cargo:rerun-if-changed={}", sidl_path);

    // 1. Re-create the src/generated and client/generated directories if needed.
    let _ = DirBuilder::new().recursive(true).create("src/generated");
    let _ = DirBuilder::new().recursive(true).create("client/generated");

    // 2. Generate Rust code from the sidl representation.
    sidl_codegen::generate_rust_service(Path::new(&sidl_path), Path::new("src/generated/"))
        .expect("Generating Rust service");

    // 3. Generate the Rust module for src/generated
    if let Err(err) = generate_mod("src/generated") {
        panic!("Failed to create mod.rs : {}", err);
    }

    // 4. Generate the Javascript client from the sidl representation.
    sidl_codegen::generate_javascript_code(
        Path::new(&sidl_path),
        Path::new("client/generated/service.js"),
    )
    .expect("Generating Javascript client.");

    // 5. Generate the Javascript documentation.
    sidl_codegen::generate_javascript_doc(
        Path::new(&sidl_path),
        Path::new(&format!("client/generated/{}_service.md", sidl_name)),
        sidl_name,
    )
    .expect("Generating Javascript documentation.");
}

/// Builds a service whose SIDL description is at $service/src/$sidl_name.sidl
pub fn build_service(sidl_name: &str) {
    build_service_with_config(sidl_name)
}
