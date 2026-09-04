fn main() {
    println!("cargo:rustc-link-arg-tests=-Wl,--unresolved-symbols=ignore-all");
}
