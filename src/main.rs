mod config;
mod error;
mod handler;
mod invidious;
mod tools;

use std::sync::Arc;

use rust_mcp_sdk::mcp_server::server_runtime;
use rust_mcp_sdk::mcp_server::{HyperServerOptions, McpServerOptions, ServerRuntime, hyper_server};
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, ProtocolVersion, ServerCapabilities, ServerCapabilitiesTools,
};
use rust_mcp_sdk::{McpServer, StdioTransport, ToMcpServerHandler, TransportOptions};
use tracing::info;

use crate::config::TransportMode;
use crate::handler::InvidiousHandler;

fn build_server_details() -> InitializeResult {
    InitializeResult {
        server_info: Implementation {
            name: "mcp-invidious".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            title: Some("MCP Invidious Search Server".into()),
            description: Some(
                "Search YouTube videos, channels, and playlists using an Invidious instance".into(),
            ),
            icons: vec![],
            website_url: None,
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some(
            "Search YouTube content via Invidious. Use: search_videos (with optional \
             sort_by/date/duration filters), search_channels, search_playlists, \
             video_details, channel_details, video_comments, trending."
                .into(),
        ),
        protocol_version: ProtocolVersion::V2025_11_25.into(),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = match config::AppConfig::from_env() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };

    info!("Configuration loaded: transport={:?}", config.transport);

    let handler = match InvidiousHandler::new(config) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("Failed to initialize handler: {}", e);
            std::process::exit(1);
        }
    };
    let server_details = build_server_details();

    match handler.config.transport {
        TransportMode::Stdio => run_stdio(server_details, handler).await,
        TransportMode::Http => run_http(server_details, handler).await,
    }
}

async fn run_stdio(server_details: InitializeResult, handler: InvidiousHandler) {
    info!("Starting MCP Invidious server (stdio transport)");

    let transport = match StdioTransport::new(TransportOptions::default()) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to create transport: {}", e);
            std::process::exit(1);
        }
    };

    let server: Arc<ServerRuntime> = server_runtime::create_server(McpServerOptions {
        server_details,
        transport,
        handler: handler.to_mcp_server_handler(),
        task_store: None,
        client_task_store: None,
        message_observer: None,
    });

    if let Err(e) = server.start().await {
        tracing::error!("Server error: {}", e);
        std::process::exit(1);
    }
}

async fn run_http(server_details: InitializeResult, handler: InvidiousHandler) {
    let host = handler.config.host.clone();
    let port = handler.config.port;

    info!(
        "Starting MCP Invidious server (HTTP transport) on {}:{}",
        host, port
    );

    let server = hyper_server::create_server(
        server_details,
        handler.to_mcp_server_handler(),
        HyperServerOptions {
            host,
            port,
            ..Default::default()
        },
    );

    if let Err(e) = server.start().await {
        tracing::error!("Server error: {}", e);
        std::process::exit(1);
    }
}
