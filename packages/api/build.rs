use std::process::Command;

fn main() {
    let command_output = Command::new(env!("CARGO"))
        .arg("pkgid")
        .arg("--offline")
        .arg("--package")
        .arg("wasmer")
        .output()
        .expect("Failed to execute `cargo` to read package ID.")
        .stdout;
    let wasmer_pkgid = String::from_utf8_lossy(command_output.as_slice());
    let separator_index = wasmer_pkgid
        .rfind(':')
        .expect("Failed to find the version of `wasmer`.");
    let wasmer_version = &wasmer_pkgid[separator_index + 1..];

    println!("cargo:rustc-env=WASMER_VERSION={}", wasmer_version);
}
