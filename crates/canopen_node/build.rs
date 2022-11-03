macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    p!("initial canopen playground");
    //cc::Build::new()
    //    .file("src/hello.c")
    //    .compile("hello");
    //println!("cargo:rerun-if-changed=src/hello.c");
}