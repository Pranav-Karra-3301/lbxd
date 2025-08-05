use anyhow::Result;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

pub struct ViuViewer {
    client: reqwest::Client,
}

impl Default for ViuViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl ViuViewer {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self { client }
    }

    /// Check if viu is available on the system
    pub fn is_available() -> bool {
        // Check for viu command
        if let Ok(output) = Command::new("viu").arg("--help").output() {
            if output.status.success() {
                return true;
            }
        }

        false
    }

    /// Display an image using viu with optimal settings
    pub async fn display_image_url(
        &self,
        image_url: &str,
        width: u32,
        use_pixelated_mode: bool,
    ) -> Result<()> {
        // Download the image to a temporary file
        let image_data = self.fetch_image(image_url).await?;

        // Create temporary file
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(&image_data)?;
        let temp_path = temp_file.path();

        // Build viu command based on pixelated mode preference
        let mut cmd = Command::new("viu");
        cmd.arg("--width").arg(width.to_string()).arg(temp_path);

        if use_pixelated_mode {
            // Try -b first, then --blocks for compatibility
            if let Ok(mut child) = cmd.arg("-b").spawn() {
                if let Ok(status) = child.wait() {
                    if status.success() {
                        return Ok(());
                    }
                }
            }

            // Fallback to --blocks if -b didn't work
            let mut cmd_fallback = Command::new("viu");
            cmd_fallback
                .arg("--blocks")
                .arg("--width")
                .arg(width.to_string())
                .arg(temp_path);
            if let Ok(mut child) = cmd_fallback.spawn() {
                if let Ok(status) = child.wait() {
                    if status.success() {
                        return Ok(());
                    }
                }
            }
        } else {
            // Full resolution mode - no blocks flag
            if let Ok(mut child) = cmd.spawn() {
                if let Ok(status) = child.wait() {
                    if status.success() {
                        return Ok(());
                    }
                }
            }
        }

        Err(anyhow::anyhow!("Failed to display image with viu"))
    }

    /// Display an image from local file path using viu
    pub fn display_image_file(&self, file_path: &str, width: u32) -> Result<()> {
        // Try -b first, then --blocks for compatibility
        if let Ok(mut child) = Command::new("viu")
            .arg("-b") // Force block output for better quality
            .arg("--width")
            .arg(width.to_string()) // Use specified width
            .arg(file_path)
            .spawn()
        {
            if let Ok(status) = child.wait() {
                if status.success() {
                    return Ok(());
                }
            }
        }

        // Fallback to --blocks if -b didn't work
        if let Ok(mut child) = Command::new("viu")
            .arg("--blocks") // Alternative blocks flag for compatibility
            .arg("--width")
            .arg(width.to_string()) // Use specified width
            .arg(file_path)
            .spawn()
        {
            if let Ok(status) = child.wait() {
                if status.success() {
                    return Ok(());
                }
            }
        }

        Err(anyhow::anyhow!("Failed to display image with viu"))
    }

    /// Get installation instructions for viu
    pub fn get_installation_instructions() -> String {
        r#"
viu (Rust terminal image viewer) not found. To install:

macOS:
  brew install viu

Linux (Arch):
  pacman -S viu

Ubuntu/Debian:
  cargo install viu

Rust (any platform):
  cargo install viu

Alternative: Use --ascii flag for ASCII art display
"#
        .to_string()
    }

    async fn fetch_image(&self, url: &str) -> Result<Vec<u8>> {
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch image: HTTP {}",
                response.status()
            ));
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}
