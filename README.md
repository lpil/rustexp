# Rustexp

A Rust regular expression editor and tester. It compiles to web assembly and
is served from from GitHub pages (from the docs directory). There's no
server-side component at all!

```sh
# Setup
rustup install nightly
cargo install -f cargo-web

# Run dev server
cargo web start --auto-reload

# Run tests
cargo test
cargo web test --nodejs

# Build binary
cargo web build
```
