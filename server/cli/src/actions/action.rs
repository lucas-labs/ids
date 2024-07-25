use {
    crate::shared::config::CliConfig,
    eyre::Result,
    pico_args::Arguments,
    std::{env::current_dir, ffi::OsString, path::PathBuf},
};

pub enum Action {
    Help,
    Version,
    Server(CliConfig),
}

pub fn get_action() -> Result<(Action, Vec<OsString>)> {
    let mut arguments = Arguments::from_env();

    // check if wants help
    if arguments.contains(["-h", "--help"]) {
        return Ok((Action::Help, arguments.finish()));
    }

    // check if wants version
    if arguments.contains(["-v", "--version"]) {
        return Ok((Action::Version, arguments.finish()));
    }

    let curr_dir = current_dir()?;
    let dir_opt: Option<String> = arguments.opt_value_from_str(["-d", "--dir"])?;

    let port: u32 = arguments.opt_value_from_str(["-p", "--port"])?.unwrap_or(8788);
    let host: String =
        arguments.opt_value_from_str(["-h", "--host"])?.unwrap_or("localhost".into());

    // spa: defaults to true, if --no-spa is passed, it will be false
    let spa: bool = !arguments.contains("--no-spa");

    // serve_ui: defaults to true, if --no-ui is passed, it will be false
    let serve_ui: bool = !arguments.contains("--no-ui");

    let rest = arguments.finish();

    // cwd is dir_opt if it exists, otherwise curr_dir
    let cwd: PathBuf = match dir_opt {
        Some(dir) => {
            let path = PathBuf::from(dir);
            if path.is_absolute() {
                path
            } else {
                curr_dir.join(path)
            }
        }
        None => curr_dir,
    };

    let config = CliConfig {
        path: std::path::absolute(cwd)?,
        port,
        host,
        spa,
        serve_ui,
    };

    Ok((Action::Server(config), rest))
}
