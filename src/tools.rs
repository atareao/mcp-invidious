use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::{CallToolResult, TextContent, schema_utils::CallToolError};

use crate::config::AppConfig;
use crate::invidious::{InvidiousClient, SearchParams};

// ── search_videos ─────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_videos",
    description = "Search for videos on YouTube via Invidious",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchVideos {
    /// The search query
    query: String,
    /// Page number (default: 1)
    page: Option<u32>,
    /// Sort order: relevance (default), rating, date, views
    sort_by: Option<String>,
    /// Date filter: hour, today, week, month, year
    date: Option<String>,
    /// Duration filter: short, long
    duration: Option<String>,
}

impl SearchVideos {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &InvidiousClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            page: self.page,
            sort_by: self.sort_by.clone(),
            date: self.date.clone(),
            duration: self.duration.clone(),
            r#type: Some("video".to_string()),
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_search_results(&response, "videos"),
        )]))
    }
}

// ── search_channels ──────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_channels",
    description = "Search for YouTube channels via Invidious",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchChannels {
    /// The search query
    query: String,
    /// Page number (default: 1)
    page: Option<u32>,
}

impl SearchChannels {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &InvidiousClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            page: self.page,
            sort_by: None,
            date: None,
            duration: None,
            r#type: Some("channel".to_string()),
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_search_results(&response, "channels"),
        )]))
    }
}

// ── search_playlists ─────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_playlists",
    description = "Search for YouTube playlists via Invidious",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchPlaylists {
    /// The search query
    query: String,
    /// Page number (default: 1)
    page: Option<u32>,
}

impl SearchPlaylists {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &InvidiousClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            page: self.page,
            sort_by: None,
            date: None,
            duration: None,
            r#type: Some("playlist".to_string()),
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_search_results(&response, "playlists"),
        )]))
    }
}

// ── video_details ────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "video_details",
    description = "Get detailed information about a YouTube video",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct VideoDetails {
    /// The video ID (e.g. 'dQw4w9WgXcQ')
    video_id: String,
}

impl VideoDetails {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &InvidiousClient,
    ) -> Result<CallToolResult, CallToolError> {
        let details = client
            .video_details(&self.video_id)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_video_details(&details),
        )]))
    }
}

// ── channel_details ──────────────────────────────────────────────────────────

#[mcp_tool(
    name = "channel_details",
    description = "Get detailed information about a YouTube channel",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct ChannelDetails {
    /// The channel ID
    channel_id: String,
}

impl ChannelDetails {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &InvidiousClient,
    ) -> Result<CallToolResult, CallToolError> {
        let details = client
            .channel_details(&self.channel_id)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_channel_details(&details),
        )]))
    }
}

// ── video_comments ───────────────────────────────────────────────────────────

#[mcp_tool(
    name = "video_comments",
    description = "Get comments for a YouTube video",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct VideoComments {
    /// The video ID
    video_id: String,
    /// Sort order: top (default), new
    sort_by: Option<String>,
}

impl VideoComments {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &InvidiousClient,
    ) -> Result<CallToolResult, CallToolError> {
        let response = client
            .comments(&self.video_id, self.sort_by.as_deref())
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_comments(&response),
        )]))
    }
}

// ── trending ─────────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "trending",
    description = "Get trending videos on YouTube via Invidious",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct Trending {
    /// Trending type filter: music, gaming, movies, default
    r#type: Option<String>,
    /// Region code (e.g. 'US', 'ES', 'JP')
    region: Option<String>,
}

impl Trending {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &InvidiousClient,
    ) -> Result<CallToolResult, CallToolError> {
        let response = client
            .trending(self.r#type.as_deref(), self.region.as_deref())
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_trending(&response),
        )]))
    }
}

// ── toolbox ───────────────────────────────────────────────────────────────────

use rust_mcp_sdk::tool_box;

tool_box!(
    InvidiousTools,
    [
        SearchVideos,
        SearchChannels,
        SearchPlaylists,
        VideoDetails,
        ChannelDetails,
        VideoComments,
        Trending,
    ]
);

// ── Formatting helpers ───────────────────────────────────────────────────────

use crate::invidious::{
    ChannelDetails as ChannelDetailsResponse, CommentsResponse, SearchResult, TrendingResponse,
    VideoDetails as VideoDetailsResponse,
};

