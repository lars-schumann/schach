{
  description = "rust dev shell for yew";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          # Rust toolchain with the wasm32-unknown-unknown target
          (rust-bin.nightly.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          })
          rust-analyzer
          rustfmt

          clippy
          bacon
          cargo-hack

          trunk

          nixfmt
          nixd

          nodejs

          samply
        ];
        env = {
          RUST_BACKTRACE = "1";
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };

        shellHook = ''
          export PS1="\[\e[1;33m\]{chess}\[\e[0m\] $PS1"
        '';
      };
    };
}
