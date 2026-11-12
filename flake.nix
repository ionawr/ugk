{
  description = "";

  outputs = {self, ...} @ args: let
    inputs = (import ./.tack) {overrides = args.tackOverrides or {};};

    inherit (inputs.nixpkgs) lib;
    extraLib = import inputs.nixexprs {inherit lib;};

    mkFlake = {
      systems,
      flake ? {},
      perSystem ? (_: {}),
      ...
    }: let
      allSystems = lib.genAttrs systems (system:
        perSystem {
          inherit self inputs system;
          pkgs = import inputs.nixpkgs {inherit system;};
        });
    in
      flake // extraLib.transposeAttrs allSystems;
  in
    mkFlake {
      systems = lib.systems.flakeExposed;
      flake = {};
      perSystem = {
        pkgs,
        self,
        inputs,
        system,
      }: {
        checks = import ./nix/checks.nix {inherit pkgs system inputs;};
        formatter = import ./nix/formatter.nix {inherit pkgs self system;};
        devShells = import ./nix/shell.nix {inherit pkgs self system;};
        packages = rec {
          ugk = pkgs.callPackage ./package.nix {};
          default = ugk;
        };
      };
    };
}
