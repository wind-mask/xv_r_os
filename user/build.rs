fn main() {
    println!("cargo:rustc-link-arg=-Tsrc/lds/user.ld");
    println!("cargo:rerun-if-changed=src/lds/user.ld");
}
