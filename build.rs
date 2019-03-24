extern crate cc;

fn main() {
    cc::Build::new()
        .file("libspnav/spnav.c")
        .flag("-fno-strict-aliasing")
        .include("include")
        .compile("spnav");
}
