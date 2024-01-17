extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/gnostr-get-relays.c")
        .compile("libgnostr-get-relays.a");
}
