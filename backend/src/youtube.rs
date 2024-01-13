use async_graphql::SimpleObject;
use google_youtube3::client::NoToken;
use google_youtube3::hyper::client::HttpConnector;
use google_youtube3::{hyper, Error, YouTube};
use hyper_rustls::HttpsConnector;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

pub type Youtube = Arc<YoutubeClient>;

#[derive(Debug)]
pub enum YoutubeError {
    Unknown(Error),
    NotFound,
}

impl Display for YoutubeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            YoutubeError::Unknown(e) => write!(f, "Youtube Error: {}", e),
            YoutubeError::NotFound => write!(f, "Youtube Error: Not Found"),
        }
    }
}

pub struct YoutubeClient {
    key: String,
    hub: YouTube<HttpsConnector<HttpConnector>>,
}

#[derive(Debug, SimpleObject)]
pub struct Video {
    pub id: String,
    pub url: String,
    pub platform: String,
    pub title: String,
    pub description: String,
    pub channel: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
    pub thumbnail_url: String,
}

impl YoutubeClient {
    pub fn new(key: String) -> Self {
        let hub = YouTube::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
            NoToken,
        );

        Self { key, hub }
    }

    pub async fn query(&self, term: String, max_results: u32) -> Result<Vec<Video>, YoutubeError> {
        let results = self
            .hub
            .search()
            .list(&vec!["snippet".into()])
            .q(term.as_str())
            .safe_search("strict")
            .max_results(max_results)
            .param("key", self.key.as_str())
            .doit()
            .await
            .map_err(YoutubeError::Unknown)?
            .1
            .items;

        match results {
            None => Ok(Vec::new()),
            Some(e) => Ok(e
                .into_iter()
                .filter(|e| e.id.clone().unwrap().kind.unwrap() == "youtube#video")
                .map(|e| {
                    let snippet = e.snippet.unwrap();
                    let id = e.id.unwrap();

                    Video {
                        id: id.video_id.clone().unwrap(),
                        platform: "youtube".into(),
                        title: snippet.title.unwrap(),
                        description: snippet.description.unwrap(),
                        published_at: snippet.published_at.unwrap(),
                        channel: snippet.channel_title.unwrap(),
                        url: format!("https://www.youtube.com/watch?v={}", id.video_id.unwrap()),
                        thumbnail_url: snippet.thumbnails.unwrap().medium.unwrap().url.unwrap(),
                    }
                })
                .collect()),
        }
    }
}
