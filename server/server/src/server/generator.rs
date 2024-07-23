//! ### Http
//!
//! Serves the following under the same host/port, using `rouille`:
//! - **Frontend**: preact embedded app
//! - **Websocket Server**: `/__ws` endpoint to send `refresh` messages to the clients, so they can
//!   refresh the page and get the latest changes

use crate::{assets::contents, Server};

impl Server {
    /// generates the runtime assets:
    /// - the contents.json file
    /// - svg files (not ready to be implemented yet, but it's a placeholder)
    pub(crate) fn generate_assets(&self) {
        let runtime_assets = self.runtime_assets.clone();

        // get status from git
        let status = cli::git::status(&self.path, Some(vec![".svg"]));
        cli::print::git_status(&status);

        let (json_path, json_content) = contents::json::generate(
            &status,
            format!("{}/svg", self.runtime_assets_prefix).as_str(),
        );

        let mut files = vec![];

        for group in status {
            let dir = group.dir;
            for file in group.files {
                let file_name = file.path.file_name();
                // insert each file in /svg/<dir>/<file>
                let path = format!("svg/{}/{}", dir, file_name.unwrap().to_str().unwrap());

                if let Ok(content) = std::fs::read(file.path) {
                    files.push((path, content));
                }
            }
        }

        {
            let mut runtime_assets = runtime_assets.lock().unwrap();
            runtime_assets.insert(json_path.as_str(), json_content);

            // remove everything from the /svg folder
            runtime_assets.remove_path("svg/");

            // insert each of the svg files
            for (path, content) in files {
                runtime_assets.insert(path.as_str(), content);
            }
        }
    }
}
