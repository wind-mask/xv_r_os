fn main(){
    println!("cargo:rustc-link-arg=-Tsrc/lds/kernel.ld");
    println!("cargo:rerun-if-changed=src/lds/kernel.ld");
}