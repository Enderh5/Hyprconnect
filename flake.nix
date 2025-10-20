{
  description = "Java development environment for Hyprconnect desktop client (with Eclipse JDT LSP and Neovim support)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.openjdk25
            pkgs.maven
            pkgs.jdt-language-server
          ];
          shellHook = ''
            echo "Java development environment activated!"
            echo "OpenJDK: $(java -version 2>&1 | head -n 1)"
            echo "Maven: $(mvn -version | head -n 1)"
            echo "Eclipse JDT LSP: $(ls ${pkgs.jdt-language-server})"
            echo "Neovim: $(nvim --version | head -n 1)"
          '';
        };
      }
    );
}
