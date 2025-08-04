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
  desc "Beautiful command-line tool for Letterboxd - view activity, browse collections, and explore movies"
  homepage "https://github.com/Pranav-Karra-3301/lbxd"
  url "https://github.com/Pranav-Karra-3301/lbxd/archive/refs/tags/v2.1.1.tar.gz"
  sha256 "032dfc3f1a4da5be92ca93c1fafee838be1d5989899f00eaa22e21f3ed7cfc3d"
  license "MIT"

  depends_on "rust" => :build
  depends_on "python@3.12"
  depends_on "curl"

  def install
    # Install Python dependencies
    system Formula["python@3.12"].opt_bin/"pip3", "install", "letterboxdpy"

    # Build Rust project
    system "cargo", "install", *std_cargo_args

    # Ensure binary is installed correctly
    bin.install "target/release/lbxd" if File.exist?("target/release/lbxd")
  end

  def post_install
    # Verify Python dependencies are available
    python_cmd = Formula["python@3.12"].opt_bin/"python3"
    system python_cmd, "-c", "import letterboxdpy"
  end

  test do
    # Test that the binary runs and shows version
    output = shell_output("#{bin}/lbxd --version")
    assert_match "lbxd 2.1.1", output

    # Test that Python dependencies are accessible
    python_cmd = Formula["python@3.12"].opt_bin/"python3"
    system python_cmd, "-c", "import letterboxdpy"
  end

  def caveats
    <<~EOS
      lbxd requires Python 3 with the letterboxdpy package.
      
      Dependencies installed:
      - Python 3.12
      - letterboxdpy (Python package)
      - curl (for network requests)
      
      Usage:
        # Show version and help
        lbxd
        
        # Browse a user's collection interactively
        lbxd browse username
        
        # Show recent activity
        lbxd recent username
      
      For more information, visit: https://github.com/Pranav-Karra-3301/lbxd
    EOS
  end
end