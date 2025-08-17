{
  rustPlatform,
  clap,
  pkg-config
}:

rustPlatform.buildRustPackage {
  name = "rustcroissant";

  src = ./.;
  buildInputs = [
    clap
  ];

  nativeBuildInputs = [ pkg-config ];
  cargoLock.lockFile = ./Cargo.lock; # FOR DEVELOPMENT
}