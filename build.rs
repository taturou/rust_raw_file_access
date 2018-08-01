extern crate gcc;

fn main() {
    gcc::Config::new()
                .file("src/c/rawopen.c")
                .include("src")
                .compile("librawopen.a");
}
