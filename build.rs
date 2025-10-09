fn main() {
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=X11");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=Xft");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=cairo")
}