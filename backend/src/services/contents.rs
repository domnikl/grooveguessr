use crate::youtube::{Video, Youtube};

use super::Error;

pub struct ContentsService<'a> {
    youtube_client: &'a Youtube,
}

impl<'a> ContentsService<'a> {
    pub fn new(youtube_client: &'a Youtube) -> Self {
        Self { youtube_client }
    }

    pub async fn search(&self, term: &str) -> Result<Vec<Video>, Error> {
        let youtube_client = self.youtube_client;

        youtube_client
            .query(term.to_string(), 10)
            .await
            .map_err(Error::Youtube)
    }
}
