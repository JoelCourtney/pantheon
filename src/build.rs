use std::process::{Command, Output};
use walkdir::WalkDir;
use std::fs::File;
use std::io::Write;

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
        let out = Command::new("sh")
            .arg("-c")
            .arg("yarn --cwd src/www/uikit compile")
            .output()
            .expect("failed to execute process");
        let path = "build.txt";
        let mut output = File::create(path).unwrap();
        write!(output, "{}", out.stdout.iter().map(|c| *c as char).collect::<String>()).unwrap();
        out
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