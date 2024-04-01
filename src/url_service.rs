pub mod interface;

use anyhow::anyhow;
use axum::extract::Host;
use std::collections::HashMap;

use self::interface::{UrlKey, UrlShortener};

pub(crate) struct UrlService<T> {
    url_shortener: T,
}

impl UrlService<InMemoryUrlShortener> {
    pub(crate) fn new() -> Self {
        Self {
            url_shortener: InMemoryUrlShortener::new(),
        }
    }
}

impl<T> UrlShortener for UrlService<T>
where
    T: UrlShortener,
{
    type Key = T::Key;
    type Error = T::Error;

    fn shorten_url(&mut self, url: &str) -> Self::Key {
        self.url_shortener.shorten_url(url)
    }

    fn get_url(&self, key: &Self::Key) -> Result<&str, Self::Error> {
        self.url_shortener.get_url(key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct InMemoryUrlKey {
    key: String,
}

impl UrlKey for InMemoryUrlKey {
    fn from_url(url: &str) -> anyhow::Result<Self> {
        let key = url
            .rsplit_once('/')
            .ok_or_else(|| anyhow!("Invalid URL : {url}"))?
            .1
            .to_string();
        Ok(Self { key })
    }

    fn from_id(key_id: &str) -> Self {
        Self {
            key: key_id.to_owned(),
        }
    }

    fn generate_random() -> Self {
        let key = nanoid::nanoid!(5);
        Self { key }
    }

    fn build_url(&self, host: &Host) -> String {
        format!("{}/{}", host.0, self.key)
    }
}

pub(crate) struct InMemoryUrlShortener {
    keys_to_urls: HashMap<InMemoryUrlKey, String>,
    urls_to_keys: HashMap<String, InMemoryUrlKey>,
}

impl InMemoryUrlShortener {
    pub(crate) fn new() -> Self {
        Self {
            keys_to_urls: HashMap::new(),
            urls_to_keys: HashMap::new(),
        }
    }
}

impl UrlShortener for InMemoryUrlShortener {
    type Key = InMemoryUrlKey;
    type Error = anyhow::Error;

    fn shorten_url(&mut self, url: &str) -> Self::Key {
        match self.urls_to_keys.get(url) {
            Some(key) => key.clone(),
            None => {
                let key = InMemoryUrlKey::generate_random();
                self.keys_to_urls.insert(key.clone(), url.to_string());
                self.urls_to_keys.insert(url.to_string(), key.clone());
                key
            }
        }
    }

    fn get_url(&self, key: &Self::Key) -> anyhow::Result<&str> {
        match self.keys_to_urls.get(key) {
            Some(url) => Ok(url.as_str()),
            None => Err(anyhow!("URL not found for key: {:?}", key)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_key_from_url() {
        let url = "https://example.com/abc".to_string();
        let key = InMemoryUrlKey::from_url(&url).unwrap();
        assert_eq!(key.key.as_str(), "abc");
    }

    #[test]
    fn test_in_memory_url_service() {
        let mut service = InMemoryUrlShortener::new();
        let url = "https://example.com/abc";
        let key = service.shorten_url(url);
        assert_eq!(service.get_url(&key).unwrap(), url);
    }

    #[test]
    fn test_in_memory_url_service_same_key() {
        let mut service = InMemoryUrlShortener::new();
        let url = "https://example.com/abc";
        let key = service.shorten_url(url);
        let key2 = service.shorten_url(url);
        assert_eq!(key, key2);
    }

    #[test]
    fn test_in_memory_url_service_url_not_present() {
        let service = InMemoryUrlShortener::new();
        let key = UrlKey::generate_random();
        assert!(service.get_url(&key).is_err());
    }
}
