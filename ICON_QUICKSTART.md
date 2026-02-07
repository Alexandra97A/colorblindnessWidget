# Quick Icon Setup Guide

Since you don't have an `icon.ico` file yet, here's how to get one quickly:

---

## Option 1: Download a Free Icon (5 minutes)

### Step-by-Step:

1. **Go to Icons8** (no signup required):
   - Visit: https://icons8.com/icons/set/eye
   - Search for: "eye", "vision", "accessibility", or "color palette"

2. **Select an icon you like**:
   - Click on the icon
   - Choose size: **256px** (or largest available)
   - Choose format: **ICO** (Windows Icon)
   - Download for free (with attribution)

3. **Save the icon**:
   - Rename it to: `icon.ico`
   - Save it in your project root folder (same folder as Cargo.toml)

4. **Build with icon**:
   ```powershell
   cargo clean
   cargo build --release
   ```

5. **Done!** Your .exe now has a custom icon!

---

## Option 2: Convert an Existing Image (10 minutes)

If you have a PNG or JPG image:

1. **Go to online converter**:
   - Visit: https://convertio.co/png-ico/
   - Or: https://www.icoconverter.com/

2. **Upload your image**:
   - Best results with square images (256x256 or larger)
   - Simple designs work better than complex ones

3. **Convert to ICO**:
   - Select multiple sizes (recommended)
   - Download the .ico file

4. **Save as `icon.ico`** in project root

5. **Build**:
   ```powershell
   cargo clean
   cargo build --release
   ```

---

## Option 3: Create Your Own Icon (30 minutes)

### Using GIMP (Free):

1. **Download GIMP**: https://www.gimp.org/downloads/

2. **Create new image**:
   - File → New
   - Size: 256x256 pixels
   - Fill: Transparent

3. **Design your icon**:
   - Use simple shapes and colors
   - Keep it recognizable when small
   - Ideas: eye symbol, palette, accessibility symbol

4. **Export as ICO**:
   - File → Export As
   - Filename: `icon.ico`
   - Format: Microsoft Windows Icon (*.ico)
   - Select multiple sizes: 16, 32, 48, 256

5. **Save to project root**

6. **Build**:
   ```powershell
   cargo clean
   cargo build --release
   ```

---

## Suggested Icon Themes

For a color blindness checker, consider:

1. **Eye Icon** 👁️
   - Simple eye with color rings
   - Eye with spectrum colors
   - Stylized eye symbol

2. **Accessibility Icon** ♿
   - Universal accessibility symbol
   - Combined with color palette

3. **Color Palette** 🎨
   - Painter's palette with colors
   - Color swatches
   - Rainbow/spectrum icon

4. **Combination Icons**
   - Eye + color palette
   - Checkmark + colors
   - Magnifying glass + palette

---

## Testing Your Icon

After building with the icon:

1. **Check the .exe**:
   ```powershell
   # Navigate to the executable
   cd target\release
   
   # Right-click colorblind-widget.exe
   # Select Properties
   # You should see your icon in the dialog!
   ```

2. **Test shortcuts**:
   ```powershell
   # Create a shortcut
   # The shortcut should automatically use your icon
   ```

3. **Test in different views**:
   - Desktop shortcut (large icon view)
   - File Explorer (various sizes)
   - Taskbar (when pinned)

---

## Temporary Solution

If you want to test everything WITHOUT an icon first:

1. **Comment out the icon code** in `build.rs`:
   ```rust
   fn main() {
       slint_build::compile("ui/app.slint").unwrap();
       
       // Embed icon on Windows builds
       // #[cfg(windows)]
       // {
       //     embed_resource::compile("icon.rc", embed_resource::NONE);
       // }
   }
   ```

2. **Build**:
   ```powershell
   cargo build --release
   ```

3. **Add icon later** when you're ready

---

## Icon Attribution

If using free icons that require attribution:

1. **Add to README**:
   ```markdown
   ## Credits
   - Icon by [Author Name] from [Icon8s/Flaticon/etc]
   - Licensed under [License Type]
   ```

2. **Add to application**:
   - Include in the "About" tab
   - Or in a separate credits file

---

## Quick Command Reference

```powershell
# After adding icon.ico to project root:

# Clean previous builds
cargo clean

# Build with icon
cargo build --release

# Check the .exe
cd target\release
explorer .

# Right-click colorblind-widget.exe → Properties
# You should see your icon!

# Create release package
cd ..\..
.\create-release.ps1
```

---

## What If I Don't Want a Custom Icon?

That's okay! The application will work fine without one. Windows will:
- Use a default application icon
- Still be fully functional
- Still create shortcuts normally

You can always add an icon later!

---

## File Checklist

Before building with icon:
- [ ] `icon.ico` exists in project root
- [ ] `icon.rc` exists in project root
- [ ] `embed-resource` added to Cargo.toml
- [ ] build.rs updated with embed code
- [ ] Run `cargo clean`
- [ ] Run `cargo build --release`
- [ ] Check icon in .exe properties

---

## Ready to Proceed?

**With Icon:**
1. Get icon (download or create)
2. Save as `icon.ico` in project root
3. `cargo clean && cargo build --release`
4. `.\create-release.ps1` to package

**Without Icon:**
1. Comment out icon code in build.rs
2. `cargo build --release`
3. `.\create-release.ps1` to package
4. Add icon later when ready

Either way works! The installer will function perfectly.

---

**Questions? See ICON_AND_DISTRIBUTION.md for more details!**
