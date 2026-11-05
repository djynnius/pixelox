fn main() {
    let code = pixelox_cli::main_with_args(std::env::args().collect());
    std::process::exit(code);
}
