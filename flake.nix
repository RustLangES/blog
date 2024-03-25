{
  description = "A Blog";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    #pkgs = nixpkgs.legacyPackages.${system};
    pkgs = import nixpkgs {
      inherit system overlays;
    };
    rust = pkgs.buildPackages.rust-bin.stable.latest.minimal;
    nativeBuildInputs = with pkgs; [
      rust
      leptosfmt
      cargo-watch
      miniserve
      openssl
    ];
    buildInputs = with pkgs; [
      pkg-config
    ];
  in {
    formatter.x86_64-linux = nixpkgs.legacyPackages.${system}.alejandra;
    environment.variables = {
      PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    };
    devShells.${system}.default = pkgs.mkShell {
      inherit nativeBuildInputs buildInputs;
      shellHook = ''mkdir -p out
      echo "Bienvenido al Blog."
      echo -e 'puede usar los comandos:
                \x1b[93m#[Para compilar y ejecutar el servidor web local]\x1b[0m
                  cargo watch -x run --shell "npx tailwindcss -i ./input.css -o ./out/output.css && cargo run" &
                \x1b[93m#[Para ejecutar o correr los archivos estáticos de tu sitio web localmente]\x1b[0m
                   miniserve out --index index.html'
                    '';
    };
  };
}
