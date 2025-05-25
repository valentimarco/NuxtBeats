use crate::error::{Error, Result};
use std::{path::PathBuf, process::Stdio, time::Duration};
use tokio::{process::Command, time::timeout};

#[derive(Debug, Clone)]
pub struct YtHandler {
    path: PathBuf,
    timeout: Duration,
}

pub struct OutputHandler {
    pub stdout: Vec<u8>,
    pub stderr: String,
}

impl YtHandler {
    pub fn new(path: PathBuf, timeout: Duration) -> YtHandler {
        YtHandler { path, timeout }
    }

    // Code taken from ytdlp rust bind
    pub async fn execute(&self, url: String) -> Result<OutputHandler> {
        let mut cmd = Command::new(&self.path)
            .arg("--cache-dir")
            .arg("ytdlp")
            .arg("-f")
            .arg("bestaudio/best")
            .arg("-x")
            .arg("-q")
            .arg("-o") // set output
            .arg("-") // stdout
            .arg(url)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout_handler = cmd
            .stdout
            .take()
            .ok_or_else(|| Error::Command(String::from("Failed to capture stdout")))?;
        let stderr_handler = cmd
            .stderr
            .take()
            .ok_or_else(|| Error::Command(String::from("Failed to capture stderr")))?;

        let stdout_task = tokio::spawn(async move {
            let mut buffer = Vec::new();
            tokio::io::copy(&mut tokio::io::BufReader::new(stdout_handler), &mut buffer).await?;
            Ok::<Vec<u8>, std::io::Error>(buffer)
        });

        let stderr_task = tokio::spawn(async move {
            let mut buffer = Vec::new();
            tokio::io::copy(&mut tokio::io::BufReader::new(stderr_handler), &mut buffer).await?;
            Ok::<Vec<u8>, std::io::Error>(buffer)
        });

        // Wait for the process to finish with timeout
        let exit_code = match timeout(self.timeout, cmd.wait()).await {
            Ok(res) => res?,
            Err(_) => {
                if let Err(e) = cmd.kill().await {
                    #[cfg(feature = "tracing")]
                    tracing::error!("Failed to kill the process after timeout: {}", e)
                }
                return Err(Error::Timeout(self.timeout));
            }
        };

        let stdout = match stdout_task.await {
            Ok(Ok(buf)) => buf,
            Ok(Err(e)) => return Err(Error::from(e)),
            Err(e) => return Err(Error::from(e)),
        };

        let stderr = match stderr_task.await {
            Ok(Ok(buf)) => String::from_utf8(buf).unwrap(),
            Ok(Err(e)) => return Err(Error::from(e)),
            Err(e) => return Err(Error::from(e)),
        };

        if exit_code.success() {
            return Ok(OutputHandler { stdout, stderr });
        }

        return Err(Error::Command(format!("Process failed: {}", stderr)));
    }
}
