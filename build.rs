// build.rs
fn main() {
    // opencl
    #[cfg(feature = "opencl")]
    {
        println!("cargo:rustc-link-lib=dylib=OpenCL");
    }

    #[cfg(feature = "cuda")]
    {
        println!("cargo:rustc-link-lib=dylib=cuda");
        println!("cargo:rustc-link-lib=dylib=nvrtc");
    }
}
