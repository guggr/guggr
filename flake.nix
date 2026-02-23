{
  description = "Dev Shell for development of guggr";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    nixpkgs-diesel-cli-ext-fix.url = "github:NixOS/nixpkgs/refs/pull/487982/head";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      nixpkgs-diesel-cli-ext-fix,
      rust-overlay,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];

      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);

      nixpkgsFor = forAllSystems (
        system:
        import nixpkgs {
          inherit system;
          overlays = [
            (final: prev: {
              diesel-cli-ext = (import nixpkgs-diesel-cli-ext-fix { inherit system; }).diesel-cli-ext;
            })
            (import rust-overlay)
          ];
        }
      );
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
          rustNightlyToolchain = pkgs.rust-bin.nightly.latest.minimal.override {
            extensions = [ "rustfmt" ];
          };

          rustfmt-nightly = pkgs.symlinkJoin {
            name = "rustfmt-nightly";
            paths = [ rustNightlyToolchain ];
            nativeBuildInputs = [ pkgs.makeWrapper ];
            postBuild = ''
              wrapProgram $out/bin/cargo \
              --set DYLD_LIBRARY_PATH "${rustNightlyToolchain}/lib" \
              --set LD_LIBRARY_PATH "${rustNightlyToolchain}/lib" \
              --set RUSTFMT "${rustNightlyToolchain}/bin/rustfmt" \
              --add-flags "fmt"
              mv $out/bin/cargo $out/bin/rustfmt-nightly
            '';
          };

        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              just
              pnpm
              nodejs_24
              trufflehog
              prek
              kubernetes-helm
              cargo-nextest
              cargo-autoinherit
              cargo-machete
              diesel-cli
              diesel-cli-ext
              libpq
              rust-bin.stable.latest.default
              rustfmt-nightly
              protobuf_33
              podman
            ];
            LD_LIBRARY_PATH = "${pkgs.libpq}/lib";
          };
        }
      );
    };
}
