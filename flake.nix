{
  description = "DevShell con fenix y cargo-aoc";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    fenix.url = "github:nix-community/fenix";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
    }:
    let
      system = "x86_64-linux"; # cámbialo si usas otra arquitectura
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          (fenix.packages.${system}.stable.toolchain)
          pkgs.clippy
        ];
      };
    };
}
