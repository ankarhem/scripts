{
  config,
  lib,
  packages,
  pkgs,
  ...
}:
let
  cfg = config.programs.yt-sub;
in
{
  options.programs.yt-sub = {
    enable = lib.mkEnableOption "yt-sub";
    package = lib.mkOption {
      type = lib.types.package;
      default = packages.${pkgs.system}."yt-sub";
    };
  };

  config = lib.mkIf cfg.enable { environment.systemPackages = [ cfg.package ]; };
}
