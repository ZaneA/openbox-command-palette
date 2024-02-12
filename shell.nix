with (import <nixpkgs> {
  overlays = [];
});
mkShell {
  buildInputs = [
    cargo rustc stdenv cmake
  ];

  shellHook = ''
  '';
}
