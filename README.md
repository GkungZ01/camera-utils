# Horizon Camera Utils

A native **horizon-mixins** camera mod for [Horizon](https://github.com/horizon-rust/horizon) (Minecraft client written in Rust). Provides **Smooth Zoom**, **Freelock** (perspective decoupling), and **Freecam** (detached camera).

Runtime logic lives in a single testable Rust file (`runtime.rs`) that compiles both as a standalone Cargo crate and as an injected module into the native Horizon client.

---

## ✨ Features & Controls

| Feature | Default Key | Description |
|---|---|---|
| **🔍 Zoom** | `C` (Hold) | 4x smooth FOV zoom with automatic mouse sensitivity dampening using Horizon's native zoom pipeline. |
| **🔒 Freelock** | `X` (Toggle) | Decouples camera orientation from character rotation. Walk forward while looking around freely. |
| **🛸 Freecam** | `U` (Toggle) | Detaches the camera from the player entity for free-roam observation. |

---

## 📥 How to Install in Horizon Launcher

1. Open your **Horizon Launcher**.
2. Go to **Mods** -> **Native Mods**.
3. Under **"Add a native mod from GitHub"**, paste your repository URL:
   ```text
   https://github.com/gkungz01/camera-utils
   ```
4. Click **"Add from GitHub"**.
5. The launcher will automatically fetch, mixin, and compile the modded Horizon executable for you.

---

## 🛠️ Repository Layout

```text
Cargo.toml                             Standalone runtime library workspace
compatibility.json                     Pinned Horizon and horizon-mixins revisions
mixins/camera_utils/
  horizon-mixin.toml                   Mixin declarations (Zoom, Key, and Turn hooks)
  runtime.rs                           Camera state, thread-safe storage, and unit tests
  zoom.rs                              Native zoom hook adapter
  key.rs                               Keybind input hook adapter
  turn.rs                              Mouse turn decoupling adapter
tools/horizon.py                       Export → plan → apply → stage/check/build
```

---

## 🧪 Development & Validation

### Standalone Unit Tests
Runs unit tests offline without needing a Horizon checkout:
```sh
cargo test
```

### Stage & Check with Horizon Engine
Validate against the pinned upstream Horizon sources:

1. Clone `horizon` and `horizon-mixins` alongside this folder:
   ```sh
   git clone https://github.com/horizon-rust/horizon ../horizon
   git clone https://github.com/horizon-rust/horizon-mixins ../horizon-mixins
   git -C ../horizon-mixins checkout d9c83d7096b7ea5b83678da2efdfd6161f74997e
   cargo fetch --locked --manifest-path ../horizon-mixins/Cargo.toml
   ```

2. Run Stage and Verify:
   ```sh
   # Stage the mod into pristine Horizon sources
   python tools/horizon.py stage --horizon ../horizon

   # Check compilation of the staged modded client
   python tools/horizon.py check --horizon ../horizon
   ```

### Compatibility Target
- **Horizon**: `ba6344a59cb9e755b730dcf02285fc0d33b97d3c` (Nightly 2026-10-30 / 26.3)
- **Horizon Mixins**: `d9c83d7096b7ea5b83678da2efdfd6161f74997e` (0.6.0, schema 6)

---

## 📜 License
MIT
