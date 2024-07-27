use {
    crate::assets::RuntimeAssets,
    bus::Bus,
    cli::{git, print, CliConfig},
    crossbeam_channel::{select, unbounded, Receiver},
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
    cfg: CliConfig,
    repo_path: PathBuf,
    runtime_assets: Arc<Mutex<RuntimeAssets>>,
    runtime_assets_prefix: String,
    ws_bus: Arc<Mutex<Bus<String>>>,
}

impl Server {
    pub fn create(cfg: CliConfig, repo_path: PathBuf) -> Self {
        let runtime_assets_prefix = s!("/_ids_runtime");

        Self {
            cfg,
            repo_path,
            runtime_assets: Arc::new(Mutex::new(RuntimeAssets::create(&runtime_assets_prefix))),
            runtime_assets_prefix,
            ws_bus: Arc::new(Mutex::new(Bus::new(50))),
        }
    }

    pub fn log_url(&self) {
        let url = format!("http://{}:{}", self.cfg.host, self.cfg.port).green();
        println!("» Serving {} on {}", "ids".magenta().bold(), url);
    }

    pub fn start(&mut self) {
        print::logo();

        // initial asset generation
        self.generate_assets();

        let watcher_rcv = self.start_watcher();
        self.serve_http_and_socket();

        self.handle_messages(watcher_rcv);
    }

    /// receives from the watcher and:
    /// - generates the js or json file with the changed files
    /// - sends a refresh signal to the web socket
    pub fn handle_messages(&mut self, watcher_rcv: Receiver<()>) {
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
                    let mut bus = self.ws_bus.lock().unwrap();
                    let _ = bus.try_broadcast("R".to_string());
                },
                recv(exit_rx) -> _ => break
            }
        }
    }
}

/** Create a new server instance and start it */
pub fn start(config: CliConfig) {
    // ensure the given path is in a git repository
    if let Ok(repo_path) = git::get_repo_top_level(&config.path) {
        let mut server = Server::create(config, repo_path);
        server.start();
    } else {
        eprintln!("{} » the given path is not a git repository!", "error".red().bold());
        eprintln!("{} {}", "path:".yellow(), config.path.display());
        std::process::exit(1);
    }
}
