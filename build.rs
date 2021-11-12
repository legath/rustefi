fn main() {
    println!("cargo:rerun-if-changed=bsp.ld");
    println!("cargo:rerun-if-changed=rm46l852.ld");
}
