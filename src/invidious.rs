use crate::config::AppConfig;
use crate::error::AppError;
use serde::Deserialize;

// ── Search response types ────────────────────────────────────────────────────

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    #[serde(rename = "type")]
    pub result_type: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub video_id: Option<String>,
    #[serde(default)]
    pub video_title: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub author_id: Option<String>,
    #[serde(default)]
    pub author_url: Option<String>,
    #[serde(default)]
    pub video_thumbnails: Vec<VideoThumbnail>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_html: Option<String>,
    #[serde(default)]
    pub length_seconds: Option<u64>,
    #[serde(default)]
    pub published: Option<u64>,
    #[serde(default)]
    pub published_text: Option<String>,
    #[serde(default)]
    pub view_count: Option<u64>,
    #[serde(default)]
    pub view_count_text: Option<String>,
    #[serde(default)]
    pub live_now: Option<bool>,
    #[serde(default)]
    pub premium: Option<bool>,
    #[serde(default)]
    pub is_upcoming: Option<bool>,
    // Channel/playlist specific
    #[serde(default)]
    pub playlist_id: Option<String>,
    #[serde(default)]
    pub video_count: Option<u64>,
    #[serde(default)]
    pub sub_count: Option<u64>,
    #[serde(default)]
    pub sub_count_text: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoThumbnail {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub width: Option<u64>,
    #[serde(default)]
    pub height: Option<u64>,
    #[serde(default)]
    pub quality: Option<String>,
}

// ── Video details ────────────────────────────────────────────────────────────

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub video_id: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub author_id: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_html: Option<String>,
    #[serde(default)]
    pub length_seconds: Option<u64>,
    #[serde(default)]
    pub view_count: Option<u64>,
    #[serde(default)]
    pub like_count: Option<u64>,
    #[serde(default)]
    pub dislike_count: Option<u64>,
    #[serde(default)]
    pub published: Option<u64>,
    #[serde(default)]
    pub published_text: Option<String>,
    #[serde(default)]
    pub video_thumbnails: Vec<VideoThumbnail>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub genre: Option<String>,
    #[serde(default)]
    pub genre_url: Option<String>,
}

// ── Channel details ──────────────────────────────────────────────────────────

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelDetails {
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub author_id: String,
    #[serde(default)]
    pub author_url: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_html: Option<String>,
    #[serde(default)]
    pub sub_count: Option<u64>,
    #[serde(default)]
    pub sub_count_text: Option<String>,
    #[serde(default)]
    pub total_views: Option<u64>,
    #[serde(default)]
    pub joined: Option<u64>,
    #[serde(default)]
    pub auto_generated: Option<bool>,
    #[serde(default)]
    pub author_thumbnails: Vec<VideoThumbnail>,
}

// ── Comments ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsResponse {
    #[serde(default)]
    pub comments: Vec<Comment>,
    #[serde(default)]
    pub continuation: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    pub comment_count: Option<u64>,
    #[allow(dead_code)]
    #[serde(default)]
    pub video_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub author_id: String,
    #[serde(default)]
    pub author_url: Option<String>,
    #[serde(default)]
    pub author_thumbnails: Vec<VideoThumbnail>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub content_html: Option<String>,
    #[serde(default)]
    pub is_favorited: Option<bool>,
    #[serde(default)]
    pub like_count: Option<u64>,
    #[serde(default)]
    pub published: Option<u64>,
    #[serde(default)]
    pub published_text: Option<String>,
    #[serde(default)]
    pub replies: Option<CommentReplies>,
    #[serde(default)]
    pub comment_id: Option<String>,
    #[serde(default)]
    pub verified: Option<bool>,
    #[serde(default)]
    pub is_pinned: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentReplies {
    #[serde(default)]
    pub reply_count: u64,
    #[serde(default)]
    pub continuation: Option<String>,
}

