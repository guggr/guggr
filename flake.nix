{
  description = "Dev Shell for development of guggr";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
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
          rustStableToolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
          };
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
              rustStableToolchain
              rustfmt-nightly
              protobuf_33
              podman
              docker-compose
            ];
            LD_LIBRARY_PATH = "${pkgs.libpq}/lib";
          };
        }
      );
    };
}
