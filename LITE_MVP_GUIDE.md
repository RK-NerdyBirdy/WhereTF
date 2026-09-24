# WhereTF Lite MVP

This milestone branch keeps manual upload, MiniLM semantic indexing, and direct semantic search. Folder watching, query expansion, relationship mapping, multimodal vision extraction, and the advanced Dioxus panels are disconnected.

## Start the backend

From `Backend/`, with Docker Desktop running:

```powershell
docker compose up --build -d
```

FastAPI is available at `http://localhost:8000`. PostgreSQL is mapped to port `5433`. The first start downloads `all-MiniLM-L6-v2` and EasyOCR.

Stop the stack with:

```powershell
docker compose down
```

## Run the Rust frontend

From `Frontend/`, install Rust and the Dioxus CLI if needed, then run:

```powershell
cargo install dioxus-cli
cargo run
```

The desktop app checks backend health, lets you select a file with `+ add file`, and searches indexed content with the core search bar.