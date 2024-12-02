// build.rs
fn main() {
    // Tell Cargo to link the Foundation framework
    println!("cargo:rustc-link-lib=framework=Foundation");
}
