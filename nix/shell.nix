{
  pkgs,
  self,
  system,
  ...
}: let
  inherit (self.checks.${system}.pre-commit-check) shellHook enabledPackages;
in {
  default = pkgs.mkShellNoCC {
    inherit shellHook;

    env.RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

    packages = with pkgs; [
      prek
    ];

    buildInputs = enabledPackages;

    nativeBuildInputs = with pkgs; [
      cargo
      clippy
      cargo-edit
      rustc
      rustfmt
      rust-analyzer
    ];
  };
}
