# MCP Invidious

Model Context Protocol server that provides YouTube search capabilities through [Invidious](https://github.com/iv-org/invidious) instances.

## Features

- **Search videos, channels, and playlists** with filtering by date, duration, and sort order
- **Video details** including title, description, views, likes, duration, and keywords
- **Channel details** with subscriber count, description, and metadata
- **Video comments** with reply counts and sorting options
- **Trending videos** by category and region
- **Dual transport**: stdio for CLI agents, HTTP for remote connections

## Available Tools

| Tool | Description |
|------|-------------|
| `search_videos` | Search YouTube videos with optional filters (sort_by, date, duration) |
| `search_channels` | Search YouTube channels |
| `search_playlists` | Search YouTube playlists |
| `video_details` | Get detailed information about a specific video |
| `channel_details` | Get detailed information about a specific channel |
| `video_comments` | Get comments for a video |
| `trending` | Get trending videos by type and region |

## Quick Start

### Prerequisites

- Rust 1.93+
- An Invidious instance URL

### Build and Run

```bash
cargo build --release

# Run with stdio transport (default)
INVIDIOUS_URL=https://vid.puffyan.us cargo run --release

# Run with HTTP transport
INVIDIOUS_URL=https://vid.puffyan.us MCP_TRANSPORT=http cargo run --release
```

### Configuration

| Variable | Default | Required | Description |
|----------|---------|----------|-------------|
| `INVIDIOUS_URL` | — | Yes | Invidious instance base URL |
| `MCP_TRANSPORT` | `stdio` | No | Transport mode: `stdio` or `http` |
| `MCP_HOST` | `0.0.0.0` | No | HTTP bind address |
| `MCP_PORT` | `3005` | No | HTTP bind port |
| `INVIDIOUS_LANG` | `es` | No | Default search language (ISO 639-1) |

## Docker

Build the image:

```bash
podman build -t mcp-invidious:latest .
```

Run with Podman:

```bash
podman run -e INVIDIOUS_URL=https://vid.puffyan.us -p 3005:3005 mcp-invidious:latest
```

Or use the provided systemd container unit:

```bash
# Edit mcp-invidious.container with your INVIDIOUS_URL, then:
systemctl --user enable --now mcp-invidious.container
```

## Architecture

```
src/
├── main.rs          # Entry: env → config → transport (stdio/http) → boot
├── config.rs        # AppConfig from env vars with defaults
├── handler.rs       # ServerHandler impl (tool dispatch via tool_box!)
├── error.rs         # thiserror-based error types
├── invidious.rs     # reqwest client + Invidious API response types
└── tools.rs         # Tools via #[mcp_tool] macro + formatting helpers
```

## Development

```bash
cargo build
cargo run
cargo test
cargo clippy -- -D warnings
cargo fmt
```

## License

MIT
