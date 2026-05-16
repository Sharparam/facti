{
  perSystem =
    {
      self',
      craneLib,
      pkgs,
      ...
    }:
    {
      devShells.default = craneLib.devShell {
        inputsFrom = builtins.attrValues self'.checks;

        packages = [
          pkgs.rust-analyzer
        ];
      };
    };
}
