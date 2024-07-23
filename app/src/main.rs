mod build_info;

use {
    cli::actions::{
        get_action, help, version,
        Action::{self, *},
    },
    eyre::Result,
};

fn main() -> Result<()> {
    let (action, _) = get_action()?;

    handle_action(action)
}

fn handle_action(action: Action) -> Result<()> {
    match action {
        Help => help::show(),
        Version => version::show(build_info::get()),
        Server(path, port, host) => server::start(path, port, host),
    }

    Ok(())
}
