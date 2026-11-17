# Packaging

What the CI builds on every push to main (`.github/workflows/build.yml`)
and publishes on a release (a tag `v*`). None of it carries the CD's data.

| Where | What | How |
|---|---|---|
| The release | `gadget-generator-x86_64.AppImage`, `gadget-generator-aarch64.AppImage` | CI (linuxdeploy) |
| The release | `gadget-generator-x86_64.deb`, `gadget-generator-aarch64.deb`, `.rpm`s | CI (cargo-deb, cargo-generate-rpm; `crates/widget/Cargo.toml`) |
| The release | `gadget-generator-linux-x86_64.tar.gz`, `-aarch64` | CI: the bare program |
| The release | `gadget-generator-windows-x86_64.exe`, `gadget-generator-windows-arm64.exe` | CI |
| The release | `gadget-generator-macos.dmg` (Apple silicon and Intel) | CI |
| The release | `gadget-generator-web.zip`: the site, as on GitHub Pages | CI (`docs/web.md`) |
| ghcr.io | `ghcr.io/doridian/gadgetgenerator` (`:latest`, `:X.Y.Z`, `:X.Y`): the site served by nginx, linked to this repository | CI (`docker/Dockerfile`) |
| Nix | `nix run github:Doridian/GadgetGenerator` | `flake.nix` |

## A release

Tag `vX.Y.Z` and push the tag. The CI builds everything (the .deb, the
.rpm and the macOS app take their version from the tag), makes the
release with every file on it and pushes the container image.
