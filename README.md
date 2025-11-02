
# 🎮 CS Matchmaking System

A high-performance **Rust backend** that powers competitive matchmaking for multiplayer games.  
This project simulates the full lifecycle of online match coordination — from player queueing to server allocation and ranking updates.

---

## 🚀 Overview

**CS Matchmaking System** is a modular, async microservice built with Rust, designed for real-time player coordination in competitive environments (like Counter-Strike or other team-based games).

It provides:
- **Queue management** for solo players and parties
- **Matchmaking algorithm** with MMR, latency, and wait-time balancing
- **Game server allocation**
- **Player ranking system** (Elo / Glicko-based)
- **Party system**, penalties, and anti-abuse mechanics
- **Observability layer** with metrics and tracing

---

## 🧩 Architecture

| Layer | Description                                                          |
|-------|----------------------------------------------------------------------|
| **API Layer** | REST API via `Actix`, documented with `utoipa` / OpenAPI             |
| **Queue Engine** | Redis-based async queues for matchmaking candidates                  |
| **Matchmaker Worker** | Background task that forms matches by mode, region, and skill window |
| **Database** | PostgreSQL + Diesel ORM                                              |
| **Messaging / Caching** | Redis for queues and short-term state                                |
| **Metrics & Logging** | Prometheus, Grafana, and `log`                                       |
| **Deployment** | Docker Compose stack (Postgres, Redis, backend service)              |

---

## 📅 Development Sprints

| Sprint | Focus | Key Deliverables                                    |
|--------|-------|-----------------------------------------------------|
| **1. Core Setup** | Project initialization, migrations, base REST API, OpenAPI docs | `Actix + SQLx` setup, `/healthz`, sessions, queues  |
| **2. Matchmaker Engine** | Worker implementation, Redis integration, match lifecycle | Player grouping, matchmaking tick, mock allocator   |
| **3. Party & Ranking** | Party management and MMR updates | Party endpoints, Elo/Glicko rating, leaderboards    |
| **4. Hardening & Observability** | Security, testing, tracing, metrics | Prometheus metrics, integration tests, load testing |

---

## ⚙️ Tech Stack

- **Language:** Rust (edition 2021)
- **Framework:** Actix / Tokio (async runtime)
- **Database:** PostgreSQL
- **ORM:** Diesel
- **Cache / Queue:** Redis
- **Metrics:** Prometheus / Grafana
- **Docs:** Utoipa + Swagger
- **Testing:** Tokio-test, integration test suite
- **Containerization:** Docker Compose

---

## 🧱 Database Schema (Core Tables)

- `players`, `player_sessions`
- `queues`, `queue_entries`
- `matches`, `match_players`
- `parties`, `party_members`
- `player_ratings`, `rating_changes`
- `penalties`, `reports`
- `game_servers`, `server_allocations`

---

## 🧠 Matchmaking Algorithm

Each worker cycle:
1. Reads queue candidates grouped by region and mode.
2. Expands acceptable MMR and latency window over time.
3. Forms balanced teams (`team_size = N`) using scoring by:
   - Skill difference (MMR)
   - Latency
   - Wait time
4. Requests a free game server from allocator.
5. Sends WebSocket notifications to matched players.

---

## 🧰 Getting Started

### Prerequisites
- Rust (≥1.75)
- Docker & Docker Compose
- PostgreSQL, Redis

### Run locally
```bash
git clone https://github.com/Daelon02/cs-matchmaking-system.git
cd cs-matchmaking-system
cp .env.example .env
docker compose up -d
cargo run
```

Access:
- API: `http://localhost:8080`
- Swagger: `http://localhost:8080/docs`
- Metrics: `http://localhost:9090/metrics`

---

## 🧪 Testing
```bash
cargo test
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

---

## 📊 Monitoring
- `Prometheus` scrapes backend metrics.
- `Grafana` dashboards visualize:
  - Queue size / wait times
  - Match formation rates
  - Allocation failures
  - Average MMR delta

---

## 🛡️ Security & Reliability

- JWT player sessions  
- Anti-AFK / leaver penalties  
- Idempotent endpoints  
- Tracing & alerting (Prometheus rules)

---

## 🧩 Future Enhancements

- gRPC integration for external game servers  
- Kafka event streaming  
- Regional sharding for scale  
- Admin web dashboard

---

## 👤 Author
**Daelon02 (Nikita Gubin)**  
Rust backend developer, systems engineer, and creator of the **CS Matchmaking System**.  
🔗 GitHub: [Daelon02](https://github.com/Daelon02)

---

## 🏁 License
MIT License © 2025 Daelon02
