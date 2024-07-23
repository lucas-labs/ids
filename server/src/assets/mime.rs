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
