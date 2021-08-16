fn main() {
    println!(
        "cargo:rustc-env=WASMER_VERSION={}",
        env!("CARGO_PKG_VERSION")
    );
    pyo3_build_config::add_extension_module_link_args();
}
