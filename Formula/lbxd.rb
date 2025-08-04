# Formula for Homebrew
class Lbxd < Formula
  desc "Beautiful command-line tool for Letterboxd with interactive TUI"
  homepage "https://github.com/Pranav-Karra-3301/lbxd"
  url "https://github.com/Pranav-Karra-3301/lbxd/archive/v2.1.1.tar.gz"
  sha256 "SHA256_PLACEHOLDER"
  license "MIT"
  head "https://github.com/Pranav-Karra-3301/lbxd.git"

  depends_on "rust" => :build
  depends_on "python@3.12"

  def install
    # Install Python dependencies
    system "pip3", "install", "--user", "letterboxdpy"
    
    # Build and install lbxd
    system "cargo", "install", *std_cargo_args
    
    # Try to install viu for enhanced image display
    begin
      system "cargo", "install", "viu"
    rescue
      puts "Warning: Failed to install viu. lbxd will use ASCII art mode by default."
      puts "You can install viu later with: cargo install viu"
    end
  end

  def post_install
    puts ""
    puts "🎬 lbxd installation complete!"
    puts ""
    puts "Quick start:"
    puts "  • lbxd --help                  - Show all commands"
    puts "  • lbxd recent username         - View recent activity"
    puts "  • lbxd movie \"Inception\"       - Search for movies"
    puts "  • lbxd browse username         - Interactive TUI mode"
    puts ""
    puts "📖 Documentation: https://github.com/Pranav-Karra-3301/lbxd/tree/main/docs"
  end

  test do
    system "#{bin}/lbxd", "--version"
    system "#{bin}/lbxd", "--help"
  end
end