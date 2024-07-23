use {
    crate::assets::RuntimeAssets,
    cli::{git, print},
    crossbeam_channel::{select, unbounded, Receiver, Sender},
    lool::{cli::stylize::Stylize, s},
    std::{
        path::PathBuf,
        sync::{Arc, Mutex},
    },
};

mod generator;
mod http;
mod watcher;

pub struct Server {
    path: PathBuf,
    port: u32,
    host: String,
    runtime_assets: Arc<Mutex<RuntimeAssets>>,
    runtime_assets_prefix: String,
}

impl Server {
    pub fn create(path: PathBuf, port: u32, host: String) -> Self {
        let runtime_assets_prefix = s!("/_ids_runtime");

        Self {
            path,
            port,
            host,
            runtime_assets: Arc::new(Mutex::new(RuntimeAssets::create(&runtime_assets_prefix))),
            runtime_assets_prefix,
        }
    }

    pub fn log_url(&self) {
        let url = format!("http://{}:{}", self.host, self.port).green();
        println!("» Serving {} on {}", "ids".magenta().bold(), url);
    }

    pub fn start(&self) {
        print::logo();

        // initial asset generation
        self.generate_assets();

        // create mpsc for the watcher
        // let (watcher_snd, watcher_rcv) = mpsc::channel::<>();
        // create mpsc for the web socket
        let (ws_snd, _ws_rcv) = unbounded::<String>();

        let watcher_rcv = self.start_watcher();
        self.serve_http_and_socket();

        self.handle_messages(ws_snd, watcher_rcv);
    }

    /// receives from the watcher and:
    /// - generates the js or json file with the changed files
    /// - sends a refresh signal to the web socket
    pub fn handle_messages(&self, ws_snd: Sender<String>, watcher_rcv: Receiver<()>) {
        let (exit_tx, exit_rx) = unbounded();

        ctrlc::set_handler(move || {
            exit_tx.send(()).unwrap();
        })
        .expect("Error setting Ctrl-C handler");

        loop {
            select! {
                recv(watcher_rcv) -> _ => {
                    println!("» Changes detected: \n");
                    self.generate_assets();
                    self.log_url();

                    // send a refresh signal to the web socket
                    ws_snd.send("R".to_string()).unwrap();
                },
                recv(exit_rx) -> _ => break
            }
        }
    }
}

/** Create a new server instance and start it */
pub fn start(path: PathBuf, port: u32, host: String) {
    // ensure the given path is a git repository
    if !git::is_inside_repo(&path) {
        eprintln!("{} » the given path is not a git repository!", "error".red().bold());
        eprintln!("{} {}", "path:".yellow(), path.display());
        std::process::exit(1);
    }

    let server = Server::create(path, port, host);
    server.start();
}