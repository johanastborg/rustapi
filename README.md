# arXiv Paper Storage REST API (Axum + Rust)

A high-performance, asynchronous REST API for indexing and managing arXiv academic research papers, implemented with **Axum 0.8**, **Tokio**, and an **in-memory thread-safe mock storage engine** (`Arc<RwLock<HashMap<String, Paper>>>`).

Included is a modern, responsive web dashboard with dark-mode glassmorphic styling and an interactive REST API Playground.

---

## 🚀 Features

- **Blazing Fast In-Memory Storage**: Concurrent, non-blocking paper store using `tokio::sync::RwLock` pre-seeded with foundational papers (e.g. *Attention Is All You Need*, *Deep Residual Learning*, *BERT*, *GPT-3*, *LoRA*, *DPO*, *FlashAttention*).
- **Full RESTful CRUD Support**:
  - `GET /api/papers`: Filter by query keyword, category, author, sort, and paginate.
  - `GET /api/papers/{id}`: Retrieve a specific paper by arXiv ID.
  - `POST /api/papers`: Store a new paper with metadata (custom or auto-generated ID).
  - `PUT /api/papers/{id}`: Partial or full update of paper metadata.
  - `DELETE /api/papers/{id}`: Remove paper from repository.
  - `GET /api/stats`: Real-time category distribution, author counts, and total paper metrics.
  - `GET /health` & `GET /api/health`: Service health status, uptime version, and live count.
  - `POST /api/papers/reset`: Reset repository back to default seeds.
- **Rich Interactive UI & API Playground**:
  - Search in real-time with instant debounce across titles, authors, categories, and abstracts.
  - Interactive API console to execute live `GET`, `POST`, `PUT`, `DELETE` calls from the browser.
  - Generates ready-to-use `curl` commands.

---

## 🛠️ Project Structure

```text
rustwebapp/
├── Cargo.toml              # Dependencies: Axum 0.8, Tokio, Serde, Tower-HTTP, Chrono, Uuid
├── src/
│   ├── lib.rs              # App factory (create_app) and module exports
│   ├── main.rs             # Server entry point & TCP listener configuration
│   ├── models.rs           # Paper, CreatePayload, UpdatePayload, PaperQuery, PaginatedResponse
│   ├── store.rs            # PaperStore with in-memory HashMap and seed dataset
│   ├── handlers.rs         # HTTP request handlers with Axum extractors
│   └── error.rs            # AppError mapping to HTTP status codes & JSON error bodies
├── tests/
│   └── api_tests.rs        # Automated integration test suite covering all REST endpoints
└── static/
    ├── index.html          # Web dashboard & API playground
    ├── style.css           # Vanilla CSS design system (tokens, glassmorphism, responsive)
    └── app.js              # Vanilla JS frontend client & playground logic
```

---

## 🚦 Getting Started

### 1. Build & Run the Server

```bash
cargo run
```

The server binds to `http://localhost:3000` (or the port defined in `$PORT`).

### 2. Run the Automated Test Suite

```bash
cargo test
```

---

## 📡 REST API Reference

### 1. Health Check
- **Endpoint**: `GET /health` or `GET /api/health`
- **Response**: `200 OK`
```json
{
  "status": "ok",
  "service": "arxiv-paper-storage",
  "version": "0.1.0",
  "timestamp": "2026-09-29T13:23:03.475Z",
  "paper_count": 7
}
```

---

### 2. List & Search Papers
- **Endpoint**: `GET /api/papers`
- **Query Parameters**:
  - `q`: Search substring across title, authors, summary, or ID (e.g. `?q=attention`)
  - `category`: Filter by category (e.g. `?category=cs.CL`)
  - `author`: Filter by author name (e.g. `?author=Vaswani`)
  - `sort`: Sort order (`newest`, `oldest`, `title`)
  - `page`: Page index (default: `1`)
  - `limit`: Items per page (default: `20`, max: `100`)
- **Response**: `200 OK`
```json
{
  "items": [
    {
      "id": "1706.03762",
      "title": "Attention Is All You Need",
      "authors": ["Ashish Vaswani", "Noam Shazeer", "..."],
      "summary": "We propose a new simple network architecture, the Transformer...",
      "categories": ["cs.CL", "cs.LG", "cs.AI"],
      "primary_category": "cs.CL",
      "published": "2017-06-12T17:57:34Z",
      "updated": "2023-08-02T01:54:19Z",
      "doi": "10.48550/arXiv.1706.03762",
      "pdf_url": "https://arxiv.org/pdf/1706.03762.pdf",
      "arxiv_url": "https://arxiv.org/abs/1706.03762",
      "comment": "15 pages, 5 figures, NeurIPS 2017",
      "journal_ref": "NeurIPS 2017"
    }
  ],
  "total": 1,
  "page": 1,
  "limit": 20,
  "total_pages": 1
}
```

---

### 3. Get Paper by ID
- **Endpoint**: `GET /api/papers/{id}`
- **Response**: `200 OK` with Paper JSON, or `404 Not Found` if not found.

---

### 4. Create New Paper
- **Endpoint**: `POST /api/papers`
- **Headers**: `Content-Type: application/json`
- **Request Body**:
```json
{
  "id": "2404.12345",
  "title": "Mamba: Linear-Time Sequence Modeling with Selective State Spaces",
  "authors": ["Albert Gu", "Tri Dao"],
  "summary": "Foundation models are now predominantly based on the Transformer architecture...",
  "categories": ["cs.LG", "cs.AI"],
  "primary_category": "cs.LG",
  "comment": "Preprint"
}
```
- **Response**: `201 Created` with `Location: /api/papers/2404.12345`

---

### 5. Update Paper
- **Endpoint**: `PUT /api/papers/{id}`
- **Headers**: `Content-Type: application/json`
- **Request Body**:
```json
{
  "title": "Updated Title Here",
  "comment": "Revised preprint"
}
```
- **Response**: `200 OK` with updated Paper JSON.

---

### 6. Delete Paper
- **Endpoint**: `DELETE /api/papers/{id}`
- **Response**: `200 OK` with deleted Paper JSON (or `404 Not Found`).

---

### 7. Repository Statistics
- **Endpoint**: `GET /api/stats`
- **Response**: `200 OK`
```json
{
  "total_papers": 7,
  "categories_count": {
    "cs.AI": 6,
    "cs.CL": 5,
    "cs.CV": 1,
    "cs.LG": 6
  },
  "total_authors": 44
}
```

---

### 8. Reset to Mock Seeds
- **Endpoint**: `POST /api/papers/reset`
- **Response**: `200 OK`
```json
{
  "message": "Storage reset to default mock seed papers",
  "total_papers": 7
}
```

---

## 💻 Quick cURL Examples

```bash
# Health check
curl http://localhost:3000/health

# List all papers
curl http://localhost:3000/api/papers

# Search papers with keyword 'attention'
curl "http://localhost:3000/api/papers?q=attention"

# Retrieve specific paper
curl http://localhost:3000/api/papers/1706.03762

# Create paper
curl -X POST http://localhost:3000/api/papers \
  -H "Content-Type: application/json" \
  -d '{
    "id": "2401.00001",
    "title": "Sample Paper",
    "authors": ["Ada Lovelace"],
    "summary": "Exploration of algorithms.",
    "categories": ["cs.AI"]
  }'

# Update paper
curl -X PUT http://localhost:3000/api/papers/2401.00001 \
  -H "Content-Type: application/json" \
  -d '{"title": "Updated Sample Paper"}'

# Delete paper
curl -X DELETE http://localhost:3000/api/papers/2401.00001
```
