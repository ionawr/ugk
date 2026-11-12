{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  pname = "ugk";
  version = "0.1.0";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.fromSource (lib.cleanSource ./.);
  };
  cargoLock.lockFile = ./Cargo.lock;
}
