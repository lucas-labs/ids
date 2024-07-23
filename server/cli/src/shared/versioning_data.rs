use {
    core::fmt,
    std::fmt::{Display, Formatter},
};

/// ### Versioning/Build data
/// This struct holds basic information about the binary.
///
/// _intended to be used with the `built` crate and collected at build time._
pub struct VersioningData {
    /// The name of the project.
    pub name: &'static str,
    /// The version of the project.
    pub version: &'static str,
    /// The full git commit hash.
    pub git_hash: &'static str,
    /// The short git commit hash.
    pub git_shash: &'static str,
}

impl Display for VersioningData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ansi_blue = "\x1b[34m";
        let ansi_green = "\x1b[32m";
        let ansi_magenta = "\x1b[35m";
        let ansi_dim = "\x1b[2m";
        let ansi_reset = "\x1b[0m";

        let b = |s: &str| format!("{}{}{}", ansi_blue, s, ansi_reset);
        let g = |s: &str| format!("{}{}{}", ansi_green, s, ansi_reset);
        let m = |s: &str| format!("{}{}{}", ansi_magenta, s, ansi_reset);
        let dim = |s: &str| format!("{}{}{}", ansi_dim, s, ansi_reset);

        write!(f, "{}@{}{} {} ", b(self.name), g(&dim("v")), g(self.version), dim("/"))?;
        write!(f, "{}{}", dim("ⵖ "), m(self.git_shash))?;

        Ok(())
    }
}
