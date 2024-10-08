use askama::Template;
use axum::{extract::State, response::Html};
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use tracing::warn;

use crate::routing::SharedState;

#[derive(Debug, Deserialize)]
struct VideoItem {
    etag: String,
    id: ResourceId,
    snippet: Snippet,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Snippet {
    title: String,
    description: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct ResourceId {
    videoId: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct ApiResponse {
    items: Vec<VideoItem>,
}

#[derive(Debug)]
pub struct VideoInfo {
    url: String,
    title: String,
    // description: String,
}

impl From<VideoItem> for VideoInfo {
    fn from(value: VideoItem) -> Self {
        let title = html_escape::decode_html_entities(&value.snippet.title).to_string();
        // let description = html_escape::decode_html_entities(&value.snippet.description).to_string();

        Self {
            url: format!("https://www.youtube.com/embed/{}", value.id.videoId),
            title,
            // description,
        }
    }
}

async fn get_video_urls() -> anyhow::Result<Vec<VideoInfo>> {
    dotenv().ok();
    let api_key = env::var("YOUTUBE_API_KEY").expect("failed to get youtube api key");
    let channel_id = env::var("YOUTUBE_CHANNEL_ID").expect("failed to get youtube api key");

    let client = Client::new();
    let url = format!(
        "https://www.googleapis.com/youtube/v3/search?key={api_key}&channelId={channel_id}&part=snippet,id&order=date&maxResults=20&publishedAfter=2023-01-01T00:00:00Z"
    );

    let response = client.get(&url).send().await?;

    let body: serde_json::Value = response.json().await?;
    warn!("got response: {body:#?}");

    let api_response: ApiResponse =
        serde_json::from_value(body).expect("failed to parse json as ApiResponse");
    let videos: Vec<VideoInfo> = api_response
        .items
        .into_iter()
        .map(|item| VideoInfo::from(item))
        .collect();

    Ok(videos)
}

#[derive(Template, Debug)]
#[template(path = "videos.html")]
pub struct VideosTemplate<'s> {
    videos: &'s Vec<VideoInfo>,
}

pub async fn videos(State(data): State<SharedState>) -> Html<String> {
    let r = data.read().await;
    if r.videos_cache.is_some() {
        warn!("videos are cached");
        let tmpl = VideosTemplate {
            videos: r.videos_cache.as_ref().unwrap(),
        };
        return match tmpl.render() {
            Ok(r) => Html(r),
            Err(err) => Html(format!("Error rendering Layout: {}", err.to_string())),
        };
    }
    drop(r);

    warn!("videos not cached, getting");
    let videos = get_video_urls().await.expect("Failed to get videos");
    let mut w = data.write().await;
    w.videos_cache = Some(videos);
    let tmpl = VideosTemplate {
        videos: w.videos_cache.as_ref().unwrap(),
    };
    return match tmpl.render() {
        Ok(r) => Html(r),
        Err(err) => Html(format!("Error rendering Layout: {}", err.to_string())),
    };
}

mod tests {
    use std::sync::LazyLock;

    use crate::TRACING;

    #[tokio::test]
    async fn get_urls_works() {
        LazyLock::force(&TRACING);
        if let Err(err) = super::get_video_urls().await {
            panic!("{err:#?}")
        }
    }
}
