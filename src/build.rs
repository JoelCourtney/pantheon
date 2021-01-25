fn main() {
    let bytes = rand::random::<[u8; 32]>();
    let b64 = base64::encode(&bytes);
    let template = std::fs::read_to_string("src/Rocket.toml.template")
        .expect("read template failed");
    let output = template.replace("%%%SECRET%%%", &b64);
    std::fs::write("Rocket.toml", output).expect("write toml failed");
    println!("cargo:rerun-if-changed=src/Rocket.toml.template");
}
