//! ### Http
//!
//! Serves the following under the same host/port, using `rouille`:
//! - **Frontend**: preact embedded app
//! - **Websocket Server**: `/__ws` endpoint to send `refresh` messages to the clients, so they can
//!   refresh the page and get the latest changes

mod websocket;

use {
    crate::{
        assets::{self, resp, RuntimeAssets},
        Server,
    },
    bus::Bus,
    rouille::{self, Request, Response, Server as RouilleServer},
    std::{
        net::ToSocketAddrs,
        sync::{Arc, Mutex},
        thread::spawn,
    },
};

impl Server {
    pub(crate) fn serve_http_and_socket(&self) {
        self.log_url();

        // spawn and run the http server in a new thread
        let host = self.cfg.host.clone();
        let port = self.cfg.port;
        let runtime_assets = self.runtime_assets.clone();
        let spa = self.cfg.spa;
        let serve_ui = self.cfg.serve_ui;
        let bus = self.ws_bus.clone();

        spawn(move || http(host, port, runtime_assets, spa, serve_ui, bus));
    }
}

fn http(
    host: String,
    port: u32,
    runtime_assets: Arc<Mutex<RuntimeAssets>>,
    spa: bool,
    serve_ui: bool,
    bus: Arc<Mutex<Bus<String>>>,
) {
    let url = format!("{}:{}", host, port);
    let prefix = {
        let runtime_assets = runtime_assets.lock().unwrap();
        runtime_assets.prefix()
    };

    start_server(url, move |request| {
        // if its a GET to /{prefix}/ws, serve as websocket
        if request.method() == "GET" && request.url().starts_with(&format!("{prefix}/ws")) {
            return websocket::handle(request, bus.clone());
        }

        // if the request starts with the prefix, serve the asset from the runtime assets
        if request.url().starts_with(&prefix) {
            let runtime_assets = runtime_assets.lock();

            if let Ok(runtime_assets) = runtime_assets {
                let file = runtime_assets.get(request.url());
                match file {
                    Some(file) => return resp::from_data(file.mime, file.content),
                    None => return resp::not_found(),
                }
            } else {
                return resp::err("error aquiring runtime assets", 500);
            }
        }

        if serve_ui {
            let response = assets::get(request, spa);
            if response.is_success() {
                return response;
            }
        }

        resp::not_found()
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
