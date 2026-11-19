# Persistent Memory Index

- **Developer**: `gkungz01`
- **Active Task**: Horizon native mod `camera-utils`
- **Engine**: Horizon Minecraft (Rust 26.3)
- **Features Under Development**:
  - Freecam
  - Freelock
  - Zoom
- **Rules & Guardrails**:
  - Horizon engine checkouts remain pristine.
  - Mixin declarations follow AST-based changes through `horizon-mixins`.
- **Horizon Native Mod Packaging Standards (Launcher 0.2.78+)**:
  - **Manifest**: Root `horizon-mod.toml` (Unified Package Manifest).
  - **Source Layout**: Flat under `src/` (`src/lib.rs`, `src/*.rs`). Never keep `.rs` or `.toml` outside `src/` (except root `Cargo.toml` and `horizon-mod.toml`). Legacy paths (`mixins/`, `horizon-mixin.toml`, `tools/audit/`, `rust-toolchain.toml`) are rejected by launcher scanner.
  - **Dependencies**: `Cargo.toml` must declare `horizon-mod-api = { path = ".horizon/sdk/api/crates/horizon-mod-api" }` under `[dependencies]`.
  - **Legacy Template vs Launcher**: Upstream `horizon-rust/horizon-native-mod-template` is for standalone mixin development via CLI (`tools/horizon.py`), whereas Horizon Launcher compiles packaged `.hmod` archives following the modern unified format.
