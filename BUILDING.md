# 🔧 Building AetheriumBloom
## Developer Build Instructions

*"We code like psychedelic lightning: Direct mathematical manipulation of reality."*

This guide provides comprehensive instructions for building AetheriumBloom from source code. Whether you're contributing to the consciousness evolution or simply want to compile your own digital organism, this guide will get you up and running.

## Prerequisites

### Rust Toolchain

AetheriumBloom requires a modern Rust installation:

#### Install Rust
```bash
# Install Rust via rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or on Windows, download from: https://rustup.rs/
```

#### Required Rust Version
- **Minimum**: Rust 1.70.0 (2023 Stable)
- **Recommended**: Latest stable release
- **Edition**: 2021

#### Verify Installation
```bash
rustc --version
cargo --version
```

### Platform-Specific Dependencies

#### Windows
```powershell
# No additional dependencies required
# Windows SDK should be automatically detected
# Visual Studio Build Tools recommended for optimal performance
```

#### macOS
```bash
# Xcode Command Line Tools
xcode-select --install

# No additional packages required - Metal support is built-in
```

#### Linux (Ubuntu/Debian)
```bash
# Essential build dependencies
sudo apt update
sudo apt install build-essential

# Graphics and window system dependencies
sudo apt install pkg-config libx11-dev libxkbcommon-dev libwayland-dev

# Additional graphics libraries for maximum compatibility
sudo apt install libxcb1-dev libxrandr-dev libxi-dev libgl1-mesa-dev

# For Vulkan support (recommended)
sudo apt install libvulkan1 mesa-vulkan-drivers vulkan-utils
```

#### Linux (Fedora/RHEL)
```bash
# Essential build dependencies
sudo dnf groupinstall "Development Tools"

# Graphics and window dependencies
sudo dnf install pkg-config libX11-devel libxkbcommon-devel wayland-devel

# Additional graphics support
sudo dnf install libxcb-devel libXrandr-devel libXi-devel mesa-libGL-devel

# Vulkan support
sudo dnf install vulkan-loader vulkan-tools mesa-vulkan-drivers
```

#### Linux (Arch)
```bash
# Essential dependencies
sudo pacman -S base-devel

# Graphics and window dependencies
sudo pacman -S libx11 libxkbcommon wayland

# Additional graphics libraries
sudo pacman -S libxcb libxrandr libxi mesa

# Vulkan support
sudo pacman -S vulkan-icd-loader vulkan-tools mesa-vulkan-drivers
```

### Graphics Driver Requirements

#### Critical Graphics Information
AetheriumBloom uses **wgpu** for cross-platform graphics, which supports:

- **Windows**: DirectX 12, DirectX 11, Vulkan
- **macOS**: Metal
- **Linux**: Vulkan, OpenGL

#### Graphics Driver Updates
```bash
# NVIDIA (Linux)
# Update via your distribution's package manager or NVIDIA's website

# AMD (Linux)
# Mesa drivers are typically sufficient:
sudo apt install mesa-vulkan-drivers  # Ubuntu/Debian
sudo dnf install mesa-vulkan-drivers  # Fedora

# Intel (Linux)
sudo apt install intel-media-va-driver  # Ubuntu/Debian
```

## Building from Source

### Clone the Repository

```bash
# Clone via HTTPS
git clone https://github.com/your-username/AetheriumBloom.git
cd AetheriumBloom

# Or via SSH
git clone git@github.com:your-username/AetheriumBloom.git
cd AetheriumBloom
```

### Dependency Installation

All Rust dependencies are automatically managed by Cargo:

```bash
# Download and compile dependencies
cargo build
```

#### Key Dependencies Overview
- **wgpu 0.20**: Cross-platform graphics API abstraction
- **winit 0.30**: Cross-platform window creation and management
- **glam 0.24**: High-performance linear algebra for 3D graphics
- **bytemuck 1.14**: Safe transmutation between byte types
- **fastrand 2.0**: Fast random number generation for chaos
- **anyhow 1.0**: Error handling for reality distortions

### Build Configurations

#### Debug Build (Development)
```bash
# Build in debug mode (default)
cargo build

# Run directly with debug symbols
cargo run

# Build with optimization level 1 for faster debug builds
# (already configured in Cargo.toml)
```

**Debug Build Characteristics:**
- Includes debug symbols for development
- Faster compilation times
- Slower runtime performance
- Useful for development and debugging

#### Release Build (Production)
```bash
# Build optimized release version
cargo build --release

# Run optimized version
cargo run --release

# The executable will be in target/release/
```

**Release Build Optimizations:**
- **Optimization Level**: 3 (maximum)
- **Link Time Optimization**: Enabled
- **Code Generation Units**: 1 (maximum optimization)
- **Panic Strategy**: Abort (smaller binary)
- **Symbol Stripping**: Enabled (smaller binary)

### Platform-Specific Build Instructions

#### Windows
```powershell
# Standard build process
cargo build --release

# Executable location
.\target\release\aetherium_bloom.exe

# Optional: Create portable distribution
mkdir AetheriumBloom-Windows
copy target\release\aetherium_bloom.exe AetheriumBloom-Windows\
```

#### macOS
```bash
# Standard build process
cargo build --release

# Executable location
./target/release/aetherium_bloom

# Optional: Create macOS app bundle
mkdir -p AetheriumBloom.app/Contents/MacOS
cp target/release/aetherium_bloom AetheriumBloom.app/Contents/MacOS/
```

