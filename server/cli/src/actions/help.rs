use {
    crate::print,
    lool::{cli::stylize::Stylize, s},
};

/// prints the help message of the CLI tool
pub fn show() {
    print::logo();

    print_section("usage", vec![("ids", Some("[options] [args...]"), None)]);
    println!(
        "\n{} {} helps you visualize icons you modified\nor added to a git repository during development\n",
        "ids".bold(),
        "(icons dev server)".italic()
    );

    print_section(
        "options",
        vec![
            ("--help, -h", None, Some("show this help message and exit")),
            ("--version, -v", None, Some("show version information")),
            ("--port, -v", None, Some("set the port to run the server on (defaults to 8788)")),
            ("--host, -h", None, Some("set the host to run the server on")),
            ("--no-spa", None, Some("deactivate SPA (single page application) beahvior")),
            ("--no-ui", None, Some("do not serve the UI")),
            (
                "--dir, -d",
                None,
                Some("set the directory to run the command in (defaults to current)"),
            ),
        ],
    );
}

/// prints a section with a title and a list of tuples with the following format:
/// (name/command, aliases<opt>, description<opt>)
#[inline]
fn print_section(title: &str, contents: Vec<(&str, Option<&str>, Option<&str>)>) {
    println!("{}", title.blue().bold());

    // we need the max length of the first element to align the rest
    let max_len = contents.iter().map(|(name, _, _)| name.len()).max().unwrap();

    for (name, aliases, description) in contents {
        let padding = " ".repeat(max_len - name.len());
        let name = name.white().bold();
        let aliases = aliases.map(|a| format!(" {}", a.dim())).unwrap_or(s!(""));
        let description = description.map(|d| format!("  {}", d)).unwrap_or_default();

        println!("  {}{}{}{}", name, padding, aliases, description);
    }
}