fn format_search_results(results: &[SearchResult], category: &str) -> String {
    if results.is_empty() {
        return "No results found.".to_string();
    }

    let mut text = format!("## Search Results ({})\n\n", category);

    for (i, result) in results.iter().enumerate() {
        let title = if !result.title.is_empty() {
            &result.title
        } else {
            result.video_title.as_deref().unwrap_or("Untitled")
        };
        text.push_str(&format!("### {}. {}\n\n", i + 1, escape_md(title)));

        if let Some(ref desc) = result.description {
            let preview = truncate(desc, 200);
            text.push_str(&format!("{}\n\n", escape_md(&preview)));
        }

        if let Some(ref author) = result.author {
            text.push_str(&format!("- **Channel:** {}", escape_md(author)));
            if let Some(ref author_id) = result.author_id {
                text.push_str(&format!(" ({})", author_id));
            }
            text.push('\n');
        }

        if let Some(len) = result.length_seconds {
            text.push_str(&format!("- **Duration:** {}\n", format_duration(len)));
        }

        if let Some(vc) = result.view_count {
            text.push_str(&format!("- **Views:** {}\n", format_number(vc)));
        }

        if let Some(ref published) = result.published_text {
            text.push_str(&format!("- **Published:** {}\n", escape_md(published)));
        }

        if result.live_now == Some(true) {
            text.push_str("- **LIVE**\n");
        }

        if let Some(ref vid) = result.video_id {
            text.push_str(&format!("- **Video ID:** {}\n", vid));
        } else if let Some(ref pid) = result.playlist_id {
            text.push_str(&format!("- **Playlist ID:** {}\n", pid));
        }

        if let Some(ref vc) = result.video_count {
            text.push_str(&format!("- **Videos in playlist:** {}\n", vc));
        }

        if let Some(ref sc) = result.sub_count_text {
            text.push_str(&format!("- **Subscribers:** {}\n", escape_md(sc)));
        }

        text.push('\n');
    }

    text
}

fn format_video_details(details: &VideoDetailsResponse) -> String {
    let mut text = format!("## {}\n\n", escape_md(&details.title));

    text.push_str(&format!(
        "- **Channel:** {} ({})\n",
        escape_md(&details.author),
        details.author_id
    ));
    text.push_str(&format!("- **Video ID:** {}\n", details.video_id));

    if let Some(len) = details.length_seconds {
        text.push_str(&format!("- **Duration:** {}\n", format_duration(len)));
    }
    if let Some(vc) = details.view_count {
        text.push_str(&format!("- **Views:** {}\n", format_number(vc)));
    }
    if let Some(lc) = details.like_count {
        text.push_str(&format!("- **Likes:** {}\n", format_number(lc)));
    }
    if let Some(dc) = details.dislike_count {
        text.push_str(&format!("- **Dislikes:** {}\n", format_number(dc)));
    }
    if let Some(ref pt) = details.published_text {
        text.push_str(&format!("- **Published:** {}\n", escape_md(pt)));
    }
    if let Some(ref genre) = details.genre {
        text.push_str(&format!("- **Genre:** {}\n", escape_md(genre)));
    }
    if !details.keywords.is_empty() {
        text.push_str(&format!(
            "- **Keywords:** {}\n",
            details.keywords.join(", ")
        ));
    }

    if let Some(ref desc) = details.description {
        let preview = truncate(desc, 500);
        text.push_str(&format!("\n### Description\n\n{}\n", escape_md(&preview)));
    }

    text
}

fn format_channel_details(details: &ChannelDetailsResponse) -> String {
    let mut text = format!("## {}\n\n", escape_md(&details.author));

    text.push_str(&format!("- **Channel ID:** {}\n", details.author_id));

    if let Some(ref sc) = details.sub_count_text {
        text.push_str(&format!("- **Subscribers:** {}\n", escape_md(sc)));
    } else if let Some(sc) = details.sub_count {
        text.push_str(&format!("- **Subscribers:** {}\n", format_number(sc)));
    }

    if let Some(tv) = details.total_views {
        text.push_str(&format!("- **Total views:** {}\n", format_number(tv)));
    }

    if details.auto_generated == Some(true) {
        text.push_str("- **Auto-generated**\n");
    }

    if let Some(ref desc) = details.description {
        let preview = truncate(desc, 500);
        text.push_str(&format!("\n### Description\n\n{}\n", escape_md(&preview)));
    }

    text
}

