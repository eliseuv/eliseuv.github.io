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
          set -euo pipefail
          # Outside a `nix develop` shell (e.g. plain `nix run` in CI) nothing
          # puts the pinned toolchain on PATH, so a bare `cargo` falls back to
          # whatever the runner happens to have, which lacks the
          # wasm32-unknown-unknown std. cargo also shells out to `rustc` by
          # bare name internally, so it must be on PATH too, not just cargo.
          export PATH="${rustToolchain}/bin:$PATH"
          echo ">> Building Rust Simulations..."
          if [ -d "simulations" ]; then
            cd simulations

            # Compile every workspace member in one pass.
            cargo build --release --target wasm32-unknown-unknown

            # Bindgen: Generate the JS glue code for each simulation.
            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              --out-dir ../static/wasm \
              --target web \
              --no-typescript \
              target/wasm32-unknown-unknown/release/game_of_life.wasm

            # Crates backed by artificial-systems carry rand/std overhead the
            # dependency-free ones don't; -Oz trims ~30% of it.
            ${pkgs.binaryen}/bin/wasm-opt -Oz \
              -o ../static/wasm/game_of_life_bg.wasm \
              ../static/wasm/game_of_life_bg.wasm

            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              --out-dir ../static/wasm \
              --target web \
              --no-typescript \
              target/wasm32-unknown-unknown/release/ising_2d.wasm

            ${pkgs.binaryen}/bin/wasm-opt -Oz \
              -o ../static/wasm/ising_2d_bg.wasm \
              ../static/wasm/ising_2d_bg.wasm

            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              --out-dir ../static/wasm \
              --target web \
              --no-typescript \
              target/wasm32-unknown-unknown/release/contact_process.wasm

            ${pkgs.binaryen}/bin/wasm-opt -Oz \
              -o ../static/wasm/contact_process_bg.wasm \
              ../static/wasm/contact_process_bg.wasm

            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              --out-dir ../static/wasm \
              --target web \
              --no-typescript \
              target/wasm32-unknown-unknown/release/chiarella.wasm

            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              --out-dir ../static/wasm \
              --target web \
              --no-typescript \
              target/wasm32-unknown-unknown/release/tsp_annealing.wasm

            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              --out-dir ../static/wasm \
              --target web \
              --no-typescript \
              target/wasm32-unknown-unknown/release/spectral_criticality.wasm

            ${pkgs.binaryen}/bin/wasm-opt -Oz \
              -o ../static/wasm/spectral_criticality_bg.wasm \
              ../static/wasm/spectral_criticality_bg.wasm

              cd ..
          else
            echo "No 'simulations' folder found, skipping WASM build."
          fi
        '';

        # Build Resume
        buildResume = pkgs.writeShellScriptBin "build-resume" ''
          set -euo pipefail
          echo ">> Building Resume..."
          TYPST_FONT_PATHS="${pkgs.font-awesome}/share/fonts" ${pkgs.typst}/bin/typst compile --root . \
            --input RESUME_PHONE="''${RESUME_PHONE:-}" \
            resume/resume.typ static/resume.pdf
        '';

        # Render the GitHub profile README from the resume data. Kept out of
        # build-site: it is published to eliseuv/eliseuv, not to Cloudflare.
        buildReadme = pkgs.writeShellScriptBin "build-readme" ''
          set -euo pipefail
          echo ">> Building Profile README..."
          mkdir -p public
          ${pkgs.minijinja}/bin/minijinja-cli --strict \
            readme/README.md.j2 resume/content.yml -o public/README.md
        '';

        # Build Zola Site
        buildZola = pkgs.writeShellScriptBin "build-zola" ''
          set -euo pipefail
          echo ">> Building Zola Site..."
          ${pkgs.zola}/bin/zola build
        '';

        # Deploy the built site to Cloudflare Pages via direct upload.
        # Building stays in Nix rather than Cloudflare's Git integration, whose
        # build image has no Nix. Auth comes from CLOUDFLARE_API_TOKEN and
        # CLOUDFLARE_ACCOUNT_ID; the target branch is detected from git, so
        # only deploys from main land in production.
        deploySite = pkgs.writeShellScriptBin "deploy-site" ''
          set -euo pipefail
          echo ">> Deploying to Cloudflare Pages..."
          ${pkgs.wrangler}/bin/wrangler pages deploy public \
            --project-name "''${CLOUDFLARE_PAGES_PROJECT:-eliseuv}" \
            "$@"
        '';

        # Default build script (runs all)
        buildSite = pkgs.writeShellScriptBin "build-site" ''
          set -euo pipefail
          ${buildSimulations}/bin/build-simulations
          ${buildResume}/bin/build-resume
          ${buildZola}/bin/build-zola
        '';

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [

            # Task runner
            just

            # Website
            zola
            wrangler

            # Resume
            typst
            tinymist
            typstyle

            # Profile README
            minijinja

            # Rust
            rustToolchain
            wasm-pack
            wasm-bindgen-cli
            binaryen
          ];

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
          readme = flake-utils.lib.mkApp {
            drv = buildReadme;
          };
          zola = flake-utils.lib.mkApp {
            drv = buildZola;
          };
          deploy = flake-utils.lib.mkApp {
            drv = deploySite;
          };
          default = flake-utils.lib.mkApp {
            drv = buildSite;
          };
        };
      }
    );
}
