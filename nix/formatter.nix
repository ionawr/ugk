{
  pkgs,
  self,
  system,
  ...
}: let
  config = self.checks.${system}.pre-commit-check.config;
  inherit (config) package configFile;
in
  pkgs.writeShellScriptBin "pre-commit-run"
  # bash
  ''
    ${pkgs.lib.getExe package} run --all-files --config ${configFile}
  ''