// ── Trending ─────────────────────────────────────────────────────────────────

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendingResponse {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub video_id: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub author_id: String,
    #[serde(default)]
    pub author_url: Option<String>,
    #[serde(default)]
    pub video_thumbnails: Vec<VideoThumbnail>,
    #[serde(default)]
    pub length_seconds: Option<u64>,
    #[serde(default)]
    pub view_count: Option<u64>,
    #[serde(default)]
    pub published: Option<u64>,
    #[serde(default)]
    pub published_text: Option<String>,
}

// ── Client ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: String,
    pub page: Option<u32>,
    pub sort_by: Option<String>,
    pub date: Option<String>,
    pub duration: Option<String>,
    pub r#type: Option<String>,
}

pub struct InvidiousClient {
    http: reqwest::Client,
    base_url: String,
    default_lang: String,
}

impl InvidiousClient {
    pub fn new(config: &AppConfig) -> Result<Self, AppError> {
        let mut builder = reqwest::Client::builder();
        if config.invidious_insecure {
            builder = builder.danger_accept_invalid_certs(true);
        }
        Ok(Self {
            http: builder.build().map_err(AppError::Http)?,
            base_url: config.invidious_url.trim_end_matches('/').to_string(),
            default_lang: config.default_lang.clone(),
        })
    }

    pub async fn search(&self, params: SearchParams) -> Result<Vec<SearchResult>, AppError> {
        let query_params: Vec<(&str, String)> = [
            Some(("q", params.query)),
            params.page.map(|p| ("page", p.to_string())),
            params.sort_by.map(|s| ("sort_by", s)),
            params.date.map(|d| ("date", d)),
            params.duration.map(|d| ("duration", d)),
            params.r#type.map(|t| ("type", t)),
            Some(("hl", self.default_lang.clone())),
        ]
        .into_iter()
        .flatten()
        .collect();

        let url = format!("{}/api/v1/search", self.base_url);
        let resp = self.http.get(&url).query(&query_params).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::InvidiousApi(format!("HTTP {status}: {body}")));
        }

        let results: Vec<SearchResult> = resp.json().await?;
        Ok(results)
    }

    pub async fn video_details(&self, video_id: &str) -> Result<VideoDetails, AppError> {
        let url = format!("{}/api/v1/videos/{}", self.base_url, video_id);
        let resp = self.http.get(&url).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::InvidiousApi(format!("HTTP {status}: {body}")));
        }

        let details: VideoDetails = resp.json().await?;
        Ok(details)
    }

    pub async fn channel_details(&self, channel_id: &str) -> Result<ChannelDetails, AppError> {
        let url = format!("{}/api/v1/channels/{}", self.base_url, channel_id);
        let resp = self.http.get(&url).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::InvidiousApi(format!("HTTP {status}: {body}")));
        }

        let details: ChannelDetails = resp.json().await?;
        Ok(details)
    }

    pub async fn comments(
        &self,
        video_id: &str,
        sort_by: Option<&str>,
    ) -> Result<CommentsResponse, AppError> {
        let query_params: Vec<(&str, String)> = [sort_by.map(|s| ("sort_by", s.to_string()))]
            .into_iter()
            .flatten()
            .collect();

        let url = format!("{}/api/v1/comments/{}", self.base_url, video_id);
        let resp = self.http.get(&url).query(&query_params).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::InvidiousApi(format!("HTTP {status}: {body}")));
        }

        let response: CommentsResponse = resp.json().await?;
        Ok(response)
    }

    pub async fn trending(
        &self,
        r#type: Option<&str>,
        region: Option<&str>,
    ) -> Result<Vec<TrendingResponse>, AppError> {
        let query_params: Vec<(&str, String)> = [
            r#type.map(|t| ("type", t.to_string())),
            region.map(|r| ("region", r.to_string())),
            Some(("hl", self.default_lang.clone())),
        ]
        .into_iter()
        .flatten()
        .collect();

        let url = format!("{}/api/v1/trending", self.base_url);
        let resp = self.http.get(&url).query(&query_params).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::InvidiousApi(format!("HTTP {status}: {body}")));
        }

        let response: Vec<TrendingResponse> = resp.json().await?;
        Ok(response)
    }
}
