{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
      naerskLib = pkgs.callPackage naersk {};
    in {

      packages."x86_64-linux".default = naerskLib.buildPackage {
       
        src = ./.;
        buildInputs = [pkgs.clap];
        nativeBuildInputs = [pkgs.pkg-config];

      };

      devShells."x86_64-linux".default = pkgs.mkShell {

        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
          clap
        ];

        nativeBuildInputs = [ pkgs.pkg-config ];

        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        shellHook = ''
          # Check if we are already in a Nix shell
          if [ -z "$IN_NIX_SHELL" ]; then
            echo "Entering Nix Shell"
            exec nix develop
          fi

          # Set Rust/Cargo paths
          export CARGO_HOME=$HOME/.cargo
          export PATH=$CARGO_HOME/bin:$PATH

          # Set CGO flags equivalent for Rust
          export RUSTFLAGS="-C opt-level=2"

          # Set zsh as default shell
          export SHELL=${pkgs.zsh}/bin/zsh

          echo "Rust development environment activated!"
          echo "Rust version: $(rustc --version)"
          echo "Cargo version: $(cargo --version)"
          echo "Rustfmt version: $(rustfmt --version)"
          echo "Clippy version: $(cargo clippy --version)"
          echo "PowerShell version: $(pwsh -Version)"
          echo "jq version: $(jq --version)"
          echo "GCC version: $(gcc --version | head -n 1)"
          echo "Zsh version: $(zsh --version)"

          # Launch zsh
          exec zsh
        '';
      };

    };
}
