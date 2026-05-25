{
  clippy,
  rustfmt,
  callPackage,
  rust-analyzer,
}: let
  mainPkg = callPackage ./default.nix {};
in
  mainPkg.overrideAttrs (oldAttributes: {
    nativeBuildInputs =
      [
        clippy
        rustfmt
        rust-analyzer
      ]
      ++ (oldAttributes.nativeBuildInputs or []);
  })
