# GitHub Setup Instructions

## Step 1: Initialize Git Repository

```powershell
# Navigate to your project directory
cd "c:\Users\Asus\Desktop\Medium posts\Widget_colorBlindness"

# Initialize git (if not already done)
git init

# Add all files
git add .

# Commit
git commit -m "Initial commit: Color Blindness Widget with Rust + Slint"
```

## Step 2: Create GitHub Repository

1. Go to [GitHub](https://github.com/new)
2. Create a new repository named: `colorblind-widget`
3. **Do NOT** initialize with README, .gitignore, or license (we already have these)
4. Click "Create repository"

## Step 3: Push to GitHub

```powershell
# Add the remote repository (replace YOUR-USERNAME with your GitHub username)
git remote add origin https://github.com/YOUR-USERNAME/colorblind-widget.git

# Push to GitHub
git branch -M main
git push -u origin main
```

## Alternative: Using GitHub Desktop

1. Download [GitHub Desktop](https://desktop.github.com/)
2. File → Add Local Repository → Select your widget folder
3. Click "Publish repository" button
4. Choose repository name and visibility (public/private)
5. Click "Publish Repository"

---

# Desktop Shortcut & Taskbar Setup

## Option 1: Quick Launch Script (Recommended)

1. **Double-click `launch-widget.bat`** to run the application
2. To create a desktop shortcut:
   - Right-click `launch-widget.bat`
   - Select "Send to" → "Desktop (create shortcut)"
3. To pin to taskbar:
   - Right-click the desktop shortcut
   - Select "Pin to taskbar"

## Option 2: Direct Executable Shortcut

1. Navigate to: `target\release\`
2. Find `colorblind-widget.exe`
3. Right-click → "Send to" → "Desktop (create shortcut)"
4. Right-click the shortcut → "Pin to taskbar"

### Customize the Icon (Optional)

1. Right-click the shortcut → Properties
2. Click "Change Icon"
3. Browse to select a custom .ico file
4. Click OK

---

# Docker Setup Instructions

⚠️ **Important Note**: GUI applications in Docker require special setup and are complex on Windows. Docker is primarily designed for server applications. For desktop widgets, native installation is recommended.

However, if you still want to explore Docker:

## Prerequisites

- Install [Docker Desktop for Windows](https://www.docker.com/products/docker-desktop/)
- Install VcXsrv or Xming (X11 server for Windows)

## Building Docker Image

```powershell
docker build -t colorblind-widget .
```

## Running with X11 Forwarding (Advanced)

1. Start VcXsrv with "Disable access control" checked
2. Run:
```powershell
docker run -e DISPLAY=host.docker.internal:0 colorblind-widget
```

## Why Docker is Not Ideal for This Application

- **GUI Complexity**: Docker containers don't natively support GUI applications
- **Performance**: X11 forwarding adds overhead
- **File Access**: Harder to access local images
- **Screenshot Capture**: May not work in containers
- **Native Experience**: Desktop shortcuts work better with native apps

**Recommendation**: Use the native executable with desktop shortcuts for the best user experience!

---

# Recommended Workflow

1. ✅ Build the release version: `cargo build --release`
2. ✅ Test the application: `cargo run --release`
3. ✅ Create desktop shortcut using `launch-widget.bat`
4. ✅ Pin to taskbar for quick access
5. ✅ Push to GitHub for version control
6. ❌ Skip Docker (not suitable for GUI desktop widgets)

---

# Troubleshooting

## Application won't start
- Make sure you've built the release version
- Check that all dependencies are installed

## Shortcut doesn't work
- Verify the path in the shortcut properties
- Try using the `launch-widget.bat` script instead

## Git issues
- Make sure Git is installed: `git --version`
- Configure Git if needed:
  ```powershell
  git config --global user.name "Your Name"
  git config --global user.email "your.email@example.com"
  ```
