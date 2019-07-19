let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      stdenv
      figlet

      protobuf
    ];
    shellHook = ''
      figlet "capataz-monitor"
      echo ""
      PATH="$HOME/.cargo/bin:$PATH"
      cargo --version
      rustc --version
    '';
  }
