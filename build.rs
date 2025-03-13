fn main() {
    println!("cargo:rustc-link-lib=static=nauty");
    println!("cargo:rustc-link-search=native=/home/igor/code/rust/nauty-colourings/nauty2_8_9");
}
