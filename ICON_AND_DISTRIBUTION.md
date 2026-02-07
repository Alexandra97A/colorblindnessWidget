# How to Add a Custom Icon and Create Desktop Shortcuts

This guide will show you how to add a custom icon to your widget and distribute it to others.

---

## Part 1: Creating a Custom Icon

### Option A: Create Your Own Icon

1. **Design your icon** (256x256 pixels recommended)
   - Use tools like:
     - **GIMP** (free): https://www.gimp.org/
     - **Paint.NET** (free, Windows): https://www.getpaint.net/
     - **Photoshop** (paid)
     - **Canva** (free, online): https://www.canva.com/

2. **Convert to .ico format**
   - Online tools:
     - https://convertio.co/png-ico/
     - https://www.icoconverter.com/
     - https://image.online-convert.com/convert-to-ico
   
   - Desktop tools:
     - **GIMP**: File → Export As → Select .ico format
     - **IcoFX** (Windows, free for personal use)

3. **Include multiple sizes** for best results:
   - 16x16, 32x32, 48x48, 64x64, 128x128, 256x256

### Option B: Find Free Icons

**Free Icon Resources:**
- **Flaticon**: https://www.flaticon.com/ (requires attribution)
- **Icons8**: https://icons8.com/ (free with link)
- **The Noun Project**: https://thenounproject.com/
- **IconArchive**: https://www.iconarchive.com/

**Search terms to try:**
- "eye icon"
- "accessibility icon"
- "color palette icon"
- "vision icon"
- "color blindness icon"

---

## Part 2: Add Icon to Your Application

### Step 1: Save Your Icon File

Save your icon as `icon.ico` in your project root:
```
Widget_colorBlindness/
├── icon.ico          ← Save here
├── src/
├── ui/
└── ...
```

### Step 2: Update Cargo.toml

Add the `embed-resource` dependency to `Cargo.toml`:

```toml
[build-dependencies]
slint-build = "1.3"
embed-resource = "2.4"  # Add this line
```

### Step 3: Update build.rs

Replace your `build.rs` content with:

```rust
fn main() {
    slint_build::compile("ui/app.slint").unwrap();
    
    // Embed icon on Windows
    #[cfg(windows)]
    {
        embed_resource::compile("icon.rc", embed_resource::NONE);
    }
}
```

### Step 4: Create icon.rc File

Create a new file `icon.rc` in your project root:

```rc
// icon.rc
1 ICON "icon.ico"
```

### Step 5: Rebuild

```powershell
cargo clean
cargo build --release
```

Now your `.exe` will have the embedded icon! 🎨

---

## Part 3: Easy Installation for Others

I've created an installer script that automatically:
- ✅ Creates desktop shortcut with icon
- ✅ Pins to taskbar (user confirms)
- ✅ Adds to Start Menu
- ✅ Optionally adds to Startup

**Users just need to:**
1. Download your release
2. Run `install.bat`
3. Done!

See `install.bat` in your project folder.

---

## Part 4: Distribute to Others

### Method 1: GitHub Releases (Recommended)

1. **Build release version:**
   ```powershell
   cargo build --release
   ```

2. **Create a release package** with these files:
   ```
   colorblind-widget-v0.1.0-windows/
   ├── colorblind-widget.exe    ← From target/release/
   ├── install.bat              ← Installation script
   ├── uninstall.bat           ← Uninstallation script
   └── README.txt              ← Quick instructions
   ```

3. **Zip the folder**:
   ```powershell
   Compress-Archive -Path "colorblind-widget-v0.1.0-windows" -DestinationPath "colorblind-widget-v0.1.0-windows.zip"
   ```

4. **Upload to GitHub Releases**:
   - Go to your repository on GitHub
   - Click "Releases" → "Create a new release"
   - Tag: `v0.1.0`
   - Title: "Color Blindness Widget v0.1.0"
   - Upload the `.zip` file
   - Add release notes

