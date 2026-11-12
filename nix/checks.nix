{
  pkgs,
  system,
  inputs,
  ...
}: {
  pre-commit-check = inputs.git-hooks.lib.${system}.run {
    src = ../.;
    hooks = {
      alejandra.enable = true;
      flake-checker.enable = true;
      nil.enable = true;
      deadnix = {
        enable = true;
        settings.noLambdaPatternNames = true;
      };

      # rust
      cargo-check.enable = true;
      cargo-sort.enable = true;
      clippy = {
        enable = true;
        packageOverrides = {
          inherit (pkgs) cargo clippy;
        };
        settings.allFeatures = true;
      };
      rustfmt.enable = true;

      # git
      gitlint.enable = true;

      # secret detection
      ripsecrets.enable = true;
    };

    settings = {
      rust.check.cargoDeps = pkgs.rustPlatform.importCargoLock {
        lockFile = ../Cargo.lock;
      };
    };
  };
}
