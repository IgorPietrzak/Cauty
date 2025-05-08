fn main() {
    println!("cargo:rustc-link-lib=static=nauty");
    println!("cargo:rustc-link-search=native=/home/igor/code/rust/Cauty/nauty2_8_9");
}
