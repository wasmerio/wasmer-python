use std::process::Command;

fn main() {
    let command_output = Command::new(env!("CARGO"))
        .arg("pkgid")
        .arg("--offline")
        .arg("--package")
        .arg("wasmer-runtime-core")
        .output()
        .expect("Failed to execute `cargo` to read package ID.")
        .stdout;
    let wasmer_runtime_core_pkgid = String::from_utf8_lossy(command_output.as_slice());
    let separator_index = wasmer_runtime_core_pkgid
        .rfind(':')
        .expect("Failed to find the version of `wasmer-runtime-core`.");
    let wasmer_runtime_core_version = &wasmer_runtime_core_pkgid[separator_index + 1..];

    println!(
        "cargo:rustc-env=WASMER_RUNTIME_CORE_VERSION={}",
        wasmer_runtime_core_version
    );
}