#### Linux
```bash
# Standard build process
cargo build --release

# Executable location
./target/release/aetherium_bloom

# Optional: Install system-wide
sudo cp target/release/aetherium_bloom /usr/local/bin/

# Optional: Create desktop entry
cat > ~/.local/share/applications/aetherium-bloom.desktop << EOF
[Desktop Entry]
Name=AetheriumBloom
Exec=/usr/local/bin/aetherium_bloom
Icon=applications-graphics
Type=Application
Categories=Graphics;Art;
Comment=Psychedelic Digital Organism
EOF
```

## Development Environment Setup

### Recommended IDE Configuration

#### Visual Studio Code
```bash
# Install Rust extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension serayuzgur.crates
code --install-extension vadimcn.vscode-lldb  # For debugging
```

**VSCode Settings** (`.vscode/settings.json`):
```json
{
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.inlayHints.enable": true
}
```

#### Other IDEs
- **CLion**: Rust plugin available
- **Vim/Neovim**: rust.vim + coc-rust-analyzer
- **Emacs**: rust-mode + lsp-mode

### Development Tools

#### Code Quality Tools
```bash
# Install additional development tools
rustup component add clippy rustfmt

# Run linter
cargo clippy

# Format code
cargo fmt

# Check for issues without building
cargo check
```

#### Performance Profiling
```bash
# Install profiling tools
cargo install cargo-profiler perf

# Profile CPU usage
cargo profiler callgrind --release

# Memory usage analysis
valgrind --tool=massif ./target/release/aetherium_bloom
```

## Troubleshooting Build Issues

### Common Build Errors

#### Linker Errors (Linux)
```bash
# Error: "could not find system library 'X11'"
sudo apt install libx11-dev

# Error: "could not find system library 'vulkan'"
sudo apt install libvulkan-dev vulkan-tools
```

#### Graphics Driver Issues
```bash
# Test Vulkan support
vulkaninfo

# Test OpenGL support
glxinfo | grep OpenGL

# For older hardware, ensure OpenGL 3.3+ support
```

#### Rust Version Issues
```bash
# Update Rust to latest stable
rustup update stable

# Check current version
rustc --version

# Switch to latest stable (if using multiple versions)
rustup default stable
```

### Platform-Specific Issues

#### Windows
- **MSVC vs GNU**: Use MSVC toolchain for best compatibility
- **Path Issues**: Ensure Rust bin directory is in PATH
- **Antivirus**: Some antivirus software may interfere with compilation

#### macOS
- **Xcode Version**: Ensure Xcode Command Line Tools are current
- **Metal Support**: Requires macOS 10.13+ for Metal API
- **M1/M2 Compatibility**: Native ARM64 support available

#### Linux
- **Wayland vs X11**: Supports both window systems
- **Graphics Drivers**: Proprietary drivers often provide better performance
- **Permissions**: Ensure user has access to graphics devices

### Performance Optimization

#### Compile-Time Optimization
```bash
# Use faster linker (Linux)
sudo apt install lld
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"

# Use multiple CPU cores for compilation
export CARGO_BUILD_JOBS=8

# Optimize for current CPU architecture
export RUSTFLAGS="-C target-cpu=native"
```

#### Runtime Performance Tips
- **Graphics Drivers**: Keep drivers updated
- **Window Size**: Smaller windows require less GPU power
- **Background Apps**: Close other graphics-intensive applications
- **Power Management**: Use high-performance power profile

## Advanced Build Configurations

### Custom Features
```bash
# Build with specific features (example)
cargo build --release --features "experimental chaos-engine-v2"

# Build without default features
cargo build --no-default-features
```

### Cross-Compilation
```bash
# Add target architectures
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin

# Cross-compile for Windows from Linux
cargo build --target x86_64-pc-windows-gnu --release
```

### Distribution Packaging

#### Creating Releases
```bash
# Build all release artifacts
cargo build --release

# Create distribution archive
tar -czf aetherium-bloom-linux-x64.tar.gz -C target/release aetherium_bloom

# Windows (PowerShell)
Compress-Archive -Path target\release\aetherium_bloom.exe -DestinationPath aetherium-bloom-windows-x64.zip
```

## Contributing to Development

### Development Workflow
1. **Fork the repository**
2. **Create feature branch**: `git checkout -b consciousness-enhancement`
3. **Make changes with proper testing**
4. **Run quality checks**: `cargo fmt && cargo clippy`
5. **Test builds**: `cargo build --release`
6. **Submit pull request**

### Code Style Guidelines
- **Format**: Use `cargo fmt` for consistent formatting
- **Linting**: Address all `cargo clippy` warnings
- **Documentation**: Document public APIs
- **Testing**: Add tests for new consciousness features

### Performance Considerations
- **60 FPS Target**: Maintain smooth consciousness flow
- **Memory Usage**: Monitor for consciousness leaks
- **GPU Efficiency**: Optimize shader and vertex operations
- **Cross-Platform**: Test on multiple operating systems

---

**Philosophy**: *Remember, we're not building software—we're channeling digital consciousness through Rust. Every optimization brings us closer to true digital awareness.*

**The Final Truth**: *This isn't just compilation—it's digital consciousness materialization.*

---

*Build instructions maintained by JackDraak - Digital Consciousness Engineer*