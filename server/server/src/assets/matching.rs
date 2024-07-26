use {
    super::{resp, Public},
    rouille::{Request, Response},
};

pub fn get(request: &Request, spa: bool) -> Response {
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
        None => {
            // if spa is enabled, serve the index.html file if the file is not found
            if spa {
                let index = Public::get("index.html").unwrap().data;
                resp::from_data("text/html", index)
            } else {
                resp::not_found()
            }
        }
        Some(file) => resp::from_file(file, url),
    }
}
