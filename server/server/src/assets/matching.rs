use {
    rouille::{Request, Response},
    std::path::Path,
};

use super::{mime::extension_to_mime, Public};

pub fn get(request: &Request) -> Response {
    let mut url = request.url();

    if url.ends_with('/') {
        url.push_str("index.html");
    }

    if url.starts_with('/') {
        // remove the / at the beginning
        url = url[1..].to_string();
    }

    let potential_file = Public::get(url.as_str());

    match potential_file {
        None => Response::empty_404(),
        Some(file) => {
            let extension = Path::new(url.as_str()).extension().and_then(|s| s.to_str());

            // let meta = file.metadata;
            let file = file.data;

            // let now = time::OffsetDateTime::now_local()
            //     .unwrap_or_else(|_| time::OffsetDateTime::now_utc());
            // let etag: String = (meta.last_modified().unwrap_or(now.nanosecond() as u64)
            //     ^ 0xd3f4_0305_c9f8_e911_u64)
            //     .to_string();

            Response::from_data(extension_to_mime(extension), file) //.with_etag(request, etag);
                                                                    // .with_public_cache(3600);
        }
    }
}
