{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.programs.yt-sub;
in
{
  options.programs.yt-sub = {
    enable = lib.mkEnableOption "yt-sub";
    package = lib.mkPackageOption pkgs "yt-sub" { };
  };

  config = lib.mkIf cfg.enable { home.packages = [ cfg.package ]; };
}
