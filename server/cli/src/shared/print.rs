use {crate::git::OrganizedStatus, lool::cli::stylize::Stylize};

#[inline]
pub fn logo() {
    println!("{}", "  ╭".white().bold());
    println!("{} {}", " { }".white().bold(), "ids".white().bold());
    println!();
}

#[inline]
pub fn git_status(status: &[OrganizedStatus]) {
    if status.is_empty() {
        return;
    }

    // join with \n the result of each to_string on the OrganizedStatus
    let result = status.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\n\n");
    println!("{}\n", result);
}
