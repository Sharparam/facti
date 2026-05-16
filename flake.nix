{
  description = "Factorio mod tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    inputs@{
      advisory-db,
      crane,
      flake-parts,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      inherit (flake-parts.lib) mkFlake;
      systems = lib.systems.flakeExposed;
      fs = lib.fileset;
    in
    mkFlake { inherit inputs; } {
      inherit systems;
      imports = [
        flake-parts.flakeModules.easyOverlay
        ./nix/treefmt.nix
        ./nix/devshell.nix
      ];
      perSystem =
        {
          config,
          pkgs,
          system,
          ...
        }:
        let
          craneLib = (crane.mkLib pkgs).overrideToolchain (
            p:
            p.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            }
          );
          root = ./.;
          # src = craneLib.cleanCargoSource ./.;
          src = fs.toSource {
            inherit root;
            fileset = fs.unions [
              (craneLib.fileset.commonCargoSources root)
              (fs.fileFilter (file: file.hasExt "pest") root)
              (fs.fileFilter (file: file.hasExt "md") root)
              (fs.fileFilter (file: file.hasExt "txt") ./crates/lib)
            ];
          };
          commonArgs = {
            inherit src;
            strictDeps = true;
            nativeBuildInputs = builtins.attrValues { inherit (pkgs) pkg-config; };
          };
          workspaceArgs = {
            pname = "facti-workspace";
            version = "0.1.0";
          };
          cargoArtifacts = craneLib.buildDepsOnly (commonArgs // workspaceArgs);
          individualCrateArgs = commonArgs // {
            inherit cargoArtifacts;
            doCheck = false;
          };
          facti = craneLib.buildPackage (
            individualCrateArgs
            // {
              inherit (craneLib.crateNameFromCargoToml { src = ./crates/cli; }) version;
              pname = "facti";
              cargoExtraArgs = "--package facti";
              nativeBuildInputs = [ pkgs.installShellFiles ];
              buildInputs = [ pkgs.cacert ];

              outputs = [
                "out"
                "dev"
                "man"
              ];

              separateDebugInfo = true;

              CARGO_BUILD_RUSTFLAGS = "-g";

              postBuild = ''
                cargo xtask man
              '';

              postInstall = ''
                installShellCompletion \
                  --cmd facti \
                  --bash <($out/bin/facti --log-level off completion bash) \
                  --zsh <($out/bin/facti --log-level off completion zsh) \
                  --fish <($out/bin/facti --log-level off completion fish)

                installManPage \
                  target/assets/man/*
              '';
            }
          );
        in
        {
          _module.args = {
            inherit craneLib;
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            };
          };

          packages = {
            inherit facti;
            default = config.packages.facti;
          };

          checks = {
            inherit facti;

            clippy = craneLib.cargoClippy (
              commonArgs
              // workspaceArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets -- --deny warnings";
              }
            );

            doc = craneLib.cargoDoc (
              commonArgs
              // workspaceArgs
              // {
                inherit cargoArtifacts;
                cargoDocExtraArgs = "--document-private-items";
                env.RUSTDOCFLAGS = "--deny warnings";
              }
            );

            docTest = craneLib.cargoDocTest (
              commonArgs
              // workspaceArgs
              // {
                inherit cargoArtifacts;

                nativeBuildInputs = [ pkgs.cacert ];
              }
            );

            audit = craneLib.cargoAudit (workspaceArgs // { inherit src advisory-db; });

            deny = craneLib.cargoDeny (workspaceArgs // { inherit src; });

            nextest = craneLib.cargoNextest (
              commonArgs
              // workspaceArgs
              // {
                inherit cargoArtifacts;
                partitions = 1;
                partitionType = "count";
                cargoNextestPartitionsExtraArgs = "--no-tests=pass";
              }
            );
          };

          overlayAttrs = {
            inherit (config.packages) facti;
          };
        };
    };
}