### Method 2: Direct Sharing

Share the `.zip` file via:
- Email
- Cloud storage (Google Drive, Dropbox, OneDrive)
- USB drive
- Company network share

---

## Part 5: Installation Instructions for Users

Create a `README.txt` in your release package:

```
Color Blindness Accessibility Checker v0.1.0
============================================

Installation:
1. Extract this ZIP file to any location
2. Double-click "install.bat"
3. Follow the prompts
4. Done! Find the shortcut on your desktop or taskbar

Manual Installation:
1. Extract this ZIP file
2. Right-click "colorblind-widget.exe"
3. Select "Create shortcut"
4. Move shortcut to desktop
5. Right-click shortcut → "Pin to taskbar"

Uninstallation:
- Run "uninstall.bat"
- Or manually delete shortcuts

Requirements:
- Windows 10 or later
- No additional software needed!

For more info: https://github.com/YOUR-USERNAME/colorblind-widget
```

---

## Part 6: Testing Your Icon

### Test Embedded Icon:

```powershell
# Build with icon
cargo build --release

# Check the .exe properties
# Right-click target/release/colorblind-widget.exe → Properties
# You should see your custom icon!
```

### Test Shortcuts:

```powershell
# Run the installer
.\install.bat

# Check:
# - Desktop shortcut shows correct icon
# - Start Menu entry shows correct icon
# - Taskbar (if pinned) shows correct icon
```

---

## Part 7: Advanced - Create Installer (Optional)

For a more professional distribution, create a proper installer:

### Option A: Inno Setup (Free, Easy)

1. Download: https://jrsoftware.org/isinfo.php
2. Create installer script (`.iss` file)
3. Build installer `.exe`
4. Users run single installer file

### Option B: WiX Toolset (Free, Professional)

1. Download: https://wixtoolset.org/
2. Create MSI installer
3. More professional, Windows-native

### Option C: cargo-wix (Rust-native)

```powershell
cargo install cargo-wix
cargo wix init
cargo wix
```

---

## Troubleshooting

### Icon doesn't show in .exe
- Check `icon.ico` exists in project root
- Check `icon.rc` syntax is correct
- Run `cargo clean` then rebuild
- Verify embed-resource is in Cargo.toml

### Shortcut icon is wrong
- The .exe needs to be built with embedded icon first
- Update the shortcut after rebuilding
- Delete old shortcut and create new one

### Icon looks pixelated
- Include multiple icon sizes (16x16 up to 256x256)
- Use .ico format, not .png
- Save with proper bit depth (32-bit recommended)

---

## Example Icon Ideas

For a color blindness checker, consider:
- 👁️ Eye with color spectrum
- 🎨 Palette with accessibility symbol
- 🌈 Rainbow with checkmark
- 👓 Glasses icon
- 🔍 Magnifying glass over colors
- ♿ Accessibility symbol with colors

---

## File Size Considerations

- Icon file: ~50-200 KB
- .exe without icon: ~8-15 MB
- .exe with icon: ~8-15 MB (minimal increase)
- Release ZIP: ~3-5 MB (with compression)

---

## Quick Command Reference

```powershell
# Build with icon
cargo clean
cargo build --release

# Create release package
mkdir release-package
copy target\release\colorblind-widget.exe release-package\
copy install.bat release-package\
copy uninstall.bat release-package\
copy README.txt release-package\

# Zip it
Compress-Archive -Path release-package -DestinationPath colorblind-widget-v0.1.0.zip

# Test
cd release-package
.\install.bat
```

---

## Next Steps

1. ✅ Create or find an icon
2. ✅ Add it to your project
3. ✅ Rebuild with embedded icon
4. ✅ Test the installer script
5. ✅ Create release package
6. ✅ Upload to GitHub Releases
7. ✅ Share with others!

**Your widget is now ready for easy distribution! 🚀**
