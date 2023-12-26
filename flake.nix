{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.naersk.url = "github:nix-community/naersk";
  inputs.cargo-aoc.url = "github:traxys/aoc-tool";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
    rust-overlay,
    cargo-aoc,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      rust = pkgs.rust-bin.stable.latest.default;
      naersk' = pkgs.callPackage naersk {
        cargo = rust;
        rustc = rust;
      };
    in {
      devShell = pkgs.mkShell {
        nativeBuildInputs = [
          rust
          cargo-aoc.defaultPackage.${system}
          pkgs.hyperfine
          pkgs.cargo-flamegraph
          pkgs.z3
        ];
        RUST_PATH = "${rust}";
        RUST_DOC_PATH = "${rust}/share/doc/rust/html/std/index.html";
        AOC_YEAR = "2023";
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        LD_LIBRARY_PATH = "${pkgs.z3.lib}/lib";
        BINDGEN_EXTRA_CLANG_ARGS = let
          inherit (pkgs) lib stdenv;
        in
          "-I${pkgs.z3.dev}/include "
          + "-isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc} -isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc}/${stdenv.hostPlatform.config}";
      };

      defaultPackage = naersk'.buildPackage ./.;
    });
}
