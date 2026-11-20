# Horizon Modding Architecture & Guidelines

เอกสารอ้างอิงมาตรฐานการพัฒนา Mod สำหรับ **Horizon (Minecraft 26.3 written in Rust)** ตามเอกสารทางการ ([playhorizon.dev](https://playhorizon.dev/docs/make-a-mod) และ Horizon Modding Book)

---

## 1. แพ็กเกจมาตรฐาน: `.hmod` (One Package, Three Kinds)

Horizon Mod ทุกชนิดจัดส่งเป็นไฟล์ `.hmod` ไฟล์เดียว (ZIP archive) ภายใต้โครงสร้างมาตรฐาน:

| ชนิด Mod | สิ่งที่บรรจุในไฟล์ | วิธีการทำงาน |
| :--- | :--- | :--- |
| **Content mod** | `assets/` และ `data/` (บล็อก, ไอเทม, สูตร, loot, textures) | เกมอ่านตอนเริ่มรัน ไม่มีโค้ด ไม่ต้องคอมไพล์ |
| **Native mod** | รหัสต้นฉบับ Rust (`src/lib.rs` + `Cargo.toml`) | Horizon Launcher คอมไพล์บนเครื่องผู้เล่นโดยตรงแบบ Native ได้ความเร็วสูงสุดและสิทธิ์เต็ม |
| **WebAssembly mod** | `mod.wasm` ที่คอมไพล์จาก Rust | ทำงานใน Sandbox ปลอดภัย ไม่ต้องมี Rust toolchain บนเครื่องผู้เล่น |

---

## 2. โครงสร้างโปรเจกต์มาตรฐาน (Native / WASM Mod)

```text
my_mod/
├── horizon-mod.toml     # Manifest หลักของ Mod (ไฟล์เดียวเท่านั้น)
├── Cargo.toml           # การพึ่งพาและ Crate spec
├── src/
│   └── lib.rs           # โค้ด Rust หลักที่ implement mod_api::Hooks
├── assets/<mod_id>/     # Resources (โมเดล, textures, ภาษา en_us.json)
├── data/<mod_id>/       # Data (recipes, tags, loot tables)
└── icon.png             # ไอคอนของ mod (PNG, สูงสุด 512x512, ไม่เกิน 256 KiB)
```

> [!IMPORTANT]
> - ไม่ต้องมีโฟลเดอร์ระบุ ID ซ้อนที่ root ของ package
> - ห้ามใช้ `horizon-native.json` หรือ `horizon-mixin.toml` ในโปรเจกต์ `.hmod` สมัยใหม่

---

## 3. สเปกไฟล์ Manifest (`horizon-mod.toml`)

ตัวอย่างการกำหนดค่าสำหรับ Native Mod:

```toml
id = "camera_utils"
name = "Camera Utils"
version = "0.1.0"
description = "Camera utilities mod (Freecam, Freelock, Zoom) for Horizon"
authors = ["GkungZ01"]
license = "MIT"
api = "0.2"
side = "client"
capabilities = ["registries"]

[native]
game = "26.3"
horizon = "f00125f61f794aaa3d4e950d301f353b35b6ee3d"
```

---

## 4. การเขียน Code Hooks ด้วย `mod_api`

1. นำเข้า trait `Hooks` จาก `mod_api`
2. Implement เฉพาะ method ที่ต้องการใช้งาน
3. ใช้ attribute macro `#[mod_api::horizon_mod]` บน `impl Hooks`:

```rust
use mod_api::{ClientHost, Hooks, ZoomEvent, Zoom};

#[derive(Default)]
pub struct CameraUtilsMod;

#[mod_api::horizon_mod]
impl Hooks for CameraUtilsMod {
    fn zoom(&self, host: &dyn ClientHost, event: &mut ZoomEvent) -> Option<Zoom> {
        // คืนค่า Option<Zoom> เพื่อปรับ FOV
        Some(Zoom {
            factor: 4.0,
            smooth: true,
            turn: 0.25,
        })
    }
}
```

---

## 5. Tooling & คำสั่งการทำงาน

- **สร้างโปรเจกต์ใหม่:**
  ```sh
  cargo horizon-mod new <mod-id> --template native
  ```
- **คอมไพล์และทดสอบบน Launcher โดยตรง:**
  ```sh
  horizon-launcher native-build . --trust-native-code
  ```
- **คอมไพล์เป็น WebAssembly:**
  ```sh
  cargo horizon-mod build --wasm
  ```
- **ทดสอบ Unit Test โลคอล:**
  ```sh
  cargo test
  ```
