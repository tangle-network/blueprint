use super::CommandInstalled;
use color_eyre::eyre::Result;
use dialoguer::console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    sync::mpsc,
    thread,
};

pub struct Forge {
    /// Default template to use if none specified
    pub default_template: String,
    /// Default commit to use if none specified
    pub default_commit: String,
}

impl Default for Forge {
    fn default() -> Self {
        Self::new()
    }
}

#[expect(clippy::unused_self)]
impl Forge {
    /// Creates a new Forge instance with default settings.
    pub fn new() -> Self {
        Self {
            default_template: "foundry-rs/forge-template".to_string(),
            default_commit: "main".to_string(),
        }
    }

    /// Returns the version of Forge.
    pub fn version(&self) -> Result<String> {
        let output = Command::new("forge").arg("--version").output()?;
        Ok(String::from_utf8(output.stdout)?
            .trim()
            .replace("forge", ""))
    }

    /// Returns true if Forge is installed.
    pub fn is_installed(&self) -> bool {
        Command::new("forge").is_installed()
    }

    /// Install dependencies with live progress updates.
    /// Shows real-time output from forge and tracks progress.
    pub fn install_dependencies(&self) -> Result<()> {
        // Start the command with inherited stdio to show output directly
        let status = Command::new("forge")
            .args(["soldeer", "update", "-d"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Err(color_eyre::eyre::eyre!(
                "Failed to install dependencies. Check the output above for details."
            ));
        }

        Ok(())
    }

    /// Build the contracts with progress tracking.
    pub fn build(&self) -> Result<()> {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
                .template("{spinner:.green} {msg}")?,
        );

        spinner.set_message("Building contracts...");

        let mut child = Command::new("forge")
            .arg("build")
            .arg("--via-ir")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr = child.stderr.take().expect("Failed to capture stderr");

        // Create readers for both stdout and stderr
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        // Create channels for output communication
        let (tx, rx) = mpsc::channel();
        let tx_stderr = tx.clone();

        // Keep a buffer of recent output lines
        let mut output_buffer = VecDeque::with_capacity(100);

        // Spawn thread to read stdout
        thread::spawn(move || {
            for line in stdout_reader.lines().map_while(Result::ok) {
                let _ = tx.send(line);
            }
        });

        // Spawn thread to read stderr
        thread::spawn(move || {
            for line in stderr_reader.lines().map_while(Result::ok) {
                let _ = tx_stderr.send(line);
            }
        });

        // Update spinner and show output while command runs
        while child.try_wait()?.is_none() {
            spinner.tick();

            // Check for new output
            for line in rx.try_iter() {
                if let Some(new) = push_output_line(&mut output_buffer, line) {
                    spinner.suspend(|| {
                        println!("{}", style(&new).dim());
                    });
                }
            }

            thread::sleep(std::time::Duration::from_millis(100));
        }

        let status = child.wait()?;

        // `try_wait` reaps the child as soon as it exits, which means the loop
        // above can stop before the reader threads have forwarded everything the
        // pipes held. Forge writes its compiler diagnostics at the very end, so
        // dropping this tail loses precisely the lines that explain the failure.
        // The channel closes once both readers see EOF, which the exited child
        // guarantees, so a blocking drain here cannot hang. These lines are not
        // echoed: the error below is what the caller reports on.
        for line in rx.iter() {
            push_output_line(&mut output_buffer, line);
        }

        if !status.success() {
            spinner.finish_with_message("❌ Failed to build contracts");
            let mut message = String::from("Failed to build contracts.");
            if output_buffer.is_empty() {
                message.push_str(" `forge build` produced no output.");
            } else {
                message.push_str("\n\n");
                message.push_str(&output_buffer.iter().fold(String::new(), |mut acc, line| {
                    acc.push_str(line);
                    acc.push('\n');
                    acc
                }));
            }
            return Err(color_eyre::eyre::eyre!(message));
        }

        spinner.finish_with_message("✨ Contracts built successfully!");
        Ok(())
    }
}

/// Record one line of `forge` output, keeping the buffer bounded and duplicate
/// free. Returns the line only when it was newly recorded, so the caller can
/// echo exactly what the buffer kept.
fn push_output_line(output_buffer: &mut VecDeque<String>, line: String) -> Option<String> {
    // Only add non-duplicate lines
    if output_buffer.contains(&line) {
        return None;
    }

    output_buffer.push_back(line.clone());

    // Keep only the last 100 lines
    if output_buffer.len() > 100 {
        output_buffer.pop_front();
    }

    Some(line)
}

#[cfg(test)]
mod tests {
    use super::push_output_line;

    fn drain(lines: &[&str]) -> Vec<String> {
        let mut buffer = std::collections::VecDeque::new();
        lines
            .iter()
            .filter_map(|line| push_output_line(&mut buffer, (*line).to_owned()))
            .collect()
    }

    #[test]
    fn records_each_line_once() {
        assert_eq!(drain(&["a", "b"]), vec!["a", "b"]);
    }

    #[test]
    fn suppresses_repeated_lines() {
        assert_eq!(drain(&["a", "b", "a"]), vec!["a", "b"]);
    }

    #[test]
    fn keeps_the_last_hundred_lines() {
        let lines: Vec<String> = (0..150).map(|i| i.to_string()).collect();
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        let mut buffer = std::collections::VecDeque::new();
        for line in &refs {
            push_output_line(&mut buffer, (*line).to_owned());
        }

        assert_eq!(buffer.len(), 100);
        // The earliest lines are the ones dropped; forge's diagnostics are last.
        assert_eq!(buffer.front().map(String::as_str), Some("50"));
        assert_eq!(buffer.back().map(String::as_str), Some("149"));
    }
}
