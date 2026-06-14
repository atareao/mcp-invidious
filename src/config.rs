use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub invidious_url: String,
    pub transport: TransportMode,
    pub host: String,
    pub port: u16,
    pub default_lang: String,
    pub invidious_insecure: bool,
    pub invidious_enabled_tools: Vec<String>,
}

impl AppConfig {
    pub fn is_tool_enabled(&self, name: &str) -> bool {
        self.invidious_enabled_tools
            .iter()
            .any(|t| t == name || t == "all")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransportMode {
    Stdio,
    Http,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let invidious_url =
            std::env::var("INVIDIOUS_URL").map_err(|_| AppError::MissingEnvVar("INVIDIOUS_URL"))?;

        let transport = match std::env::var("MCP_TRANSPORT").as_deref().unwrap_or("stdio") {
            "stdio" => TransportMode::Stdio,
            "http" => TransportMode::Http,
            other => {
                return Err(AppError::InvalidEnvVar(
                    "MCP_TRANSPORT",
                    format!("must be 'stdio' or 'http', got '{other}'"),
                ));
            }
        };

        let host = std::env::var("MCP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = std::env::var("MCP_PORT")
            .as_deref()
            .map(|s| s.parse::<u16>())
            .unwrap_or(Ok(3005))
            .map_err(|e| AppError::InvalidEnvVar("MCP_PORT", e.to_string()))?;

        let default_lang = std::env::var("INVIDIOUS_LANG").unwrap_or_else(|_| "es".to_string());

        let invidious_insecure = std::env::var("INVIDIOUS_INSECURE")
            .as_deref()
            .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        let invidious_enabled_tools = std::env::var("INVIDIOUS_ENABLED_TOOLS")
            .ok()
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split(',')
                    .map(|t| t.trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        Ok(Self {
            invidious_url,
            transport,
            host,
            port,
            default_lang,
            invidious_insecure,
            invidious_enabled_tools,
        })
    }
}
