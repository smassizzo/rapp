fn main() {
    println!("cargo:warning=generate");
    println!("cargo:warning=build");
}

// CARGO=/Users/developer/.rustup/toolchains/stable-x86_64-apple-darwin/bin/cargo
// CARGO_CRATE_NAME=build_script_build
// CARGO_MANIFEST_DIR=/Users/developer/Projects/rapp/runner
// CARGO_PKG_AUTHORS='Sebastiaan Massizzo'
// CARGO_PKG_DESCRIPTION='A cargo tool to facilitate building mobile apps with rapp'
// CARGO_PKG_HOMEPAGE=''
// CARGO_PKG_LICENSE=''
// CARGO_PKG_LICENSE_FILE=''
// CARGO_PKG_NAME=runner
// CARGO_PKG_REPOSITORY=''
// CARGO_PKG_RUST_VERSION=1.65
// CARGO_PKG_VERSION=0.1.0
// CARGO_PKG_VERSION_MAJOR=0
// CARGO_PKG_VERSION_MINOR=1
// CARGO_PKG_VERSION_PATCH=0
// CARGO_PKG_VERSION_PRE=''
// CARGO_PRIMARY_PACKAGE=1
// DYLD_FALLBACK_LIBRARY_PATH='/Users/developer/Projects/rapp/target/debug/deps:/Users/developer/.rustup/toolchains/stable-x86_64-apple-darwin/lib:/Users/developer/.rustup/toolchains/stable-x86_64-apple-darwin/lib:/Users/developer/lib:/usr/local/lib:/usr/lib'
// /Users/developer/.cargo/bin/sccache rustc
//     --crate-name build_script_build runner/build.rs
//     --error-format=json
//     --json=diagnostic-rendered-ansi,artifacts,future-incompat
//     --diagnostic-width=188
//     --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C metadata=3c4cc075e006ffc2 -C extra-filename=-3c4cc075e006ffc2
//     --out-dir /Users/developer/Projects/rapp/target/debug/build/runner-3c4cc075e006ffc2 -C incremental=/Users/developer/Projects/rapp/target/debug/incremental -L dependency=/Users/developer/Projects/rapp/target/debug/deps`
