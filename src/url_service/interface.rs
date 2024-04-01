use axum::extract::Host;

pub(crate) trait UrlKey {
    fn from_url(url: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn from_id(key_id: &str) -> Self;
    fn build_url(&self, host: &Host) -> String;
    fn generate_random() -> Self;
}

pub(crate) trait UrlShortener {
    type Key: UrlKey;
    type Error;

    fn shorten_url(&mut self, url: &str) -> Self::Key;
    fn get_url(&self, key: &Self::Key) -> Result<&str, Self::Error>;
}
