fn main() {
    println!(
        "cargo:rustc-env=WASMER_VERSION={}",
        env!("CARGO_PKG_VERSION")
    );
}
