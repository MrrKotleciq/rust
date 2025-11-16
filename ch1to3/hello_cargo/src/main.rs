fn main() {
    println!("Hello, world!");
}
// this is hello_cargo file
// important commands:

// cargo new -> creates a new Rust project with this main.rs file as the entry point.
// cargo build -> compiles the project
// cargo build --release -> compiles the project with optimizations for release, but takes longer to compile
// cargo run -> compiles and runs the project in one step
// cargo check -> checks the code for errors without producing an executable, faster than build
// cargo clean -> removes the target directory with compiled files to free up space

// less important commands:

// cargo fmt -> formats the code according to Rust's style guidelines
// cargo clippy -> runs a linter to catch common mistakes and improve code quality
// cargo test -> runs the tests defined in the project
// cargo doc -> generates documentation for the project based on comments in the code
// cargo update -> updates the dependencies listed in Cargo.toml to the latest versions allowed by the