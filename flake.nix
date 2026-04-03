{
  description = "Personal Website + Resume PDF Development Environment and Automation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Build WASM Simulations
        buildSimulations = pkgs.writeShellScriptBin "build-simulations" ''
          echo ">> Building Rust Simulations..."
          if [ -d "simulations" ]; then
            cd simulations

            # Compile Rust to WASM
            cargo build --release --target wasm32-unknown-unknown

            # Bindgen: Generate the JS glue code
            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              --out-dir ../static/wasm \
              --target web \
              --no-typescript \
              target/wasm32-unknown-unknown/release/spinning_cube.wasm

              cd ..
          else 
            echo "No 'simulations' folder found, skipping WASM build."
          fi
        '';

        # Build Resume
        buildResume = pkgs.writeShellScriptBin "build-resume" ''
          echo ">> Building Resume..."
          TYPST_FONT_PATHS="${pkgs.font-awesome}/share/fonts" ${pkgs.typst}/bin/typst compile --root . \
            --input RESUME_NAME="''${RESUME_NAME:-}" \
            --input RESUME_EMAIL="''${RESUME_EMAIL:-}" \
            --input RESUME_PHONE="''${RESUME_PHONE:-}" \
            resume/resume.typ static/resume.pdf
        '';

        # Build Zola Site
        buildZola = pkgs.writeShellScriptBin "build-zola" ''
          echo ">> Building Zola Site..."
          ${pkgs.zola}/bin/zola build
        '';

        # Default build script (runs all)
        buildSite = pkgs.writeShellScriptBin "build-site" ''
          ${buildSimulations}/bin/build-simulations
          ${buildResume}/bin/build-resume
          ${buildZola}/bin/build-zola
        '';

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [

            # Website
            zola

            # Resume
            typst
            tinymist
            typstyle

            # Rust
            rustToolchain
            wasm-pack
            wasm-bindgen-cli
            binaryen
            # Bevy dependencies
            pkg-config
            alsa-lib
            udev
            vulkan-loader
            libx11
            libxcursor
            libxi
            libxrandr
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
            with pkgs;
            [
              alsa-lib
              udev
              vulkan-loader
              libx11
              libxcursor
              libxi
              libxrandr
            ]
          );

          TYPST_FONT_PATHS = "${pkgs.font-awesome}/share/fonts";

          shellHook = ''
            echo "----------------------------------------------------"
            echo "Tools loaded: Zola, Rust (w/ WASM), wasm-pack, Typst"
            echo "----------------------------------------------------"
          '';
        };

        apps = {
          simulations = flake-utils.lib.mkApp {
            drv = buildSimulations;
          };
          resume = flake-utils.lib.mkApp {
            drv = buildResume;
          };
          zola = flake-utils.lib.mkApp {
            drv = buildZola;
          };
          default = flake-utils.lib.mkApp {
            drv = buildSite;
          };
        };
      }
    );
}
