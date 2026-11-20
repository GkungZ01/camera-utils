# Camera Utils

Camera utilities mod (Freecam, Freelock, Zoom) for **Horizon** (Minecraft 26.3 written in Rust).

## Features
- **Zoom**: Hold `C` to smoothly zoom in (4x FOV magnification).
- **Freelock**: Press `X` to freely look around while keeping the player moving in the original direction.
- **Freecam**: Press `U` to detach the camera from the player and fly around freely.

## Building & Playing
Compile and run with Horizon Launcher:
```sh
horizon-launcher native-build . --trust-native-code
```

Or run unit tests:
```sh
cargo test
```

## License
MIT
