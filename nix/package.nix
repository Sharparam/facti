{
  installShellFiles,
  lib,
  rustPlatform,
}:
let
  root = ./..;
  cargoToml = fromTOML (builtins.readFile (root + /Cargo.toml));
  cliToml = fromTOML (builtins.readFile (root + /crates/cli/Cargo.toml));
in
rustPlatform.buildRustPackage (self: {
  inherit (cliToml.package) name version;

  src = root;

  cargoLock.lockFile = (root + /Cargo.lock);
  cargoBuildFlags = [
    "--package"
    "facti"
  ];

  buildFeatures = [
    "ron"
    "sexpr"
    "yaml"
  ];

  useNextest = true;

  strictDeps = true;

  nativeBuildInputs = [ installShellFiles ];

  postInstall = ''
    installShellCompletion \
      --cmd facti \
      --bash <($out/bin/facti --log-level off completion bash) \
      --zsh <($out/bin/facti --log-level off completion zsh) \
      --fish <($out/bin/facti --log-level off completion fish)
  '';

  meta = {
    inherit (cargoToml.workspace.package) homepage;
    inherit (cliToml.package) description;
    license = lib.licenses.mpl20;
    mainProgram = "facti";
  };
})
