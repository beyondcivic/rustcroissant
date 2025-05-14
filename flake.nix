{
  description = "Nix Flake Environment for Rust Development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      
      # Get the complete toolchain with specific components
      rustToolchain = fenix.packages.${system}.complete.withComponents [
        "cargo"
        "clippy"
        "rust-src"
        "rustc"
        "rustfmt"
      ];
      
      # Get the latest rust-analyzer
      rustAnalyzer = fenix.packages.${system}.rust-analyzer;
      
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          # Use fenix rust toolchain instead of nixpkgs ones
          rustToolchain
          rustAnalyzer
          
          # Keep your other tools
          powershell
          jq
          gcc
          gnumake
          zsh
        ];

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
        
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
      };
    });
}