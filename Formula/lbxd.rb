# Homebrew Formula for lbxd
#
# This formula is maintained in a separate repository:
# https://github.com/Pranav-Karra-3301/homebrew-lbxd
#
# This file serves as a template for the tap repository.
# To install via Homebrew:
#   brew tap pranav-karra-3301/lbxd
#   brew install lbxd
#
# v3.0.0 - Pure Rust implementation, NO Python dependencies!
#
# IMPORTANT: SHA256 hashes below are placeholders. Before releasing:
#   1. Build release artifacts with `cargo build --release`
#   2. Create the tar.gz archives for each platform
#   3. Calculate SHA256: `shasum -a 256 <archive>.tar.gz`
#   4. Replace SHA256_PLACEHOLDER_* values with actual hashes
class Lbxd < Formula
  desc "Beautiful command-line tool for Letterboxd - view activity, browse collections, and explore movies"
  homepage "https://github.com/Pranav-Karra-3301/lbxd"
  version "3.0.0"
  license "MIT"

  on_macos do
    on_intel do
      url "https://github.com/Pranav-Karra-3301/lbxd/releases/download/v3.0.0/lbxd-macos-x86_64.tar.gz"
      sha256 "SHA256_PLACEHOLDER_MACOS_X86"
    end
    on_arm do
      url "https://github.com/Pranav-Karra-3301/lbxd/releases/download/v3.0.0/lbxd-macos-aarch64.tar.gz"
      sha256 "SHA256_PLACEHOLDER_MACOS_ARM"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/Pranav-Karra-3301/lbxd/releases/download/v3.0.0/lbxd-linux-x86_64.tar.gz"
      sha256 "SHA256_PLACEHOLDER_LINUX_X86"
    end
    on_arm do
      url "https://github.com/Pranav-Karra-3301/lbxd/releases/download/v3.0.0/lbxd-linux-aarch64.tar.gz"
      sha256 "SHA256_PLACEHOLDER_LINUX_ARM"
    end
  end

  # Optional: viu for terminal image display
  depends_on "viu" => :recommended

  def install
    bin.install "lbxd"
  end

  test do
    output = shell_output("#{bin}/lbxd --version")
    assert_match "lbxd 3.0.0", output
  end

  def caveats
    <<~EOS
      lbxd v3.0.0 - Pure Rust implementation!

      What's new:
      - NO Python dependencies required
      - Faster performance with native Rust
      - Simplified installation

      For terminal image display, install viu:
        brew install viu

      Usage:
        lbxd browse username    # Interactive TUI mode
        lbxd recent username    # View recent activity
        lbxd movie "Inception"  # Search for movies
        lbxd --help             # See all commands

      For more information: https://github.com/Pranav-Karra-3301/lbxd
    EOS
  end
end
