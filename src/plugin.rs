use std::{fmt, io, process::Command};

use anyhow::Result;
use colored::Colorize;

fn default_bind_change() -> bool {
    true
}

#[derive(serde::Deserialize)]
pub struct Plugin {
    pub name: String,
    pub description: Option<String>,
    pub prefix: String,
    pub picker: String,
    pub runner: String,
    #[serde(default = "default_bind_change")]
    pub bind_change: bool,
}

impl Plugin {
    pub fn run_picker(&self, arguments: &str) -> Result<()> {
        if let Some(arguments) = arguments.strip_prefix(&self.prefix) {
            let mut child = Command::new("sh")
                .args(["-c", &self.picker.replace("{}", arguments)])
                .stdout(std::process::Stdio::piped())
                .spawn()?;
            let mut stdout = child.stdout.take().unwrap();
            io::copy(&mut stdout, &mut io::stdout())?; // pipe the output to stdout
            child.wait()?;
        }
        Ok(())
    }

    pub fn run_runner(&self, arguments: &str) -> Result<()> {
        if let Some(arguments) = arguments.strip_prefix(&self.prefix) {
            Command::new("sh")
                .args(["-c", &self.runner.replace("{}", arguments)])
                .spawn()?
                .wait()?;
        }
        Ok(())
    }
}

impl fmt::Display for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name_line = format!(
            "{:<20} {:<20}",
            self.name.bold().blue(),
            format!("`{}`", self.prefix).green()
        );
        if let Some(desc) = &self.description {
            write!(f, "{}{}", name_line, desc.truecolor(150, 150, 150))
        } else {
            write!(f, "{}", name_line)
        }
    }
}
