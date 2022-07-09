{ pkgs }:
with pkgs;
stdenv.mkDerivation {
  name = "devshell";
  buildInputs = [
    cargo-readme
    # Build tools
    rustup

    rust-analyzer
    glibc
    cmake
    clang
    pkg-config
    openssl

    git
  ] ++ (lib.optionals stdenv.isDarwin ([
    libiconv
  ] ++ (with darwin.apple_sdk.frameworks; [
    DiskArbitration
    Foundation
  ])));
}
