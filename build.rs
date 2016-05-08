

#[cfg(target_os="macos")]
fn main() {
    println!("cargo:rustc-link-search={}/openvr/lib/osx32", env!("CARGO_MANIFEST_DIR"));
}

#[cfg(target_os="linux")]
fn main() {
    println!("cargo:rustc-link-search={}/openvr/lib/linux64", env!("CARGO_MANIFEST_DIR"));
}

#[cfg(target_os="windows")]
fn main() {
    println!("cargo:rustc-link-search={}/openvr/lib/win64", env!("CARGO_MANIFEST_DIR"));
}
