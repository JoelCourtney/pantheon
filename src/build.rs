use std::process::{Command, Output};
use walkdir::WalkDir;

fn main() {
    compile_client();
    emit_rerun();
}

fn compile_client() -> Output {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "yarn --cwd src\\www\\uikit compile"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("yarn --cwd src/www/uikit compile")
            .output()
            .expect("failed to execute process")
    }
}

fn emit_rerun() {
    println!("cargo:rerun-if-changed=src/build.rs");
    for entry in WalkDir::new("src/www/uikit-custom") {
        let entry = entry.expect("expected dir entry");
        let file_name = entry.file_name().to_str().expect("bad os str");
        if file_name.rfind(".less") != None {
            println!("cargo:rerun-if-changed={}", entry.into_path().to_str().unwrap());
        }
    }
}