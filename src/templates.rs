use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

pub(crate) struct HtmlTemplate<T>(pub(crate) T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "home.html")]
pub(crate) struct HomeTemplate;

#[derive(Template)]
#[template(path = "key_filled.html")]
pub(crate) struct KeyFilledTemplate {
    pub(crate) base_url: String,
    pub(crate) shortened_url: String,
}
