# 🕷️ Pavuk — Homelab Project

Welcome to **Pavuk**, a custom homelab ecosystem built to reclaim the cloud. This project focuses on creating high-performance software that mirrors modern cloud services but runs entirely on your own hardware with minimal friction. 

I believe that everything in software begins with data. Consequently, the first phase of Pavuk is a robust **NAS (Network Attached Storage)** foundation. It serves as a privacy-focused alternative to GDrive or Dropbox, built to thrive on modest hardware.

---

## 🏗️ Core Architecture

The system utilizes a microservice-first approach to ensure that a failure in one node doesn't bring down the entire web:

* **📦 nas-server**: Manages physical file I/O, streaming chunked uploads, and directory structures.
* **👤 user-management**: Handles the "Identity" layer—authentication, roles, and storage enforcement.
* **🛡️ admin-console**: The oversight layer for monitoring versions, blocking users, and system health.
* **📡 Communication**: Internal calls use **gRPC (Tonic)**, while asynchronous state changes (like storage quota updates) are broadcast via **RabbitMQ (Lapin)**.

---

## 🛠️ Tech Stack

### 🦀 Backend (Rust)
* **Async Runtime**: `tokio` (The backbone of high-concurrency I/O).
* **gRPC**: `tonic` & `prost` (Strictly typed, lightning-fast communication).
* **Database**: `PostgreSQL` + `sqlx` (Compile-time verified SQL queries).
* **Messaging**: `lapin` (Asynchronous event-driven architecture).

### 🖥️ Frontend (Tauri + Svelte)
* **Core**: Rust-based desktop bridge via **Tauri**.
* **UI**: **Svelte** (Minimalist, reactive interface) communicating via FFI.

---

## 💻 Hardware Specs (The "Old Laptop")

Optimizing for a 6-year-old machine is a core design constraint:
* **CPU**: Intel Core i7-9750H.
* **RAM**: 16GB DDR4.
* **GPU**: NVIDIA GeForce GTX 1650.
* **OS**: Ubuntu 24.04.2 LTS.
* **💽 Storage Strategy**: 
    * **SSD (500GB)**: System OS and high-IOPS PostgreSQL indexing.
    * **RAID Array**: 4 External HDDs (RAID 5/6) for redundant, resilient data storage.

---

## 🚦 Development Workflow

I use a `Justfile` to keep the complexity of microservices manageable:

| Command | Description |
| :--- | :--- |
| `just ready` | Preflights the environment: formats, migrates, and tests. |
| `just run-nas` | Boots the `nas-server` binary. |
| `just run-user` | Boots the `user-management` service. |
| `just seed` | Populates the DB with development data. |
| `just build` | Compiles the entire workspace. |

---

## 🛡️ Features Implemented
- [x] **File Streaming**: Native chunked uploads to bypass memory limits.
- [x] **Smart Archiving**: Integrated Gzip compression with automatic quota recalculation.
- [x] **Recursive FS**: Deep directory nesting and root folder generation.
- [x] **Async Cleanup**: Offloading heavy disk deletions to background tasks.
- [x] **Storage Quotas**: Real-time tracking of `allowed` vs `taken` storage.

---

## 🗺️ Roadmap
- [ ] **SQL Optimization**: Indexing strategies for massive file counts.
- [ ] **Security**: TLS/SSL for gRPC and end-to-end encryption.
- [ ] **Caching Layer**: Multi-level caching for DB results and hot files.
- [ ] **CLI Tool**: Remote debugging and system telemetry.
- [ ] **Global Access**: Secure networking via **WireGuard/Tailscale**.

---

## 💡 The "Maybes" (Future Vision)
- [ ] **Pavuk Stream**: Extending the NAS into a personal media streaming platform.
- [ ] **Pavuk Functions**: A Firebase-like event-driven function runner for local networks.
- [ ] **Mobile Interface**: Native Android/iOS clients for on-the-go access.

---

## 🧠 AI Disclaimer
This project is a journey of learning. I use AI exclusively for research and conceptual clarification. I avoid using agents or auto-generated code because I value the "mental model" and the pure joy of solving problems in Rust manually.

## 🤝 Contribution
I welcome anyone who sees value in this vision. If you'd like to contribute or just chat about the project, feel free to reach out:
📧 **leora2603@gmail.com**