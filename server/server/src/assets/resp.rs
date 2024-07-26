use {
    rouille::Response,
    rust_embed::EmbeddedFile,
    std::{borrow::Cow, path::Path},
};

pub fn extension_to_mime(extension: Option<&str>) -> &'static str {
    match extension {
        Some("css") => "text/css; charset=utf8",
        Some("gif") => "image/gif",
        Some("htm") => "text/html; charset=utf8",
        Some("html") => "text/html; charset=utf8",
        Some("ico") => "image/x-icon",
        Some("jpeg") => "image/jpeg",
        Some("jpg") => "image/jpeg",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("svg") => "image/svg+xml",
        Some("ttf") => "application/x-font-ttf",
        Some("txt") => "text/plain; charset=utf8",
        Some("woff") => "application/font-woff",
        Some("woff2") => "application/font-woff2",
        _ => "application/octet-stream",
    }
}

/// Builds a 200 `Response` from file.
#[inline]
pub fn from_file(file: EmbeddedFile, url: String) -> Response {
    let data = file.data;
    let extension = Path::new(url.as_str()).extension().and_then(|s| s.to_str());
    let content_type = extension_to_mime(extension);

    let mut response = Response::from_data(content_type, data);
    with_headers(&mut response);

    response
}

/// Builds a 200 `Response` with data.
#[inline]
pub fn from_data<C, D>(content_type: C, data: D) -> Response
where
    C: Into<Cow<'static, str>>,
    D: Into<Vec<u8>>,
{
    let mut response = Response::from_data(content_type, data);
    with_headers(&mut response);

    response
}

#[inline]
fn with_headers(response: &mut Response) {
    // Server
    response.headers.push(("Server".into(), "ids".into()));

    // Cors
    response.headers.push(("Access-Control-Allow-Origin".into(), "*".into()));
}

/// Builds an empty `Response` with a 404 status code.
#[inline]
pub fn not_found() -> Response {
    let mut response = Response::empty_404();
    with_headers(&mut response);

    response
}

/// Builds an error `Response`
#[inline]
pub fn err(message: &str, status: u16) -> Response {
    let mut response = rouille::Response::text(message).with_status_code(status);
    with_headers(&mut response);

    response
}
