{ inputs, ... }:
{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];
  perSystem.treefmt = {
    programs = {
      biome = {
        enable = true;
        settings = {
          formatter = {
            useEditorconfig = true;
          };
          json = {
            formatter = {
              expand = "always";
            };
          };
        };
      };
      kdlfmt.enable = true;
      nixfmt.enable = true;
      rustfmt.enable = true;
      shellcheck.enable = true;
      shfmt.enable = true;
    };
  };
}
