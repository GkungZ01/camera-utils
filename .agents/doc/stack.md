# Technology Stack & Project Spec

- **Author / GitHub**: `gkungz01`
- **Mod Name**: `camera-utils`
- **Target Platform**: Horizon Minecraft (Rust 2024 / rust-version 1.95)
- **Target Horizon Revision**: `f00125f61f794aaa3d4e950d301f353b35b6ee3d` (Nightly 2026-11-19 / 26.3)
- **Launcher Version**: Horizon Launcher 0.2.78+ (Unified Package Manifest `horizon-mod.toml`, Pure Native Mod API Hooks)
- **Injection Framework**: Pure `mod_api::Hooks` (Zero mixins, zero private item dependencies)
- **Core Features**:
  1. **Freecam**: ปลดล็อกตำแหน่งกล้องออกจากตัวละคร ควบคุมทิศทางและมุมมองกล้องได้อย่างอิสระโดยไม่ส่ง movement packet ไปยัง server
  2. **Freelock**: หันมุมมองกล้องได้อย่างอิสระโดยที่ทิศทางการเคลื่อนไหวของตัวละครยังคงเดิม
  3. **Zoom**: ย่อ/ขยาย FOV (Field of view) ของกล้องอย่างนุ่มนวล
