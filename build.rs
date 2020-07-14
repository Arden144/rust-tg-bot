fn main() {
	println!("cargo:rustc-flags=-L/home/sincl/rust-tg-bot//lib");
	println!("cargo:rustc-env=LD_LIBRARY_PATH=/home/sincl/rust-tg-bot/lib")
}
