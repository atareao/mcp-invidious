use std::sync::Arc;

use async_trait::async_trait;
use rust_mcp_sdk::McpServer;
use rust_mcp_sdk::mcp_server::ServerHandler;
use rust_mcp_sdk::schema::{
    CallToolRequestParams, CallToolResult, CompleteRequestParams, CompleteResult,
    ListResourceTemplatesResult, ListResourcesResult, ListToolsResult, PaginatedRequestParams,
    ReadResourceRequestParams, ReadResourceResult, RpcError, schema_utils::CallToolError,
};

use crate::config::AppConfig;
use crate::error::AppError;
use crate::invidious::InvidiousClient;

pub struct InvidiousHandler {
    pub config: AppConfig,
    pub client: InvidiousClient,
}

impl InvidiousHandler {
    pub fn new(config: AppConfig) -> Result<Self, AppError> {
        let client = InvidiousClient::new(&config)?;
        Ok(Self { config, client })
    }
}

#[async_trait]
impl ServerHandler for InvidiousHandler {
    async fn handle_list_tools_request(
        &self,
        _params: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        let tools: Vec<_> = crate::tools::InvidiousTools::tools()
            .into_iter()
            .filter(|t| self.config.is_tool_enabled(&t.name))
            .collect();
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools,
        })
    }

    async fn handle_call_tool_request(
        &self,
        params: CallToolRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        if !self.config.is_tool_enabled(&params.name) {
            return Err(CallToolError::from_message(format!(
                "tool '{}' is disabled; add it to INVIDIOUS_ENABLED_TOOLS or set it to 'all'",
                params.name
            )));
        }

        let tool = crate::tools::InvidiousTools::try_from(params).map_err(CallToolError::new)?;

        let config = &self.config;
        let client = &self.client;

        match tool {
            crate::tools::InvidiousTools::SearchVideos(t) => t.call_tool(config, client).await,
            crate::tools::InvidiousTools::SearchChannels(t) => t.call_tool(config, client).await,
            crate::tools::InvidiousTools::SearchPlaylists(t) => t.call_tool(config, client).await,
            crate::tools::InvidiousTools::VideoDetails(t) => t.call_tool(config, client).await,
            crate::tools::InvidiousTools::ChannelDetails(t) => t.call_tool(config, client).await,
            crate::tools::InvidiousTools::VideoComments(t) => t.call_tool(config, client).await,
            crate::tools::InvidiousTools::Trending(t) => t.call_tool(config, client).await,
        }
    }

    async fn handle_list_resources_request(
        &self,
        _params: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ListResourcesResult, RpcError> {
        Ok(ListResourcesResult {
            meta: None,
            next_cursor: None,
            resources: vec![],
        })
    }

    async fn handle_list_resource_templates_request(
        &self,
        _params: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ListResourceTemplatesResult, RpcError> {
        Ok(ListResourceTemplatesResult {
            meta: None,
            next_cursor: None,
            resource_templates: vec![],
        })
    }

    async fn handle_read_resource_request(
        &self,
        _params: ReadResourceRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ReadResourceResult, RpcError> {
        Err(RpcError::method_not_found())
    }

    async fn handle_complete_request(
        &self,
        _params: CompleteRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<CompleteResult, RpcError> {
        Err(RpcError::method_not_found())
    }
}
