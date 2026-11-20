# 📷 Camera Utils for Horizon

[English](#english) | [ภาษาไทย](#ภาษาไทย)

---

<a name="english"></a>
## 🌟 English Overview

**Camera Utils** is a high-performance native mod built for **Horizon** (Minecraft: Java Edition 26.3 written in Rust). It introduces flexible camera perspective tools including smooth zoom, freelance camera locking (Freelock), and decoupled free-flight camera (Freecam).

### 🚀 Features

- 🔍 **Smooth Zoom (`C`)**:
  - Hold `C` to smoothly zoom in with 4x magnification.
  - Automatically adjusts mouse sensitivity during zoom for smooth and precise aiming.
- 🔄 **Freelock (`X`)**:
  - Press `X` to toggle freelance camera rotation.
  - Allows you to freely inspect your surroundings in 360° while your player character continues moving straight in the original direction.
- 🛸 **Freecam (`U`)**:
  - Press `U` to detach the camera from your character.
  - Fly around freely through blocks to inspect builds, redstone, and terrains without sending player movement packets.

### 📥 Installation (Horizon Launcher)

1. Open **Horizon Launcher**.
2. Navigate to **Mods** > **Find mods** > **Add a native mod from GitHub**.
3. Paste the repository link:
   ```text
   https://github.com/GkungZ01/camera-utils.git
   ```
4. Click **Add from GitHub**.
5. Once imported, click **Build** to compile the native mod on your machine.

---

<a name="ภาษาไทย"></a>
## 🇹🇭 ภาพรวมภาษาไทย

**Camera Utils** เป็น Native Mod ประสิทธิภาพสูงสำหรับ **Horizon** (Minecraft: Java Edition 26.3 เขียนด้วยภาษา Rust) ช่วยเพิ่มเครื่องมือควบคุมมุมมองกล้อง ได้แก่ ซูมมุมมองแบบนุ่มนวล, ปลดล็อกมุมมองอิสระรอบตัว (Freelock) และกล้องบินสำรวจอิสระ (Freecam)

### 🚀 คุณสมบัติเด่น

- 🔍 **Smooth Zoom (กดค้าง `C`)**:
  - ซูมภาพ 4 เท่าอย่างนุ่มนวลแบบ Spyglass
  - ปรับความไวเมาส์ลงอัตโนมัติขณะซูมเพื่อการเล็งที่แม่นยำ
- 🔄 **Freelock (กดสลับ `X`)**:
  - กดปุ่ม `X` เพื่อสลับเปิด/ปิดการหันกล้องอิสระ
  - สามารถหมุนกล้องดูรอบตัวได้ 360 องศา โดยที่ตัวละครยังคงเดินตรงไปในทิศทางเดิม
- 🛸 **Freecam (กดสลับ `U`)**:
  - กดปุ่ม `U` เพื่อปลดล็อกกล้องหลุดออกจากตัวละคร
  - บินทะลุสิ่งกีดขวางเพื่อสำรวจงานก่อสร้างและกลไกได้อย่างอิสระ โดยไม่ส่ง packet การเคลื่อนไหวของตัวละครไปยังเซิร์ฟเวอร์

### 📥 วิธีการติดตั้งใน Horizon Launcher

1. เปิดโปรแกรม **Horizon Launcher**
2. ไปที่เมนู **Mods** > **Find mods** > **Add a native mod from GitHub**
3. ใส่ลิงก์ของ Repository:
   ```text
   https://github.com/GkungZ01/camera-utils.git
   ```
4. กดปุ่ม **Add from GitHub**
5. เมื่อติดตั้งเสร็จ ให้กดปุ่ม **Build** เพื่อคอมไพล์ mod เข้าสู่ตัวเกม

---

## 🛠️ Technology Stack
- **Game Engine**: Horizon (Minecraft Java 26.3 in Rust)
- **Mod Format**: Unified `.hmod` (API `0.2`)
- **Language**: Rust Edition 2024 (Rust 1.95+)
- **Interface**: `mod_api::Hooks`

## 📄 License
Released under the [MIT License](LICENSE).
