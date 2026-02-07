# Creating a GitHub Repository from Terminal

## Option 1: Using GitHub CLI (Easiest) ⭐

### Step 1: Install GitHub CLI (if not installed)

Download from: https://cli.github.com/

Or install via winget:
```powershell
winget install --id GitHub.cli
```

### Step 2: Login to GitHub

```powershell
gh auth login
```

Follow the prompts:
- Choose: GitHub.com
- Protocol: HTTPS
- Authenticate: Login with a web browser (easiest)
- Browser will open → Login → Authorize

### Step 3: Create Repository and Push

```powershell
# Navigate to your project
cd "C:\Users\Asus\Desktop\Medium posts\Widget_colorBlindness"

# Initialize git (if not done)
git init

# Add all files
git add .

# Commit
git commit -m "Initial commit: Color Blindness Accessibility Checker"

# Create GitHub repo and push (all in one!)
gh repo create colorblind-widget --public --source=. --remote=origin --push

# Or for private repo:
# gh repo create colorblind-widget --private --source=. --remote=origin --push
```

**Done! Your repo is created and code is pushed!** 🎉

---

## Option 2: Manual Method (Web + Terminal)

### Step 1: Create Repository on GitHub Website

1. Go to https://github.com/new
2. Repository name: `colorblind-widget`
3. Description: `Desktop widget for checking color blindness accessibility`
4. Choose **Public** or **Private**
5. **DO NOT** check any boxes (no README, no .gitignore, no license)
6. Click **Create repository**

### Step 2: Push from Terminal

```powershell
# Navigate to your project
cd "C:\Users\Asus\Desktop\Medium posts\Widget_colorBlindness"

# Initialize git (if not done)
git init

# Add all files
git add .

# Commit
git commit -m "Initial commit: Color Blindness Accessibility Checker"

# Add remote (replace YOUR-USERNAME with your GitHub username)
git remote add origin https://github.com/YOUR-USERNAME/colorblind-widget.git

# Rename branch to main (if needed)
git branch -M main

# Push to GitHub
git push -u origin main
```

---

## Option 3: Using Git with Personal Access Token

If you get authentication errors with HTTPS:

### Step 1: Create Personal Access Token

1. Go to: https://github.com/settings/tokens
2. Click "Generate new token" → "Generate new token (classic)"
3. Note: `colorblind-widget access`
4. Expiration: Choose your preference
5. Scopes: Check **repo** (all boxes under repo)
6. Click "Generate token"
7. **COPY THE TOKEN** (you won't see it again!)

### Step 2: Push Using Token

```powershell
# When prompted for password, use the token instead
git push -u origin main

# Username: YOUR-USERNAME
# Password: [paste your token here]
```

Or set up credential storage:
```powershell
# Store credentials
git config --global credential.helper wincred

# Then push (will ask once and remember)
git push -u origin main
```

---

## Complete Workflow (Copy-Paste Ready)

### Using GitHub CLI (Recommended):

```powershell
# 1. Check if GitHub CLI is installed
gh --version

# If not installed:
winget install --id GitHub.cli

# 2. Login
gh auth login

# 3. Navigate to project
cd "C:\Users\Asus\Desktop\Medium posts\Widget_colorBlindness"

# 4. Check git status
git status

# If "not a git repository", initialize:
git init

# 5. Configure git (first time only)
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# 6. Add and commit files
git add .
git commit -m "Initial commit: Color Blindness Accessibility Checker"

# 7. Create repo and push
gh repo create colorblind-widget --public --source=. --remote=origin --push

# 8. Done! Open your repo:
gh repo view --web
```

### Using Manual Method:

```powershell
# 1. Navigate to project
cd "C:\Users\Asus\Desktop\Medium posts\Widget_colorBlindness"

# 2. Initialize git
git init

# 3. Configure git (first time only)
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# 4. Add all files
git add .

# 5. Check what will be committed
git status

# 6. Commit
git commit -m "Initial commit: Color Blindness Accessibility Checker"

# 7. Create repo on GitHub website: https://github.com/new
#    Name: colorblind-widget
#    Public/Private: Your choice
#    Don't check any boxes
#    Click "Create repository"

# 8. Add remote (replace YOUR-USERNAME)
git remote add origin https://github.com/YOUR-USERNAME/colorblind-widget.git

# 9. Rename branch to main
git branch -M main

# 10. Push to GitHub
git push -u origin main

# If authentication fails, use Personal Access Token as password
```

---

## Checking Your Repository

After pushing:

```powershell
# View repository in browser
gh repo view --web

# Or manually visit:
# https://github.com/YOUR-USERNAME/colorblind-widget
```

---

## Common Issues & Solutions

### Issue: "failed to push some refs"

**Solution:**
```powershell
git pull origin main --rebase
git push -u origin main
```

### Issue: "remote origin already exists"

**Solution:**
```powershell
# Remove old remote
git remote remove origin

# Add correct remote
git remote add origin https://github.com/YOUR-USERNAME/colorblind-widget.git
```

### Issue: "Support for password authentication was removed"

**Solution:** Use Personal Access Token (see Option 3 above) or GitHub CLI

### Issue: "Permission denied (publickey)"

**Solution:** Use HTTPS instead of SSH, or set up SSH keys

```powershell
# Check current remote
git remote -v

# If it shows git@github.com, change to HTTPS:
git remote set-url origin https://github.com/YOUR-USERNAME/colorblind-widget.git
```

---

## Verify Everything Worked

```powershell
# Check remote is set
git remote -v

# Check branch
git branch

# Check commit history
git log --oneline

# Check repository URL
gh repo view --web
# Or visit: https://github.com/YOUR-USERNAME/colorblind-widget
```

You should see:
- ✅ All your files on GitHub
- ✅ README.md displayed on homepage
- ✅ Green commit history
- ✅ License badge (if public)

---

## After Repository is Created

### Clone to another machine:
```powershell
git clone https://github.com/YOUR-USERNAME/colorblind-widget.git
cd colorblind-widget
cargo build --release
```

### Make future updates:
```powershell
# Make changes to code
# ...

# Add changes
git add .

# Commit
git commit -m "Add new feature: XYZ"

# Push
git push
```

---

## Quick Reference

| Command | Purpose |
|---------|---------|
| `gh auth login` | Login to GitHub CLI |
| `gh repo create` | Create repo and push |
| `git init` | Initialize git |
| `git add .` | Stage all files |
| `git commit -m "msg"` | Commit changes |
| `git push` | Upload to GitHub |
| `git status` | Check current state |
| `git log` | View commit history |
| `gh repo view --web` | Open repo in browser |

---

## Next Steps After Creating Repo

1. **Add topics/tags:**
   - Go to GitHub repo → About section → Add tags
   - Suggested: `rust`, `slint`, `accessibility`, `color-blindness`, `desktop-app`

2. **Enable Issues:**
   - Settings → Features → Check "Issues"

3. **Create first release:**
   ```powershell
   .\create-release.ps1
   # Then upload ZIP to GitHub Releases
   ```

4. **Add a nice README badge:**
   ```markdown
   ![GitHub release](https://img.shields.io/github/v/release/YOUR-USERNAME/colorblind-widget)
   ```

---

**Recommended: Use GitHub CLI (gh) - it's the easiest!** ⭐

Need help? Run `gh --help` or visit https://cli.github.com/manual/
