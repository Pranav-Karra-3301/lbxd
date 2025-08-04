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
# This formula builds lbxd from source with all dependencies
class Lbxd < Formula
  desc "Terminal-based Letterboxd client for movie enthusiasts"
  homepage "https://github.com/Pranav-Karra-3301/lbxd"
  url "https://github.com/Pranav-Karra-3301/lbxd/archive/v2.1.1.tar.gz"
  sha256 "SHA256_PLACEHOLDER"
  license "MIT"
  head "https://github.com/Pranav-Karra-3301/lbxd.git", branch: "main"

  depends_on "rust" => :build
  depends_on "python@3.12"

  def install
    # Build lbxd from source
    system "cargo", "install", *std_cargo_args
    
    # Install Python dependencies using Homebrew's Python
    python3 = Formula["python@3.12"].opt_bin/"python3"
    system python3, "-m", "pip", "install", "--break-system-packages", "letterboxdpy"
    
    # Install viu for enhanced image display (optional but recommended)
    system "cargo", "install", "viu" rescue nil
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