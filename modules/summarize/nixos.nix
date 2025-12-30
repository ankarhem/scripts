{
  config,
  lib,
  packages,
  pkgs,
  ...
}:
let
  cfg = config.programs.summarize;

  wrappedPkg = pkgs.writeShellApplication {
    name = "summarize";
    text = ''
      set -euo pipefail

      ${lib.optionalString (cfg.envFile != null) ''
        set -a
        source ${cfg.envFile}
        set +a
      ''}

      exec ${cfg.package} "$@"
    '';
  };
in
{
  options.programs.summarize = {
    enable = lib.mkEnableOption "summarize";
    package = lib.mkOption {
      type = lib.types.package;
      default = packages.${pkgs.system}.summarize;
    };
    environmentsFile = lib.mkOption {
      type = lib.types.nullOr lib.types.path;
      default = null;
      description = "Environment file containing `ANTHROPIC_AUTH_TOKEN`";
    };
  };

  config = lib.mkIf cfg.enable { environment.systemPackages = [ wrappedPkg ]; };
}
