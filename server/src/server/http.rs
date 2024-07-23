//! ### Http
//!
//! Serves the following under the same host/port, using `rouille`:
//! - **Frontend**: preact embedded app
//! - **Websocket Server**: `/__ws` endpoint to send `refresh` messages to the clients, so they can
//!   refresh the page and get the latest changes

use std::{
    net::ToSocketAddrs,
    sync::{Arc, Mutex},
    thread::spawn,
};

use {
    crate::{
        assets::{self, RuntimeAssets},
        Server,
    },
    rouille::{self, Request, Response, Server as RouilleServer},
};

impl Server {
    pub(crate) fn serve_http_and_socket(&self) {
        self.log_url();

        // spawn and run the http server in a new thread
        let host = self.host.clone();
        let port = self.port;
        let runtime_assets = self.runtime_assets.clone();

        spawn(move || http(host, port, runtime_assets));
    }
}

fn http(host: String, port: u32, runtime_assets: Arc<Mutex<RuntimeAssets>>) {
    let url = format!("{}:{}", host, port);
    let prefix = {
        let runtime_assets = runtime_assets.lock().unwrap();
        runtime_assets.prefix()
    };

    start_server(url, move |request| {
        // if the request starts with the prefix, serve the asset from the runtime assets
        if request.url().starts_with(&prefix) {
            let runtime_assets = runtime_assets.lock();

            if let Ok(runtime_assets) = runtime_assets {
                let file = runtime_assets.get(request.url());
                match file {
                    Some(file) => return rouille::Response::from_data(file.mime, file.content),
                    None => return rouille::Response::empty_404(),
                }
            } else {
                return rouille::Response::text("error aquiring runtime assets")
                    .with_status_code(500);
            }
        }

        let response = assets::get(request);
        if response.is_success() {
            return response;
        }

        rouille::Response::empty_404()
    });
}

pub fn start_server<A, F>(addr: A, handler: F)
where
    A: ToSocketAddrs,
    F: Send + Sync + 'static + Fn(&Request) -> Response,
{
    RouilleServer::new(addr, handler).expect("Failed to start server").run();
    println!("» Server stopped");
}
