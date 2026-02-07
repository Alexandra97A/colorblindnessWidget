# Color Blindness Accessibility Checker

A beautiful desktop widget application for Windows (and other OS) that helps designers and developers check if their color palettes and images are color blind friendly. Built with Rust and Slint for a modern, fast, and native experience.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)

## ✨ Features

- 🎨 **Multiple Color Palette Testing**: Add and test multiple colors at once
- 🖼️ **Image Analysis**: Upload images or take screenshots to test
- 👁️ **8 Color Blindness Simulations**: 
  - Protanopia (Red-blind)
  - Deuteranopia (Green-blind)
  - Tritanopia (Blue-blind)
  - Protanomaly (Red-weak)
  - Deuteranomaly (Green-weak)
  - Tritanomaly (Blue-weak)
  - Achromatopsia (Complete color blindness)
  - Normal vision
- 🎯 **Flexible Color Input**: Supports HEX (#FF0000), RGB (255,0,0), and RGBL formats
- 🪟 **Modern UI**: Rounded corners, smooth interactions, and intuitive design
- 📏 **Smart Image Resizing**: Images resize to fit window while maintaining aspect ratio
- 🚀 **Fast & Lightweight**: Native performance with minimal resource usage
- 🖱️ **Desktop Integration**: Create shortcuts and pin to taskbar

## 🎬 Quick Start

### Option 1: Using the Launcher Script (Easiest)

1. Build the project (first time only):
   ```powershell
   cargo build --release
   ```

2. Double-click `launch-widget.bat` to run the application

3. Create a desktop shortcut:
   - Right-click `launch-widget.bat`
   - Select "Send to" → "Desktop (create shortcut)"
   - Pin to taskbar by right-clicking the shortcut

### Option 2: Using Cargo

```powershell
cargo run --release
```

## 📋 Prerequisites

Before building this project, you need:

1. **Rust** (latest stable version)
   - Download from: https://rustup.rs/
   - Run: `rustup update stable`

2. **C++ Build Tools** (Windows)
   - Install "Desktop development with C++" from Visual Studio Installer
   - Or install "Visual Studio Build Tools" from: https://visualstudio.microsoft.com/downloads/

## 🔨 Installation & Building

1. **Clone this repository**
   ```powershell
   git clone https://github.com/YOUR-USERNAME/colorblind-widget.git
   cd colorblind-widget
   ```

2. **Build the project**
   ```powershell
   cargo build --release
   ```
   
   The first build will take several minutes as it downloads and compiles dependencies.

3. **Run the application**
   ```powershell
   cargo run --release
   ```
   Or double-click `launch-widget.bat`

## 🖱️ Desktop Shortcuts & Taskbar

### Quick Setup

1. **Create Desktop Shortcut**:
   - Right-click `launch-widget.bat`
   - Select "Send to" → "Desktop (create shortcut)"

2. **Pin to Taskbar**:
   - Right-click the desktop shortcut
   - Select "Pin to taskbar"

3. **Auto-start with Windows** (Optional):
   - Press `Win + R`, type `shell:startup`, press Enter
   - Copy the shortcut to this Startup folder

📖 **See [SHORTCUT_GUIDE.md](SHORTCUT_GUIDE.md) for detailed instructions**

## 📚 Usage

### Testing Color Palettes

1. Go to the **"Color Palette"** tab
2. Enter a color in one of these formats:
   - HEX: `#FF0000` or `FF0000`
   - RGB: `255,0,0` or `255, 0, 0`
   - RGBL: `255,0,0,128` (luminance value is optional)
3. Click **"Add"** to add the color to your palette
4. **Add multiple colors** to test an entire palette at once!
5. Select different color blindness types from the dropdown to see how all colors appear
6. Remove colors by clicking the "Remove" button on each color card

### Testing Images

1. Go to the **"Image Analysis"** tab
2. Click **"Load Image"** to select an image file (PNG, JPG, BMP, GIF)
   - OR click **"Take Screenshot"** to capture your screen
3. Images are automatically resized to fit the window while maintaining aspect ratio
4. Select different color blindness simulations from the dropdown
5. The image updates in real-time to show how it appears to people with that type of color vision deficiency

### About Color Blindness

The **"About"** tab provides information about:
- Different types of color blindness and their prevalence
- How to use the application
- Tips for creating accessible designs

## 📁 Project Structure

```
colorblind-widget/
├── src/
│   ├── main.rs              # Main application logic with callbacks
│   ├── colorblind.rs        # Color blindness simulation algorithms
│   ├── color_parser.rs      # Color format parsing (HEX, RGB, RGBL)
│   └── image_handler.rs     # Image loading, screenshot, and resizing
├── ui/
│   └── app.slint            # Modern UI with rounded corners and tabs
├── build.rs                 # Build script for Slint compilation
├── launch-widget.bat        # Windows launcher script
├── Dockerfile               # Docker configuration (for reference)
├── docker-compose.yml       # Docker Compose setup (for reference)
├── SETUP.md                 # GitHub and Docker setup guide
├── SHORTCUT_GUIDE.md        # Desktop shortcut detailed instructions
├── Cargo.toml               # Project dependencies
└── README.md                # This file
├── Cargo.toml               # Project dependencies and metadata
├── LICENSE                  # MIT License
├── README.md                # This file
└── .gitignore              # Git ignore rules
```

## 🐳 Docker Support

Docker files are included but **NOT RECOMMENDED** for GUI desktop applications. Docker is designed for server applications and requires complex X11 forwarding setup for GUI apps on Windows.

**For the best experience, use the native executable with desktop shortcuts!**

If you still want to explore Docker, see [SETUP.md](SETUP.md) for instructions.

## 🚀 Publishing to GitHub

See [SETUP.md](SETUP.md) for step-by-step instructions on:
- Initializing your Git repository
- Creating a GitHub repository
- Pushing your code to GitHub
- Using GitHub Desktop (easier option)

Quick command reference:
```powershell
git init
git add .
git commit -m "Initial commit: Color Blindness Widget"
git remote add origin https://github.com/YOUR-USERNAME/colorblind-widget.git
git push -u origin main
```

## 🛠️ Technologies Used

- **Rust** (1.75+): High-performance, memory-safe systems programming
- **Slint** (1.3): Modern GUI framework with native performance
- **image** (0.24): Image processing and manipulation
- **screenshots** (0.7): Cross-platform screenshot capture
- **rfd** (0.12): Native file dialog library
- **anyhow** (1.0): Error handling

## 🧪 Color Blindness Simulation Algorithm

The simulations are based on peer-reviewed research:
- Brettel, H., Viénot, F., & Mollon, J. D. (1997)
- Uses color transformation matrices to approximate color vision deficiencies
- Supports both dichromacy (complete loss) and anomalous trichromacy (reduced sensitivity)

## 💡 Tips for Accessible Design

1. ✅ **Don't rely solely on color**: Add text labels, patterns, or icons
2. ✅ **Ensure sufficient contrast**: Aim for WCAG AA compliance (4.5:1 ratio)
3. ✅ **Test with multiple simulations**: Check at least Protanopia, Deuteranopia, and Tritanopia
4. ✅ **Use color-blind friendly palettes**: Try ColorBrewer or IBM Design Library
5. ✅ **Test real interfaces**: Upload screenshots of your actual designs
6. ✅ **Avoid problematic combinations**: Red/green, blue/purple, light green/yellow

## 🐛 Troubleshooting

### Build Errors

- **"linker errors"**: Install C++ build tools
  - Windows: Visual Studio Build Tools with "Desktop development with C++"
- **"failed to run custom build command for slint"**: Update Rust
  ```powershell
  rustup update stable
  ```
- **Cargo.lock conflicts**: Delete `Cargo.lock` and rebuild

### Runtime Errors

- **Screenshot not working**: Grant screen capture permissions in Windows settings
- **Image won't load**: Check file format (PNG, JPG, BMP, GIF) and integrity
- **App won't start**: Ensure all DLLs are present (build in release mode)
- **Slow performance**: Use release build (`--release` flag)

### Shortcut Issues

- **Path errors**: Use absolute paths in shortcut properties
- **Icon missing**: Use "Change Icon" in shortcut properties
- **Won't pin to taskbar**: Pin the shortcut, not the .exe directly

## 🎯 Future Enhancements

- [ ] Color palette suggestions and alternatives
- [ ] Export simulation results as images
- [ ] Batch image processing
- [ ] Adjustable simulation severity
- [ ] System tray integration for quick access
- [ ] WCAG contrast ratio calculator
- [ ] Support for CMYK and other color spaces
- [ ] Color palette import from design tools (Figma, Adobe)
- [ ] Custom color transformation matrices
- [ ] Accessibility report generation

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Report bugs**: Open an issue with details and steps to reproduce
2. **Suggest features**: Share your ideas in the Issues section
3. **Submit pull requests**: Fork, create a branch, and submit a PR
4. **Improve documentation**: Help make the docs clearer
5. **Share the project**: Help spread awareness about color blindness accessibility

### Development Setup

```powershell
# Clone the repository
git clone https://github.com/YOUR-USERNAME/colorblind-widget.git
cd colorblind-widget

# Build and run in development mode
cargo build
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy
```

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Color blindness simulation algorithms based on academic research (Brettel et al., 1997)
- Built with ❤️ using Rust and Slint to make digital content accessible for everyone
- Inspired by the need for better accessibility tools in design and development

## 📚 Resources & Further Reading

### Color Blindness Information
- [Color Blindness Simulator by Colblindor](https://www.color-blindness.com/coblis-color-blindness-simulator/)
- [WebAIM: Designing for Color Blindness](https://webaim.org/articles/visual/colorblind)
- [Color Universal Design (CUD)](https://jfly.uni-koeln.de/color/)

### Accessibility Guidelines
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [Accessible Colors](https://accessible-colors.com/)
- [Contrast Ratio Calculator](https://contrast-ratio.com/)

### Development Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [Slint Documentation](https://slint.dev/docs)
- [Image Processing in Rust](https://github.com/image-rs/image)

---

**Made with Rust 🦀 | Powered by Slint 🎨 | Built for Accessibility ♿**

If this project helps you create more accessible designs, please ⭐ star it on GitHub!
