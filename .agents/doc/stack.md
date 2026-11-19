# Technology Stack & Project Spec

- **Author / GitHub**: `gkungz01`
- **Mod Name**: `camera-utils`
- **Target Platform**: Horizon Minecraft (Rust 2024 / rust-version 1.95)
- **Target Horizon Revision**: `ba6344a59cb9e755b730dcf02285fc0d33b97d3c` (Nightly 2026-10-30 / 26.3)
- **Injection Framework**: `horizon-mixins` (0.6.0, schema 6)
- **Core Features**:
  1. **Freecam**: ปลดล็อกตำแหน่งกล้องออกจากตัวละคร ควบคุมทิศทางและมุมมองกล้องได้อย่างอิสระโดยไม่ส่ง movement packet ไปยัง server
  2. **Freelock**: หันมุมมองกล้องได้อย่างอิสระโดยที่ทิศทางการเคลื่อนไหวของตัวละครยังคงเดิม
  3. **Zoom**: ย่อ/ขยาย FOV (Field of view) ของกล้องอย่างนุ่มนวล
