# Instructions for Creating Desktop Shortcut and Taskbar Icon

## Quick Start (Easiest Method)

### Step 1: Build the Application
```powershell
cargo build --release
```

### Step 2: Create Desktop Shortcut
1. Double-click `launch-widget.bat` to test it works
2. Right-click `launch-widget.bat`
3. Select **"Send to" → "Desktop (create shortcut)"**
4. The shortcut appears on your desktop!

### Step 3: Pin to Taskbar
1. Right-click the desktop shortcut you just created
2. Select **"Pin to taskbar"**
3. Now you can launch it from your taskbar anytime!

---

## Alternative Method: Direct Executable Shortcut

### Method A: Using File Explorer

1. Open File Explorer
2. Navigate to: `C:\Users\Asus\Desktop\Medium posts\Widget_colorBlindness\target\release\`
3. Find `colorblind-widget.exe`
4. **Right-click** on it
5. Select **"Create shortcut"**
6. Windows will say "Cannot create shortcut here, create on desktop instead?"
7. Click **"Yes"**
8. Go to your desktop and find the shortcut
9. To pin to taskbar:
   - Right-click the shortcut
   - Click **"Pin to taskbar"**

### Method B: Manual Shortcut Creation

1. **Right-click** on your desktop → **New** → **Shortcut**
2. For location, enter:
   ```
   "C:\Users\Asus\Desktop\Medium posts\Widget_colorBlindness\target\release\colorblind-widget.exe"
   ```
3. Click **Next**
4. Name it: `Color Blindness Checker`
5. Click **Finish**
6. Right-click the new shortcut → **Pin to taskbar**

---

## Customizing Your Shortcut

### Change the Icon

1. Right-click the shortcut → **Properties**
2. Click **"Change Icon..."**
3. Either:
   - Browse to a custom `.ico` file you've downloaded
   - Select from Windows default icons
4. Click **OK** → **Apply** → **OK**

### Change the Name

1. Right-click the shortcut
2. Select **"Rename"**
3. Type your preferred name: `Color Blindness Widget`
4. Press Enter

### Add a Keyboard Shortcut

1. Right-click the shortcut → **Properties**
2. Click in the **"Shortcut key"** field
3. Press your desired key combination (e.g., `Ctrl + Alt + C`)
4. Click **OK**
5. Now you can launch it with that key combo!

---

## Advanced: Start with Windows

### Make it Auto-Start on Windows Boot

1. Press `Win + R` to open Run dialog
2. Type: `shell:startup` and press Enter
3. This opens the Startup folder
4. Copy your shortcut to this folder
5. The widget will now start automatically when Windows boots!

To stop auto-start: Just delete the shortcut from the Startup folder

---

## Troubleshooting

### Shortcut shows wrong icon
- The executable doesn't have an embedded icon by default
- Use "Change Icon" to select a custom one

### Shortcut doesn't work
- Verify the path is correct in Properties
- Make sure you've built the release version first
- Try using `launch-widget.bat` instead

### Can't pin to taskbar
- Pin the shortcut, not the raw .exe
- Try running the app first, then right-clicking the taskbar icon → "Pin to taskbar"

### App won't start from shortcut
- Check the "Start in" directory in shortcut properties
- Should be: `C:\Users\Asus\Desktop\Medium posts\Widget_colorBlindness`

---

## Testing Your Setup

1. ✅ Click the desktop shortcut → App should open
2. ✅ Click the taskbar icon → App should open
3. ✅ Try the keyboard shortcut (if configured)
4. ✅ Close the app and reopen from different launchers

All working? Perfect! You're all set! 🎉
