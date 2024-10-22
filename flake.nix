{
  description = "Anki's alternative for who hates GUI and mouse clicks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustVersion = pkgs.rust-bin.stable.latest.default;
        mem-yaml = with pkgs; rustPlatform.buildRustPackage rec {
          pname = "mem-yaml";
          version = "0.1.0-alpha";
          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-anSf640Ir4yKnsWRZlnTdJhssgm9EbZ8ayQPdtBemOs=";
          };
          cargoHash = "sha256-ikBJUqGmfDheKnxCNrg17pzgUolJLSQaLDXPfwu5zl0=";
          meta = with lib; {
            description = "Anki's alternative for who hates GUI and mouse clicks";
            license = licenses.mit;
            platforms = platforms.all;
            homepage = "https://github.com/haruki-nikaidou/mem-yaml.git";
          };
        };
        mem-yaml-exe = flake-utils.lib.mkApp {
          drv = mem-yaml;
          exePath = "/bin/mem-yaml";
        };
      in
      {
        packages.mem-yaml = mem-yaml;
        packages.default = mem-yaml;
        apps.mem-yaml = mem-yaml-exe;
        apps.default = mem-yaml-exe;
      }
    );
}