fn format_comments(response: &CommentsResponse) -> String {
    if response.comments.is_empty() {
        return "No comments found.".to_string();
    }

    let mut text = format!("## Comments ({})\n\n", response.comments.len());

    for (i, comment) in response.comments.iter().enumerate() {
        text.push_str(&format!(
            "### {}. {}\n\n",
            i + 1,
            escape_md(&comment.author)
        ));

        if let Some(pt) = &comment.published_text {
            text.push_str(&format!("- **Date:** {}\n", escape_md(pt)));
        }
        if let Some(lc) = comment.like_count {
            text.push_str(&format!("- **Likes:** {}\n", format_number(lc)));
        }

        text.push_str(&format!("\n{}\n", escape_md(&comment.content)));

        if let Some(ref replies) = comment.replies {
            text.push_str(&format!("\n- **{} replies**\n", replies.reply_count));
        }

        text.push('\n');
    }

    if let Some(ref cont) = response.continuation {
        text.push_str(&format!("**Continuation token:** `{}`\n\n", cont));
    }

    text
}

fn format_trending(results: &[TrendingResponse]) -> String {
    if results.is_empty() {
        return "No trending videos found.".to_string();
    }

    let mut text = String::from("## Trending Videos\n\n");

    for (i, item) in results.iter().enumerate() {
        text.push_str(&format!("### {}. {}\n\n", i + 1, escape_md(&item.title)));

        text.push_str(&format!(
            "- **Channel:** {} ({})\n",
            escape_md(&item.author),
            item.author_id
        ));

        if let Some(len) = item.length_seconds {
            text.push_str(&format!("- **Duration:** {}\n", format_duration(len)));
        }
        if let Some(vc) = item.view_count {
            text.push_str(&format!("- **Views:** {}\n", format_number(vc)));
        }
        if let Some(ref pt) = item.published_text {
            text.push_str(&format!("- **Published:** {}\n", escape_md(pt)));
        }

        text.push_str(&format!("- **Video ID:** {}\n", item.video_id));
        text.push('\n');
    }

    text
}

fn escape_md(s: &str) -> String {
    s.replace('*', "\\*")
        .replace('_', "\\_")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('<', "\\<")
        .replace('>', "\\>")
}

fn format_duration(seconds: u64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    if h > 0 {
        format!("{}:{:02}:{:02}", h, m, s)
    } else {
        format!("{}:{:02}", m, s)
    }
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        let truncated: String = s
            .char_indices()
            .take_while(|(idx, _)| *idx <= max)
            .map(|(_, c)| c)
            .collect();
        format!("{}...", truncated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_duration_should_show_minutes_seconds() {
        assert_eq!(format_duration(0), "0:00");
        assert_eq!(format_duration(5), "0:05");
        assert_eq!(format_duration(59), "0:59");
        assert_eq!(format_duration(60), "1:00");
        assert_eq!(format_duration(125), "2:05");
    }

    #[test]
    fn format_duration_should_show_hours() {
        assert_eq!(format_duration(3600), "1:00:00");
        assert_eq!(format_duration(3661), "1:01:01");
        assert_eq!(format_duration(7325), "2:02:05");
    }

    #[test]
    fn format_number_should_format_thousands() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(999), "999");
        assert_eq!(format_number(1_000), "1.0K");
        assert_eq!(format_number(1_500), "1.5K");
        assert_eq!(format_number(999_999), "1000.0K");
    }

    #[test]
    fn format_number_should_format_millions() {
        assert_eq!(format_number(1_000_000), "1.0M");
        assert_eq!(format_number(2_500_000), "2.5M");
    }

    #[test]
    fn escape_md_should_escape_special_chars() {
        assert_eq!(escape_md("*bold*"), "\\*bold\\*");
        assert_eq!(escape_md("_italic_"), "\\_italic\\_");
        assert_eq!(escape_md("[link]"), "\\[link\\]");
        assert_eq!(escape_md("<tag>"), "\\<tag\\>");
    }

    #[test]
    fn truncate_should_not_truncate_when_under_limit() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_should_add_ellipsis_when_over_limit() {
        let result = truncate("hello world", 5);
        assert!(result.ends_with("..."));
        assert_eq!(result, "hello ...");
    }

    #[test]
    fn truncate_should_not_panic_on_multibyte_utf8() {
        assert_eq!(truncate("héllo wörld", 7), "héllo w...");
        assert_eq!(truncate("日本語テスト", 9), "日本語テ...");
    }
}
