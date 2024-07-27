//! Contents js generator
//!
//! This module is responsible for generating the contents.json file that is used by the frontend to
//! display the current changes in the repository. The contents.json file is a json file that
//! contains the current changes in the repository as a JSON object. It is served by the http asset
//! server under the `_ids_runtime` prefix (`/_ids_runtime/contents.json`) and it's regenerated
//! every time a applicable change is detected in the repository.

use {
    cli::git::{GitFileStatus, GitStatus, OrganizedStatus},
    std::path::PathBuf,
};

const NAME: &str = "contents.json";

pub fn generate(status: &[OrganizedStatus], link_prefix: &str) -> (String, Vec<u8>) {
    fn parse_git_file_status(file: &GitFileStatus, link_prefix: &str) -> String {
        let status = escape_json_string(file.status.to_string());
        let path = escape_json_string(file.path.display().to_string());

        // join the link prefix with the file.link
        let prefix = PathBuf::from(link_prefix);
        let link =
            escape_json_string(prefix.join(&file.link).display().to_string().replace('\\', "/"));

        let timestamp = time::OffsetDateTime::now_utc().unix_timestamp_nanos();

        format! {r#"{{"status": "{status}","path": "{path}","link": "{link}?{timestamp}"}}"#}
    }

    fn parse_organized_status(status: &OrganizedStatus, link_prefix: &str) -> String {
        let dir = escape_json_string(status.dir.clone());
        let files = status
            .files
            .iter()
            // filter out those files with deleted status
            .filter(|file| !matches!(file.status, GitStatus::Deleted))
            .map(|file| parse_git_file_status(file, link_prefix))
            .collect::<Vec<String>>()
            .join(",\n");

        format! {r#"{{"dir": "{dir}","files": [{files}]}}"#}
    }

    let status_items = status
        .iter()
        .map(|s| parse_organized_status(s, link_prefix))
        .collect::<Vec<String>>()
        .join(",\n");

    let status = format! {r#"[{status_items}]"#};

    (NAME.to_string(), status.as_bytes().to_vec())
}

fn escape_json_string(s: String) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
