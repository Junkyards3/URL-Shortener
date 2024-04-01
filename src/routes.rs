use std::sync::{Arc, Mutex};

use axum::{
    extract::{Host, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;

use crate::{
    templates::{HomeTemplate, HtmlTemplate, KeyFilledTemplate},
    url_service::{
        interface::{UrlKey, UrlShortener},
        UrlService,
    },
};

#[derive(Deserialize)]
pub(crate) struct UrlToShorten {
    url: String,
}

pub(crate) async fn home_page() -> impl IntoResponse {
    HtmlTemplate(HomeTemplate {})
}

pub(crate) async fn shorten_url<T: UrlShortener>(
    host: Host,
    State(url_service): State<Arc<Mutex<UrlService<T>>>>,
    Form(url_to_shorten): Form<UrlToShorten>,
) -> impl IntoResponse {
    let mut url_service = url_service.lock().expect("error locking url_service");
    let url = url_to_shorten.url.as_str();
    let key = url_service.shorten_url(url);
    HtmlTemplate(KeyFilledTemplate {
        base_url: url.to_string(),
        shortened_url: key.build_url(&host).to_string(),
    })
}

pub(crate) async fn redirect<T: UrlShortener>(
    State(url_service): State<Arc<Mutex<UrlService<T>>>>,
    Path(key_id): Path<String>,
) -> Result<Redirect, StatusCode> {
    let url_service = url_service.lock().expect("error locking url_service");
    let key = T::Key::from_id(key_id.as_str());
    match url_service.get_url(&key) {
        Ok(url) => Ok(Redirect::temporary(url)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
