## About This Project

# Real-Time Fraud Alert System

A full-stack fraud detection platform that analyzes financial transactions in real-time using rule-based pattern matching and WebSocket alerts.

## Tech Stack

**Backend**
- **Rust** with Actix-Web framework
- **SQLx** for type-safe PostgreSQL queries
- **DashMap** for lock-free concurrent user profile storage
- **Tokio** async runtime for non-blocking I/O
- **WebSocket** via actix-ws for real-time communication

**Frontend**
- **TypeScript** + **React** with Vite
- **TailwindCSS** for responsive UI
- **WebSocket API** for live updates
- **React hooks** for state management

**Infrastructure**
- **Docker** + **Docker Compose** for local development
- **PostgreSQL** for persistent storage

## Features

### Real-Time Analysis
- In-memory analysis using concurrent data structures (sub-millisecond fraud scoring)
- Six fraud detection rules including velocity checks, geographic anomalies, amount patterns, duplicate detection, and more
- Instant notifications to connected dashboard when fraud is detected
- Async database writes that don't block fraud analysis engine

### Dashboard
- Alerts and notifications on fraud
- Analytics on fraud rate, average score, and high-risk transaction count
- Transaction simulator for testing and demos

### API
- RESTful endpoints for transaction submission and retrieval
- WebSocket connections for real-time alert streaming
- Type-safe error handing with custom error types
- CORS-enabled for cross-orgin requests

## Architecture

**Request Flow:**
1. Transaction submitted from React frontend
2. Rust backend performs fraud analysis (in-memory, < 1ms)
3. Result returned to user immediately
4. Transaction stored in PostgreSQL (async, non-blocking)
5. If fraud detected → WebSocket broadcast to all connected clients
6. Live dashboard updates in real-time

**Key Design Decisions:**
- **In-memory analysis** (DashMap): Fast user profile lookups for fraud scoring
- **Async database writes**: Don't block fraud analysis waiting for disk I/O
- **WebSocket broadcast**: Real-time alerts to multiple dashboard clients
- **PostgreSQL**: Persistent transaction history and audit trail

## Fraud Detection Rules

The system implements 6 rule-based fraud detection algorithms:

1. **Velocity Check**: Flags users making > 5 transactions in 5 minutes
2. **Amount Anomaly**: Detects transactions exceeding 2x user's historical maximum
3. **Geographic Impossibility**: Identifies transactions from different countries within 1 hour
4. **Unusual Country**: Alerts on first-time country usage
5. **Duplicate Transaction**: Catches identical transactions within 10 seconds
6. **Card Velocity**: Monitors card usage (> 8 uses in 10 minutes triggers alert)
