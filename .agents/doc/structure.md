# Project Structure

```text
camera-utils/
├── Cargo.toml                         # Cargo package name: camera-utils
├── compatibility.json                 # Revisions for Horizon and horizon-mixins
├── mixins/
│   └── camera_utils/                  # Renamed from example_mod
│       ├── horizon-mixin.toml         # Mixin declarations (Camera/Input hooks)
│       └── runtime.rs                 # CameraUtils state, input logic, unit tests
├── tools/
│   ├── horizon.py                     # Workflow: export -> plan -> apply -> check/test
│   ├── test_template.py               # Template tests
│   └── test_stage.py                  # Pinned source audit
└── .agents/
    └── doc/                           # Project technical specs
```
