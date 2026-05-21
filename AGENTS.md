# AGENTS.md

## Project

MCP server for Invidious (YouTube frontend) search. Single-crate Rust binary based on `mcp-searxng` pattern using `rust-mcp-sdk` 0.9 with `#[mcp_tool]` macros and `tool_box!` dispatch.

## Test Instance

Use `http://invidious.one.belcar.corp` for development and testing.

## Toolchain

- Rust 1.93+, edition 2024
- `openssl` is **vendored** — builds without system OpenSSL but needs `cc` and build essentials
- Base project: `/data/rust/mcp-searxng` (copy patterns from here)

## Commands

```
cargo build          # build
cargo run            # run
cargo test           # no tests yet
cargo clippy -- -D warnings  # lint
cargo fmt            # format
```

## Architecture

```
src/
├── main.rs          # Entry: env → config → transport (stdio/http) → boot
├── config.rs        # AppConfig from env vars with defaults
├── handler.rs       # ServerHandler impl (list_tools, call_tool via tool_box dispatch)
├── error.rs         # thiserror-based error types
├── invidious.rs     # reqwest client + Invidious response types
└── tools.rs         # Tools via #[mcp_tool] macro + tool_box! enum
```

## Environment Variables

| Variable | Default | Required | Description |
|---|---|---|---|
| `INVIDIOUS_URL` | — | Yes | Invidious instance base URL |
| `MCP_TRANSPORT` | `stdio` | No | `stdio` or `http` |
| `MCP_HOST` | `0.0.0.0` | No | HTTP bind address |
| `MCP_PORT` | `3005` | No | HTTP bind port |
| `INVIDIOUS_LANG` | `es` | No | Default search language (ISO 639-1) |

## Invidious API

Key endpoints (all under `{INVIDIOUS_URL}/api/v1/`):

- `GET /search?q=QUERY` — Search videos. Optional: `page`, `sort_by` (relevance/rating/date/views), `date` (hour/today/week/month/year), `duration` (short/long), `type` (video/playlist/channel/movie/show)
- `GET /videos/{id}` — Video details
- `GET /channels/{id}` — Channel details
- `GET /comments/{id}` — Video comments
- `GET /trending` — Trending videos (optional: `type`, `region`)

Response format differs from SearXNG — results are arrays with fields like `title`, `videoId`, `author`, `lengthSeconds`, `viewCount`, `publishedText`, `type`.

## Key Patterns (from rust-mcp-sdk 0.9)

- Tools: `#[mcp_tool(...)]` + `JsonSchema` derive + `tool_box!` macro
- Dispatch: `InvidiousTools::try_from(params).map_err(CallToolError::new)?`
- Results: `CallToolResult::text_content(vec![TextContent::from(text)])`
- Handler: `handler.to_mcp_server_handler()` for `McpServerOptions`
- Stdio: `StdioTransport::new(TransportOptions::default())?` → `server_runtime::create_server()` → `server.start().await`
- HTTP: `hyper_server::create_server(details, handler, HyperServerOptions { host, port, ..Default::default() })` → `server.start().await`
- Need `use rust_mcp_sdk::McpServer` in scope for `.start()` method
- Protocol: `ProtocolVersion::V2025_11_25.into()`
- Capabilities: `ServerCapabilities { tools: Some(ServerCapabilitiesTools { list_changed: None }), ..Default::default() }`

## Dependencies

Copy from `mcp-searxng/Cargo.toml`:
- `rust-mcp-sdk` (0.9) — MCP Server SDK with stdio + streamable-http transports, macro support
- `reqwest` (0.12) — HTTP client for Invidious API calls
- `tokio` (full) — async runtime
- `serde` / `serde_json` — serialization
- `tracing` + `tracing-subscriber` (env-filter) — logging
- `anyhow` / `thiserror` — error handling
- `async-trait` — trait impl for `ServerHandler`
- `openssl` (vendored) — TLS
