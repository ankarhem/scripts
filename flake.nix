{
  description = "A flake with a bunch of scripts";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1";
    naersk.url = "github:nix-community/naersk";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks.url = "github:cachix/git-hooks.nix";
  };

  outputs =
    inputs@{ nixpkgs, ... }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          f rec {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                (import inputs.rust-overlay)
                (self: super: {
                  rustToolchain = (super.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
                    extensions = [
                      "rust-analyzer"
                      "rust-src"
                      "rustfmt"
                    ];
                  };
                })
              ];
            };
            naerskLib = pkgs.callPackage inputs.naersk {
              cargo = pkgs.rustToolchain;
              rustc = pkgs.rustToolchain;
            };
            pre-commit-hooks = inputs.git-hooks.lib.${system}.run {
              src = ./.;
              hooks = {
                ripsecrets.enable = true;

                nixfmt-rfc-style.enable = true;
                cargo-check = {
                  enable = false;
                  package = pkgs.rustToolchain;
                };
                rustfmt = {
                  enable = true;
                  packageOverrides = {
                    cargo = pkgs.rustToolchain;
                  };
                };
                clippy = {
                  enable = true;
                  packageOverrides = {
                    cargo = pkgs.rustToolchain;
                    clippy = pkgs.rustToolchain;
                  };
                };
              };
            };
          }
        );

      modules = import ./modules { lib = nixpkgs.lib; };
    in
    {
      packages = forEachSupportedSystem (
        { pkgs, naerskLib, ... }:
        {
          yt-sub = naerskLib.buildPackage {
            pname = "yt-sub";
            src = ./.;
          };
          summarize = naerskLib.buildPackage {
            pname = "summarize";
            src = ./.;
          };
        }
      );

      formatter = forEachSupportedSystem (
        { pkgs, pre-commit-hooks, ... }:
        let
          config = pre-commit-hooks.config;
          inherit (config) package configFile;
          script = ''
            ${pkgs.lib.getExe package} run --all-files --config ${configFile}
          '';
        in
        pkgs.writeShellScriptBin "pre-commit-run" script
      );

      devShells = forEachSupportedSystem (
        { pkgs, pre-commit-hooks, ... }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              cargo-deny
              cargo-edit
              cargo-watch
              cargo-insta
              rust-analyzer
            ];

            buildInputs = [ pre-commit-hooks.enabledPackages ];
            shellHook = ''
              ${pre-commit-hooks.shellHook}
            '';

            env = {
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };
          };
        }
      );

      nixosModules.default = import ./modules/nixos.nix;
      homeManagerModules.default = import ./modules/home.nix;
    };
}
